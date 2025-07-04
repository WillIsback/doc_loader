use crate::error::{DocLoaderError, Result};
use lopdf::Document;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use tokio::fs;

/// Document metadata information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentMetadata {
    pub title: Option<String>,
    pub author: Option<String>,
    pub subject: Option<String>,
    pub keywords: Option<String>,
    pub creator: Option<String>,
    pub producer: Option<String>,
    pub creation_date: Option<String>,
    pub modification_date: Option<String>,
    pub page_count: u32,
    pub file_size: u64,
    pub pdf_version: Option<String>,
    pub custom_properties: HashMap<String, String>,
}

/// Metadata extractor for PDF documents
pub struct MetadataExtractor {}

impl MetadataExtractor {
    pub fn new() -> Self {
        Self {}
    }

    /// Extract metadata from a PDF file
    pub async fn extract_from_file(&self, path: &str) -> Result<DocumentMetadata> {
        if !Path::new(path).exists() {
            return Err(DocLoaderError::FileNotFound(path.to_string()));
        }

        let file_size = fs::metadata(path).await?.len();
        let bytes = fs::read(path).await?;
        
        let mut metadata = self.extract_from_bytes(&bytes)?;
        metadata.file_size = file_size;
        
        Ok(metadata)
    }

    /// Extract metadata from PDF bytes
    pub fn extract_from_bytes(&self, bytes: &[u8]) -> Result<DocumentMetadata> {
        let doc = Document::load_mem(bytes)?;
        
        let mut metadata = DocumentMetadata {
            title: None,
            author: None,
            subject: None,
            keywords: None,
            creator: None,
            producer: None,
            creation_date: None,
            modification_date: None,
            page_count: 0,
            file_size: 0,
            pdf_version: Some(doc.version.clone()),
            custom_properties: HashMap::new(),
        };

        // Extract page count
        metadata.page_count = doc.get_pages().len() as u32;

        // Extract document info
        if let Ok(info_ref) = doc.trailer.get(b"Info") {
            if let Ok(info_object) = doc.get_object(info_ref.as_reference()?) {
                if let Ok(info_dict) = info_object.as_dict() {
                    // Standard metadata fields
                    metadata.title = self.extract_string_field(info_dict, b"Title");
                    metadata.author = self.extract_string_field(info_dict, b"Author");
                    metadata.subject = self.extract_string_field(info_dict, b"Subject");
                    metadata.keywords = self.extract_string_field(info_dict, b"Keywords");
                    metadata.creator = self.extract_string_field(info_dict, b"Creator");
                    metadata.producer = self.extract_string_field(info_dict, b"Producer");
                    metadata.creation_date = self.extract_string_field(info_dict, b"CreationDate");
                    metadata.modification_date = self.extract_string_field(info_dict, b"ModDate");

                    // Extract custom properties
                    for (key, value) in info_dict.iter() {
                        let key_str = String::from_utf8_lossy(key);
                        if !self.is_standard_field(key) {
                            if let Ok(value_bytes) = value.as_str() {
                                let value_str = String::from_utf8_lossy(value_bytes);
                                metadata.custom_properties.insert(key_str.to_string(), value_str.to_string());
                            }
                        }
                    }
                }
            }
        }

        Ok(metadata)
    }

    /// Extract string field from dictionary
    fn extract_string_field(&self, dict: &lopdf::Dictionary, key: &[u8]) -> Option<String> {
        dict.get(key)
            .ok()
            .and_then(|obj| obj.as_str().ok())
            .map(|bytes| String::from_utf8_lossy(bytes).to_string())
    }

    /// Check if field is a standard PDF metadata field
    fn is_standard_field(&self, key: &[u8]) -> bool {
        matches!(key, 
            b"Title" | b"Author" | b"Subject" | b"Keywords" | 
            b"Creator" | b"Producer" | b"CreationDate" | b"ModDate"
        )
    }
}

impl Default for MetadataExtractor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata_creation() {
        let metadata = DocumentMetadata {
            title: Some("Test Document".to_string()),
            author: Some("Test Author".to_string()),
            subject: None,
            keywords: None,
            creator: None,
            producer: None,
            creation_date: None,
            modification_date: None,
            page_count: 5,
            file_size: 1024,
            pdf_version: Some("1.4".to_string()),
            custom_properties: HashMap::new(),
        };

        assert_eq!(metadata.title, Some("Test Document".to_string()));
        assert_eq!(metadata.page_count, 5);
    }
}
