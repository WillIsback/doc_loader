use crate::core::{
    UniversalOutput, DocumentType, ProcessingParams, DocumentMetadata, 
    DocumentChunk, ChunkPosition, ChunkMetadata, ProcessingInfo
};
use crate::error::DocLoaderError;
use crate::processors::DocumentProcessor;
use crate::utils::{chunk_text, clean_text, extract_text_metadata, normalize_line_breaks};

use std::path::Path;
use std::fs;
use encoding_rs::UTF_8;
use chrono::Utc;
use serde_json::json;

pub struct TxtProcessor;

impl TxtProcessor {
    pub fn new() -> Self {
        Self
    }
    
    /// Lit et décode un fichier texte
    fn read_text_file(&self, file_path: &Path) -> Result<String, DocLoaderError> {
        let bytes = fs::read(file_path)?;
        
        // Détecter l'encodage et décoder
        let (content, _encoding, had_errors) = UTF_8.decode(&bytes);
        
        if had_errors {
            log::warn!("Some encoding errors detected while reading {}", file_path.display());
        }
        
        Ok(normalize_line_breaks(&content))
    }
}

impl DocumentProcessor for TxtProcessor {
    fn supported_type(&self) -> DocumentType {
        DocumentType::TXT
    }
    
    fn process_file(&self, file_path: &Path, params: &ProcessingParams) -> Result<UniversalOutput, DocLoaderError> {
        let start_time = std::time::Instant::now();
        
        // Vérifier que le fichier existe
        if !file_path.exists() {
            return Err(DocLoaderError::FileNotFound(
                format!("File not found: {}", file_path.display())
            ));
        }
        
        // Lire le contenu
        let raw_text = self.read_text_file(file_path)?;
        
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
        let mut current_line = 0;
        
        for (index, chunk_text) in chunks_text.iter().enumerate() {
            let text_meta = extract_text_metadata(chunk_text);
            
            // Calculer la position approximative dans le fichier
            let chunk_lines = chunk_text.lines().count();
            let start_offset = if index == 0 { 
                0 
            } else { 
                // Approximation basée sur la position dans le texte
                text.find(chunk_text).unwrap_or(0)
            };
            
            let chunk = DocumentChunk {
                id: format!("txt_chunk_{}", index),
                content: chunk_text.clone(),
                chunk_index: index,
                position: ChunkPosition {
                    page: None, // Les fichiers TXT n'ont pas de pages
                    line: Some(current_line as u32),
                    start_offset: Some(start_offset),
                    end_offset: Some(start_offset + chunk_text.len()),
                },
                metadata: ChunkMetadata {
                    size: chunk_text.len(),
                    language: if params.language_detection { text_meta.detected_language } else { None },
                    confidence: Some(1.0), // Confiance maximale pour l'extraction de texte brut
                    format_specific: Some(json!({
                        "word_count": text_meta.word_count,
                        "line_count": chunk_lines,
                        "estimated_tokens": text_meta.estimated_tokens
                    })),
                },
            };
            
            chunks.push(chunk);
            current_line += chunk_lines;
        }
        
        // Métadonnées du fichier
        let file_metadata = fs::metadata(file_path)?;
        let text_meta = extract_text_metadata(&text);
        
        let document_metadata = DocumentMetadata {
            filename: file_path.file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("unknown.txt")
                .to_string(),
            filepath: file_path.to_string_lossy().to_string(),
            document_type: DocumentType::TXT,
            file_size: file_metadata.len(),
            created_at: file_metadata.created().ok().map(|t| t.into()),
            modified_at: file_metadata.modified().ok().map(|t| t.into()),
            title: None, // Les fichiers TXT n'ont généralement pas de titre
            author: None, // Les fichiers TXT n'ont généralement pas d'auteur
            format_metadata: json!({
                "encoding": "UTF-8",
                "total_lines": text_meta.line_count,
                "total_words": text_meta.word_count,
                "total_characters": text_meta.character_count,
                "estimated_tokens": text_meta.estimated_tokens,
                "detected_language": text_meta.detected_language
            }),
        };
        
        let processing_time = start_time.elapsed();
        let processing_info = ProcessingInfo {
            processor: "TxtProcessor".to_string(),
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
        
        // Détecter l'encodage et décoder
        let (text_content, _encoding, had_errors) = UTF_8.decode(content);
        
        if had_errors {
            log::warn!("Some encoding errors detected while processing content for {}", filename);
        }
        
        let raw_text = normalize_line_breaks(&text_content);
        
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
        let mut current_line = 0;
        
        for (index, chunk_text) in chunks_text.iter().enumerate() {
            let text_meta = extract_text_metadata(chunk_text);
            let chunk_lines = chunk_text.lines().count();
            
            let start_offset = if index == 0 { 
                0 
            } else { 
                text.find(chunk_text).unwrap_or(0)
            };
            
            let chunk = DocumentChunk {
                id: format!("txt_chunk_{}", index),
                content: chunk_text.clone(),
                chunk_index: index,
                position: ChunkPosition {
                    page: None,
                    line: Some(current_line as u32),
                    start_offset: Some(start_offset),
                    end_offset: Some(start_offset + chunk_text.len()),
                },
                metadata: ChunkMetadata {
                    size: chunk_text.len(),
                    language: if params.language_detection { text_meta.detected_language } else { None },
                    confidence: Some(1.0),
                    format_specific: Some(json!({
                        "word_count": text_meta.word_count,
                        "line_count": chunk_lines,
                        "estimated_tokens": text_meta.estimated_tokens
                    })),
                },
            };
            
            chunks.push(chunk);
            current_line += chunk_lines;
        }
        
        let text_meta = extract_text_metadata(&text);
        
        let document_metadata = DocumentMetadata {
            filename: filename.to_string(),
            filepath: format!("memory://{}", filename),
            document_type: DocumentType::TXT,
            file_size: content.len() as u64,
            created_at: Some(Utc::now()),
            modified_at: Some(Utc::now()),
            title: None,
            author: None,
            format_metadata: json!({
                "encoding": "UTF-8",
                "total_lines": text_meta.line_count,
                "total_words": text_meta.word_count,
                "total_characters": text_meta.character_count,
                "estimated_tokens": text_meta.estimated_tokens,
                "detected_language": text_meta.detected_language
            }),
        };
        
        let processing_time = start_time.elapsed();
        let processing_info = ProcessingInfo {
            processor: "TxtProcessor".to_string(),
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

impl Default for TxtProcessor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::Write;

    #[test]
    fn test_txt_processor_creation() {
        let processor = TxtProcessor::new();
        assert_eq!(processor.supported_type().to_string(), "TXT");
    }

    #[test]
    fn test_process_content() {
        let processor = TxtProcessor::new();
        let content = b"Hello world\nThis is a test file.";
        let params = ProcessingParams::default();
        
        let result = processor.process_content(content, "test.txt", &params);
        assert!(result.is_ok());
        
        let output = result.unwrap();
        assert_eq!(output.document_metadata.document_type.to_string(), "TXT");
        assert!(!output.chunks.is_empty());
    }

    #[test]
    fn test_read_text_file() {
        let processor = TxtProcessor::new();
        
        // Créer un fichier temporaire
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "Hello world\nThis is a test.").unwrap();
        
        let result = processor.read_text_file(temp_file.path());
        assert!(result.is_ok());
        
        let content = result.unwrap();
        assert!(content.contains("Hello world"));
        assert!(content.contains("This is a test."));
    }
}
