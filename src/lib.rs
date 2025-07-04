//! # Doc Loader
//! 
//! A comprehensive toolkit for extracting and processing documentation from multiple file formats.
//! 
//! This library provides unified processing for different document types:
//! - PDF documents
//! - Plain text files  
//! - JSON documents
//! - CSV files
//! - DOCX documents
//! 
//! Each processor extracts content and metadata, then formats everything into a universal JSON
//! structure ready for vector stores and RAG systems.
//! 
//! ## Features
//! 
//! - **Universal JSON Output**: Consistent format across all document types
//! - **Intelligent Text Processing**: Smart chunking, cleaning, and metadata extraction
//! - **Modular Architecture**: Each document type has its specialized processor
//! - **Vector Store Ready**: Optimized output for embedding and indexing
//! 
//! ## Example
//! 
//! ```rust
//! use doc_loader::{UniversalProcessor, ProcessingParams};
//! 
//! // Create a processor instance
//! let processor = UniversalProcessor::new();
//! let params = ProcessingParams::default();
//! 
//! // Get supported extensions
//! let extensions = UniversalProcessor::supported_extensions();
//! assert!(!extensions.is_empty());
//! assert!(extensions.contains(&"pdf"));
//! 
//! // Example of processing (would require an actual file)
//! // let result = processor.process_file(Path::new("document.pdf"), Some(params))?;
//! // println!("Extracted {} chunks", result.chunks.len());
//! ```

pub mod error;
pub mod utils;
pub mod processors;
pub mod core;

pub use error::{DocLoaderError, Result};
pub use core::{
    UniversalOutput, DocumentChunk, ChunkMetadata, DocumentMetadata, 
    ProcessingParams, DocumentType, ProcessingInfo
};
pub use processors::{UniversalProcessor, DocumentProcessor};

// Re-export key utility functions
pub use utils::{
    clean_text, chunk_text, extract_text_metadata, detect_language
};

// Python bindings module
#[cfg(feature = "python")]
pub mod python;

// Re-export Python bindings when feature is enabled
#[cfg(feature = "python")]
pub use python::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_universal_processor_creation() {
        let _processor = UniversalProcessor::new();
        // Test that processor is created successfully
        assert!(!UniversalProcessor::supported_extensions().is_empty());
    }

    #[test]
    fn test_supported_extensions() {
        let extensions = UniversalProcessor::supported_extensions();
        assert!(extensions.contains(&"pdf"));
        assert!(extensions.contains(&"txt"));
        assert!(extensions.contains(&"json"));
        assert!(extensions.contains(&"csv"));
        assert!(extensions.contains(&"docx"));
    }

    #[test]
    fn test_processing_params_default() {
        let params = ProcessingParams::default();
        assert_eq!(params.max_chunk_size, 1000);
        assert_eq!(params.chunk_overlap, 100);
        assert!(params.text_cleaning);
        assert!(!params.language_detection);
    }

    #[test]
    fn test_document_type_from_extension() {
        assert_eq!(DocumentType::from_extension("pdf"), Some(DocumentType::PDF));
        assert_eq!(DocumentType::from_extension("txt"), Some(DocumentType::TXT));
        assert_eq!(DocumentType::from_extension("json"), Some(DocumentType::JSON));
        assert_eq!(DocumentType::from_extension("csv"), Some(DocumentType::CSV));
        assert_eq!(DocumentType::from_extension("docx"), Some(DocumentType::DOCX));
        assert_eq!(DocumentType::from_extension("unknown"), None);
    }

    #[test]
    fn test_document_type_to_string() {
        assert_eq!(DocumentType::PDF.to_string(), "PDF");
        assert_eq!(DocumentType::TXT.to_string(), "TXT");
        assert_eq!(DocumentType::JSON.to_string(), "JSON");
        assert_eq!(DocumentType::CSV.to_string(), "CSV");
        assert_eq!(DocumentType::DOCX.to_string(), "DOCX");
    }

    #[test]
    fn test_text_cleaning() {
        let dirty_text = "  Hello\t\tWorld  \n\n  Test  ";
        let cleaned = clean_text(dirty_text);
        assert_eq!(cleaned, "Hello World Test");
    }

    #[test]
    fn test_text_chunking() {
        let text = "This is a test document with multiple sentences. It should be split into chunks properly.";
        let chunks = chunk_text(text, 50, 10);
        assert!(chunks.len() > 1);
        assert!(chunks[0].len() <= 50);
    }

    #[test]
    fn test_error_handling() {
        let error = DocLoaderError::UnsupportedFormat("test".to_string());
        assert!(error.to_string().contains("Unsupported file format"));
        
        let error = DocLoaderError::FileNotFound("test.txt".to_string());
        assert!(error.to_string().contains("File not found"));
    }
}
