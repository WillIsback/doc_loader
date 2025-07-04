use crate::core::{
    UniversalOutput, DocumentType, ProcessingParams, DocumentMetadata, 
    DocumentChunk, ChunkPosition, ChunkMetadata, ProcessingInfo
};
use crate::error::DocLoaderError;
use crate::processors::DocumentProcessor;
use crate::utils::{chunk_text, clean_text, extract_text_metadata};

use std::path::Path;
use std::fs;
use serde_json::{Value, Map};
use chrono::Utc;
use serde_json::json;

pub struct JsonProcessor;

impl JsonProcessor {
    pub fn new() -> Self {
        Self
    }
    
    /// Lit et parse un fichier JSON
    fn read_json_file(&self, file_path: &Path) -> Result<Value, DocLoaderError> {
        let content = fs::read_to_string(file_path)?;
        let json_value: Value = serde_json::from_str(&content)?;
        Ok(json_value)
    }
    
    /// Convertit une valeur JSON en texte lisible pour l'extraction
    fn json_to_text(&self, value: &Value, path: &str, depth: usize) -> String {
        if depth > 10 { // Limite de profondeur pour éviter les récursions infinies
            return format!("{}=[MAX_DEPTH_REACHED]", path);
        }
        
        match value {
            Value::String(s) => format!("{}=\"{}\"", path, s),
            Value::Number(n) => format!("{}={}", path, n),
            Value::Bool(b) => format!("{}={}", path, b),
            Value::Null => format!("{}=null", path),
            Value::Array(arr) => {
                let mut text = format!("{}=[", path);
                for (i, item) in arr.iter().enumerate() {
                    if i > 0 { text.push_str(", "); }
                    text.push_str(&self.json_to_text(item, &format!("{}[{}]", path, i), depth + 1));
                }
                text.push(']');
                text
            },
            Value::Object(obj) => {
                let mut text = format!("{}={{", path);
                for (i, (key, val)) in obj.iter().enumerate() {
                    if i > 0 { text.push_str(", "); }
                    let new_path = if path.is_empty() { 
                        key.clone() 
                    } else { 
                        format!("{}.{}", path, key) 
                    };
                    text.push_str(&self.json_to_text(val, &new_path, depth + 1));
                }
                text.push('}');
                text
            }
        }
    }
    
    /// Extrait les métadonnées du JSON
    fn extract_json_metadata(&self, json_value: &Value) -> serde_json::Value {
        let mut metadata = Map::new();
        
        // Compter les éléments selon le type
        match json_value {
            Value::Object(obj) => {
                metadata.insert("type".to_string(), json!("object"));
                metadata.insert("keys_count".to_string(), json!(obj.len()));
                metadata.insert("keys".to_string(), json!(obj.keys().collect::<Vec<_>>()));
            },
            Value::Array(arr) => {
                metadata.insert("type".to_string(), json!("array"));
                metadata.insert("length".to_string(), json!(arr.len()));
                
                // Analyser les types des éléments
                let mut type_counts = Map::new();
                for item in arr {
                    let item_type = match item {
                        Value::String(_) => "string",
                        Value::Number(_) => "number",
                        Value::Bool(_) => "boolean",
                        Value::Null => "null",
                        Value::Array(_) => "array",
                        Value::Object(_) => "object",
                    };
                    let count = type_counts.get(item_type).and_then(|v| v.as_u64()).unwrap_or(0);
                    type_counts.insert(item_type.to_string(), json!(count + 1));
                }
                metadata.insert("element_types".to_string(), json!(type_counts));
            },
            Value::String(_) => { metadata.insert("type".to_string(), json!("string")); },
            Value::Number(_) => { metadata.insert("type".to_string(), json!("number")); },
            Value::Bool(_) => { metadata.insert("type".to_string(), json!("boolean")); },
            Value::Null => { metadata.insert("type".to_string(), json!("null")); },
        };
        
        // Calculer la profondeur
        metadata.insert("max_depth".to_string(), json!(self.calculate_depth(json_value)));
        
        Value::Object(metadata)
    }
    
    /// Calcule la profondeur maximale du JSON
    fn calculate_depth(&self, value: &Value) -> usize {
        match value {
            Value::Object(obj) => {
                1 + obj.values().map(|v| self.calculate_depth(v)).max().unwrap_or(0)
            },
            Value::Array(arr) => {
                1 + arr.iter().map(|v| self.calculate_depth(v)).max().unwrap_or(0)
            },
            _ => 0,
        }
    }
}

impl DocumentProcessor for JsonProcessor {
    fn supported_type(&self) -> DocumentType {
        DocumentType::JSON
    }
    
    fn process_file(&self, file_path: &Path, params: &ProcessingParams) -> Result<UniversalOutput, DocLoaderError> {
        let start_time = std::time::Instant::now();
        
        // Vérifier que le fichier existe
        if !file_path.exists() {
            return Err(DocLoaderError::FileNotFound(
                format!("File not found: {}", file_path.display())
            ));
        }
        
        // Lire et parser le JSON
        let json_value = self.read_json_file(file_path)?;
        
        // Convertir en texte pour l'extraction
        let raw_text = self.json_to_text(&json_value, "", 0);
        
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
            
            let start_offset = if index == 0 { 
                0 
            } else { 
                text.find(chunk_text).unwrap_or(0)
            };
            
            let chunk = DocumentChunk {
                id: format!("json_chunk_{}", index),
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
                    confidence: Some(1.0), // Confiance maximale pour l'extraction JSON
                    format_specific: Some(json!({
                        "word_count": text_meta.word_count,
                        "estimated_tokens": text_meta.estimated_tokens,
                        "json_type": match &json_value {
                            Value::Object(_) => "object",
                            Value::Array(_) => "array",
                            _ => "primitive"
                        }
                    })),
                },
            };
            
            chunks.push(chunk);
        }
        
        // Métadonnées du fichier
        let file_metadata = fs::metadata(file_path)?;
        let json_metadata = self.extract_json_metadata(&json_value);
        let text_meta = extract_text_metadata(&text);
        
        let document_metadata = DocumentMetadata {
            filename: file_path.file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("unknown.json")
                .to_string(),
            filepath: file_path.to_string_lossy().to_string(),
            document_type: DocumentType::JSON,
            file_size: file_metadata.len(),
            created_at: file_metadata.created().ok().map(|t| t.into()),
            modified_at: file_metadata.modified().ok().map(|t| t.into()),
            title: None, // Les fichiers JSON n'ont généralement pas de titre
            author: None, // Les fichiers JSON n'ont généralement pas d'auteur
            format_metadata: json!({
                "json_metadata": json_metadata,
                "total_words": text_meta.word_count,
                "total_characters": text_meta.character_count,
                "estimated_tokens": text_meta.estimated_tokens,
                "detected_language": text_meta.detected_language
            }),
        };
        
        let processing_time = start_time.elapsed();
        let processing_info = ProcessingInfo {
            processor: "JsonProcessor".to_string(),
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
        
        // Parser le JSON depuis les bytes
        let json_value: Value = serde_json::from_slice(content)?;
        
        // Convertir en texte pour l'extraction
        let raw_text = self.json_to_text(&json_value, "", 0);
        
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
            
            let start_offset = if index == 0 { 
                0 
            } else { 
                text.find(chunk_text).unwrap_or(0)
            };
            
            let chunk = DocumentChunk {
                id: format!("json_chunk_{}", index),
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
                        "json_type": match &json_value {
                            Value::Object(_) => "object",
                            Value::Array(_) => "array",
                            _ => "primitive"
                        }
                    })),
                },
            };
            
            chunks.push(chunk);
        }
        
        let json_metadata = self.extract_json_metadata(&json_value);
        let text_meta = extract_text_metadata(&text);
        
        let document_metadata = DocumentMetadata {
            filename: filename.to_string(),
            filepath: format!("memory://{}", filename),
            document_type: DocumentType::JSON,
            file_size: content.len() as u64,
            created_at: Some(Utc::now()),
            modified_at: Some(Utc::now()),
            title: None,
            author: None,
            format_metadata: json!({
                "json_metadata": json_metadata,
                "total_words": text_meta.word_count,
                "total_characters": text_meta.character_count,
                "estimated_tokens": text_meta.estimated_tokens,
                "detected_language": text_meta.detected_language
            }),
        };
        
        let processing_time = start_time.elapsed();
        let processing_info = ProcessingInfo {
            processor: "JsonProcessor".to_string(),
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

impl Default for JsonProcessor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    

    #[test]
    fn test_json_processor_creation() {
        let processor = JsonProcessor::new();
        assert_eq!(processor.supported_type().to_string(), "JSON");
    }

    #[test]
    fn test_json_to_text() {
        let processor = JsonProcessor::new();
        let json_value = json!({
            "name": "John",
            "age": 30,
            "city": "New York"
        });
        
        let text = processor.json_to_text(&json_value, "", 0);
        assert!(text.contains("name=\"John\""));
        assert!(text.contains("age=30"));
        assert!(text.contains("city=\"New York\""));
    }

    #[test]
    fn test_process_content() {
        let processor = JsonProcessor::new();
        let json_data = json!({
            "test": "value",
            "number": 42
        });
        let content = serde_json::to_vec(&json_data).unwrap();
        let params = ProcessingParams::default();
        
        let result = processor.process_content(&content, "test.json", &params);
        assert!(result.is_ok());
        
        let output = result.unwrap();
        assert_eq!(output.document_metadata.document_type.to_string(), "JSON");
        assert!(!output.chunks.is_empty());
    }
}
