use crate::error::{DocLoaderError, Result};
use lopdf::Document;
use std::path::Path;
use tokio::fs;

/// Text extractor for PDF documents
pub struct TextExtractor {
    // Configuration options could be added here
}

impl TextExtractor {
    pub fn new() -> Self {
        Self {}
    }

    /// Extract text from a PDF file
    pub async fn extract_from_file(&self, path: &str) -> Result<String> {
        if !Path::new(path).exists() {
            return Err(DocLoaderError::FileNotFound(path.to_string()));
        }

        // Read file into memory
        let bytes = fs::read(path).await?;
        self.extract_from_bytes(&bytes)
    }

    /// Extract text from PDF bytes
    pub fn extract_from_bytes(&self, bytes: &[u8]) -> Result<String> {
        // Use lopdf for extraction from bytes
        self.extract_with_lopdf(bytes)
    }

    /// Extract text using lopdf (more control)
    fn extract_with_lopdf(&self, bytes: &[u8]) -> Result<String> {
        let doc = Document::load_mem(bytes)?;
        let mut text = String::new();

        // Get all pages
        let pages = doc.get_pages();
        
        for (page_num, _page_id) in pages {
            match doc.extract_text(&[page_num as u32]) {
                Ok(page_text) => {
                    text.push_str(&page_text);
                    text.push('\n');
                }
                Err(e) => {
                    log::warn!("Failed to extract text from page {}: {}", page_num, e);
                }
            }
        }

        if text.is_empty() {
            return Err(DocLoaderError::TextExtraction(
                "No text could be extracted from the PDF".to_string()
            ));
        }

        Ok(self.clean_text(text))
    }

    /// Clean and normalize extracted text
    fn clean_text(&self, text: String) -> String {
        // Remove excessive whitespace
        let text = regex::Regex::new(r"\s+").unwrap().replace_all(&text, " ");
        
        // Normalize line breaks
        let text = regex::Regex::new(r"\r\n|\r").unwrap().replace_all(&text, "\n");
        
        // Remove multiple consecutive newlines
        let text = regex::Regex::new(r"\n{3,}").unwrap().replace_all(&text, "\n\n");
        
        // Trim whitespace from each line
        text.lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl Default for TextExtractor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_cleaning() {
        let extractor = TextExtractor::new();
        let dirty_text = "  Hello   world  \n\n\n\n  Another line  \r\n  ";
        let clean = extractor.clean_text(dirty_text.to_string());
        assert_eq!(clean, "Hello world\n\nAnother line");
    }
}
