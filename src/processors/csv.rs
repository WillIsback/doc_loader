use crate::core::{
    UniversalOutput, DocumentType, ProcessingParams, DocumentMetadata, 
    DocumentChunk, ChunkPosition, ChunkMetadata, ProcessingInfo
};
use crate::error::DocLoaderError;
use crate::processors::DocumentProcessor;
use crate::utils::{chunk_text, clean_text, extract_text_metadata};

use std::path::Path;
use std::fs;
use csv::ReaderBuilder;
use chrono::Utc;
use serde_json::json;

pub struct CsvProcessor;

impl CsvProcessor {
    pub fn new() -> Self {
        Self
    }
    
    /// Lit et parse un fichier CSV
    fn read_csv_file(&self, file_path: &Path) -> Result<(Vec<Vec<String>>, Vec<String>), DocLoaderError> {
        let content = fs::read_to_string(file_path)?;
        self.parse_csv_content(&content)
    }
    
    /// Parse le contenu CSV
    fn parse_csv_content(&self, content: &str) -> Result<(Vec<Vec<String>>, Vec<String>), DocLoaderError> {
        let mut reader = ReaderBuilder::new()
            .has_headers(true)
            .from_reader(content.as_bytes());
        
        // Récupérer les en-têtes
        let headers = reader.headers()
            .map_err(|e| DocLoaderError::Processing(format!("Failed to read CSV headers: {}", e)))?
            .iter()
            .map(|h| h.to_string())
            .collect::<Vec<String>>();
        
        // Lire toutes les lignes
        let mut rows = Vec::new();
        for result in reader.records() {
            let record = result
                .map_err(|e| DocLoaderError::Processing(format!("Failed to read CSV record: {}", e)))?;
            
            let row = record.iter().map(|field| field.to_string()).collect();
            rows.push(row);
        }
        
        Ok((rows, headers))
    }
    
    /// Convertit les données CSV en texte lisible
    fn csv_to_text(&self, rows: &[Vec<String>], headers: &[String]) -> String {
        let mut text = String::new();
        
        // Ajouter les en-têtes
        text.push_str("Headers: ");
        text.push_str(&headers.join(", "));
        text.push_str("\n\n");
        
        // Ajouter chaque ligne avec ses labels
        for (row_index, row) in rows.iter().enumerate() {
            text.push_str(&format!("Row {}: ", row_index + 1));
            
            for (col_index, value) in row.iter().enumerate() {
                if col_index > 0 {
                    text.push_str(", ");
                }
                
                let header = headers.get(col_index).map(|h| h.as_str()).unwrap_or("unknown");
                text.push_str(&format!("{}=\"{}\"", header, value));
            }
            text.push('\n');
        }
        
        text
    }
    
    /// Extrait les métadonnées du CSV
    fn extract_csv_metadata(&self, rows: &[Vec<String>], headers: &[String]) -> serde_json::Value {
        let total_rows = rows.len();
        let total_columns = headers.len();
        
        // Analyser les types de données par colonne
        let mut column_stats = Vec::new();
        
        for (col_index, header) in headers.iter().enumerate() {
            let mut non_empty_count = 0;
            let mut numeric_count = 0;
            let mut max_length = 0;
            let mut unique_values = std::collections::HashSet::new();
            
            for row in rows {
                if let Some(value) = row.get(col_index) {
                    if !value.is_empty() {
                        non_empty_count += 1;
                        max_length = max_length.max(value.len());
                        unique_values.insert(value.clone());
                        
                        // Tenter de parser comme nombre
                        if value.parse::<f64>().is_ok() {
                            numeric_count += 1;
                        }
                    }
                }
            }
            
            let data_type = if numeric_count == non_empty_count && non_empty_count > 0 {
                "numeric"
            } else if unique_values.len() <= 10 && total_rows > 10 {
                "categorical"
            } else {
                "text"
            };
            
            column_stats.push(json!({
                "name": header,
                "data_type": data_type,
                "non_empty_count": non_empty_count,
                "fill_rate": if total_rows > 0 { non_empty_count as f64 / total_rows as f64 } else { 0.0 },
                "unique_values": unique_values.len(),
                "max_length": max_length
            }));
        }
        
        json!({
            "total_rows": total_rows,
            "total_columns": total_columns,
            "headers": headers,
            "column_statistics": column_stats,
            "data_completeness": if total_rows > 0 && total_columns > 0 {
                rows.iter().map(|row| {
                    row.iter().filter(|cell| !cell.is_empty()).count() as f64 / total_columns as f64
                }).sum::<f64>() / total_rows as f64
            } else { 0.0 }
        })
    }
}

impl DocumentProcessor for CsvProcessor {
    fn supported_type(&self) -> DocumentType {
        DocumentType::CSV
    }
    
    fn process_file(&self, file_path: &Path, params: &ProcessingParams) -> Result<UniversalOutput, DocLoaderError> {
        let start_time = std::time::Instant::now();
        
        // Vérifier que le fichier existe
        if !file_path.exists() {
            return Err(DocLoaderError::FileNotFound(
                format!("File not found: {}", file_path.display())
            ));
        }
        
        // Lire et parser le CSV
        let (rows, headers) = self.read_csv_file(file_path)?;
        
        // Convertir en texte pour l'extraction
        let raw_text = self.csv_to_text(&rows, &headers);
        
        // Nettoyer le texte si demandé
        let text = if params.text_cleaning {
            clean_text(&raw_text)
        } else {
            raw_text
        };
        
        // Découper en chunks
        let chunks_text = chunk_text(&text, params.max_chunk_size, params.chunk_overlap);
        
        // Créer les chunks avec métadonnées
        let mut chunks = Vec::new();
        
        for (index, chunk_text) in chunks_text.iter().enumerate() {
            let text_meta = extract_text_metadata(chunk_text);
            
            // Estimer quelle partie des données ce chunk représente
            let estimated_rows = chunk_text.lines().count().saturating_sub(1); // -1 pour l'en-tête
            
            let start_offset = if index == 0 { 
                0 
            } else { 
                text.find(chunk_text).unwrap_or(0)
            };
            
            let chunk = DocumentChunk {
                id: format!("csv_chunk_{}", index),
                content: chunk_text.clone(),
                chunk_index: index,
                position: ChunkPosition {
                    page: None,
                    line: None, // Les CSV sont structurés différemment
                    start_offset: Some(start_offset),
                    end_offset: Some(start_offset + chunk_text.len()),
                },
                metadata: ChunkMetadata {
                    size: chunk_text.len(),
                    language: if params.language_detection { text_meta.detected_language } else { None },
                    confidence: Some(1.0), // Confiance maximale pour l'extraction CSV
                    format_specific: Some(json!({
                        "word_count": text_meta.word_count,
                        "estimated_tokens": text_meta.estimated_tokens,
                        "estimated_rows": estimated_rows,
                        "contains_headers": index == 0 // Premier chunk contient probablement les en-têtes
                    })),
                },
            };
            
            chunks.push(chunk);
        }
        
        // Métadonnées du fichier
        let file_metadata = fs::metadata(file_path)?;
        let csv_metadata = self.extract_csv_metadata(&rows, &headers);
        let text_meta = extract_text_metadata(&text);
        
        let document_metadata = DocumentMetadata {
            filename: file_path.file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("unknown.csv")
                .to_string(),
            filepath: file_path.to_string_lossy().to_string(),
            document_type: DocumentType::CSV,
            file_size: file_metadata.len(),
            created_at: file_metadata.created().ok().map(|t| t.into()),
            modified_at: file_metadata.modified().ok().map(|t| t.into()),
            title: None, // Les fichiers CSV n'ont généralement pas de titre
            author: None, // Les fichiers CSV n'ont généralement pas d'auteur
            format_metadata: json!({
                "csv_metadata": csv_metadata,
                "total_words": text_meta.word_count,
                "total_characters": text_meta.character_count,
                "estimated_tokens": text_meta.estimated_tokens,
                "detected_language": text_meta.detected_language
            }),
        };
        
        let processing_time = start_time.elapsed();
        let processing_info = ProcessingInfo {
            processor: "CsvProcessor".to_string(),
            processor_version: self.version().to_string(),
            processed_at: Utc::now(),
            processing_time_ms: processing_time.as_millis() as u64,
            total_chunks: chunks.len(),
            total_content_size: text.len(),
            processing_params: params.clone(),
        };
        
        Ok(UniversalOutput {
            document_metadata,
            chunks,
            processing_info,
        })
    }
    
    fn process_content(&self, content: &[u8], filename: &str, params: &ProcessingParams) -> Result<UniversalOutput, DocLoaderError> {
        let start_time = std::time::Instant::now();
        
        // Convertir en string
        let content_str = String::from_utf8_lossy(content);
        
        // Parser le CSV depuis le contenu
        let (rows, headers) = self.parse_csv_content(&content_str)?;
        
        // Convertir en texte pour l'extraction
        let raw_text = self.csv_to_text(&rows, &headers);
        
        // Nettoyer le texte si demandé
        let text = if params.text_cleaning {
            clean_text(&raw_text)
        } else {
            raw_text
        };
        
        // Découper en chunks
        let chunks_text = chunk_text(&text, params.max_chunk_size, params.chunk_overlap);
        
        // Créer les chunks avec métadonnées
        let mut chunks = Vec::new();
        
        for (index, chunk_text) in chunks_text.iter().enumerate() {
            let text_meta = extract_text_metadata(chunk_text);
            let estimated_rows = chunk_text.lines().count().saturating_sub(1);
            
            let start_offset = if index == 0 { 
                0 
            } else { 
                text.find(chunk_text).unwrap_or(0)
            };
            
            let chunk = DocumentChunk {
                id: format!("csv_chunk_{}", index),
                content: chunk_text.clone(),
                chunk_index: index,
                position: ChunkPosition {
                    page: None,
                    line: None,
                    start_offset: Some(start_offset),
                    end_offset: Some(start_offset + chunk_text.len()),
                },
                metadata: ChunkMetadata {
                    size: chunk_text.len(),
                    language: if params.language_detection { text_meta.detected_language } else { None },
                    confidence: Some(1.0),
                    format_specific: Some(json!({
                        "word_count": text_meta.word_count,
                        "estimated_tokens": text_meta.estimated_tokens,
                        "estimated_rows": estimated_rows,
                        "contains_headers": index == 0
                    })),
                },
            };
            
            chunks.push(chunk);
        }
        
        let csv_metadata = self.extract_csv_metadata(&rows, &headers);
        let text_meta = extract_text_metadata(&text);
        
        let document_metadata = DocumentMetadata {
            filename: filename.to_string(),
            filepath: format!("memory://{}", filename),
            document_type: DocumentType::CSV,
            file_size: content.len() as u64,
            created_at: Some(Utc::now()),
            modified_at: Some(Utc::now()),
            title: None,
            author: None,
            format_metadata: json!({
                "csv_metadata": csv_metadata,
                "total_words": text_meta.word_count,
                "total_characters": text_meta.character_count,
                "estimated_tokens": text_meta.estimated_tokens,
                "detected_language": text_meta.detected_language
            }),
        };
        
        let processing_time = start_time.elapsed();
        let processing_info = ProcessingInfo {
            processor: "CsvProcessor".to_string(),
            processor_version: self.version().to_string(),
            processed_at: Utc::now(),
            processing_time_ms: processing_time.as_millis() as u64,
            total_chunks: chunks.len(),
            total_content_size: text.len(),
            processing_params: params.clone(),
        };
        
        Ok(UniversalOutput {
            document_metadata,
            chunks,
            processing_info,
        })
    }
}

impl Default for CsvProcessor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csv_processor_creation() {
        let processor = CsvProcessor::new();
        assert_eq!(processor.supported_type().to_string(), "CSV");
    }

    #[test]
    fn test_csv_to_text() {
        let processor = CsvProcessor::new();
        let headers = vec!["name".to_string(), "age".to_string(), "city".to_string()];
        let rows = vec![
            vec!["John".to_string(), "30".to_string(), "New York".to_string()],
            vec!["Jane".to_string(), "25".to_string(), "Boston".to_string()],
        ];
        
        let text = processor.csv_to_text(&rows, &headers);
        assert!(text.contains("Headers: name, age, city"));
        assert!(text.contains("name=\"John\""));
        assert!(text.contains("age=\"30\""));
    }

    #[test]
    fn test_process_content() {
        let processor = CsvProcessor::new();
        let csv_content = "name,age,city\nJohn,30,New York\nJane,25,Boston";
        let content = csv_content.as_bytes();
        let params = ProcessingParams::default();
        
        let result = processor.process_content(content, "test.csv", &params);
        assert!(result.is_ok());
        
        let output = result.unwrap();
        assert_eq!(output.document_metadata.document_type.to_string(), "CSV");
        assert!(!output.chunks.is_empty());
    }
}
