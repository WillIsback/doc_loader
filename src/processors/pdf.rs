use crate::core::{
    UniversalOutput, DocumentType, ProcessingParams, DocumentMetadata, 
    DocumentChunk, ChunkPosition, ChunkMetadata, ProcessingInfo
};
use crate::error::DocLoaderError;
use crate::processors::DocumentProcessor;
use crate::utils::{chunk_text, clean_text, extract_text_metadata};

use lopdf::Document;
use std::path::Path;
use std::fs;
use chrono::Utc;
use serde_json::json;

pub struct PdfProcessor;

impl PdfProcessor {
    pub fn new() -> Self {
        Self
    }
    
    /// Extrait le texte d'un document PDF
    fn extract_text_from_pdf(&self, pdf_path: &Path) -> Result<String, DocLoaderError> {
        let doc = Document::load(pdf_path)
            .map_err(|e| DocLoaderError::PdfParsing(format!("Failed to load PDF: {}", e)))?;
        
        let mut text_content = String::new();
        
        // Parcourir toutes les pages
        let pages = doc.get_pages();
        for (page_num, _page_id) in pages.iter() {
            // Extraire le contenu textuel de la page
            // Note: lopdf ne fournit pas d'extraction de texte directe
            // Dans un vrai projet, vous utiliseriez une bibliothèque comme pdf-extract ou pdfium
            text_content.push_str(&format!("--- Page {} ---\n", page_num));
            text_content.push_str("Contenu de la page (extraction non implémentée avec lopdf)\n");
        }
        
        if text_content.is_empty() {
            return Err(DocLoaderError::TextExtraction("No text content found in PDF".to_string()));
        }
        
        Ok(text_content)
    }
    
    /// Extrait les métadonnées d'un PDF
    fn extract_pdf_metadata(&self, pdf_path: &Path) -> Result<serde_json::Value, DocLoaderError> {
        let doc = Document::load(pdf_path)
            .map_err(|e| DocLoaderError::PdfParsing(format!("Failed to load PDF: {}", e)))?;
        
        let mut metadata = serde_json::Map::new();
        
        // Informations de base
        metadata.insert("pages_count".to_string(), json!(doc.get_pages().len()));
        
        // Version PDF (lopdf ne fournit pas directement la version)
        metadata.insert("pdf_version".to_string(), json!("Unknown"));
        
        // Essayer d'extraire les métadonnées du document
        if let Ok(info) = doc.trailer.get(b"Info") {
            if let Ok(info_dict) = doc.get_dictionary(info.as_reference().unwrap()) {
                if let Ok(title) = info_dict.get(b"Title") {
                    if let Ok(title_str) = title.as_str() {
                        metadata.insert("title".to_string(), json!(title_str));
                    }
                }
                if let Ok(author) = info_dict.get(b"Author") {
                    if let Ok(author_str) = author.as_str() {
                        metadata.insert("author".to_string(), json!(author_str));
                    }
                }
                if let Ok(subject) = info_dict.get(b"Subject") {
                    if let Ok(subject_str) = subject.as_str() {
                        metadata.insert("subject".to_string(), json!(subject_str));
                    }
                }
                if let Ok(creator) = info_dict.get(b"Creator") {
                    if let Ok(creator_str) = creator.as_str() {
                        metadata.insert("creator".to_string(), json!(creator_str));
                    }
                }
            }
        }
        
        Ok(serde_json::Value::Object(metadata))
    }
}

impl DocumentProcessor for PdfProcessor {
    fn supported_type(&self) -> DocumentType {
        DocumentType::PDF
    }
    
    fn process_file(&self, file_path: &Path, params: &ProcessingParams) -> Result<UniversalOutput, DocLoaderError> {
        let start_time = std::time::Instant::now();
        
        // Vérifier que le fichier existe
        if !file_path.exists() {
            return Err(DocLoaderError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("File not found: {}", file_path.display())
            )));
        }
        
        // Extraire le texte
        let raw_text = self.extract_text_from_pdf(file_path)?;
        
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
            
            let chunk = DocumentChunk {
                id: format!("pdf_chunk_{}", index),
                content: chunk_text.clone(),
                chunk_index: index,
                position: ChunkPosition {
                    page: None, // TODO: implémenter la détection de page
                    line: None,
                    start_offset: None,
                    end_offset: None,
                },
                metadata: ChunkMetadata {
                    size: chunk_text.len(),
                    language: if params.language_detection { text_meta.detected_language } else { None },
                    confidence: Some(1.0), // Confiance maximale pour l'extraction PDF
                    format_specific: Some(json!({
                        "word_count": text_meta.word_count,
                        "estimated_tokens": text_meta.estimated_tokens
                    })),
                },
            };
            chunks.push(chunk);
        }
        
        // Métadonnées du fichier
        let file_metadata = fs::metadata(file_path)
            .map_err(|e| DocLoaderError::Io(e))?;
        
        // Métadonnées PDF spécifiques
        let pdf_metadata = self.extract_pdf_metadata(file_path)?;
        
        let document_metadata = DocumentMetadata {
            filename: file_path.file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("unknown.pdf")
                .to_string(),
            filepath: file_path.to_string_lossy().to_string(),
            document_type: DocumentType::PDF,
            file_size: file_metadata.len(),
            created_at: file_metadata.created().ok().map(|t| t.into()),
            modified_at: file_metadata.modified().ok().map(|t| t.into()),
            title: pdf_metadata.get("title").and_then(|v| v.as_str()).map(|s| s.to_string()),
            author: pdf_metadata.get("author").and_then(|v| v.as_str()).map(|s| s.to_string()),
            format_metadata: pdf_metadata,
        };
        
        let processing_time = start_time.elapsed();
        let processing_info = ProcessingInfo {
            processor: "PdfProcessor".to_string(),
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
    
    fn process_content(&self, _content: &[u8], _filename: &str, _params: &ProcessingParams) -> Result<UniversalOutput, DocLoaderError> {
        // Pour le contenu brut PDF, nous devrions utiliser une approche différente
        // Pour simplifier, retournons une erreur indiquant que cette fonctionnalité n'est pas implémentée
        Err(DocLoaderError::UnsupportedFormat(
            "Processing PDF from raw content is not yet implemented".to_string()
        ))
    }
}

impl Default for PdfProcessor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pdf_processor_creation() {
        let processor = PdfProcessor::new();
        assert_eq!(processor.supported_type().to_string(), "PDF");
    }
}
