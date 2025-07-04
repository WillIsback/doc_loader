use thiserror::Error;

/// Main error type for the doc_loader crate
#[derive(Error, Debug)]
pub enum DocLoaderError {
    #[error("Unsupported file format: {0}")]
    UnsupportedFormat(String),
    
    #[error("PDF parsing error: {0}")]
    PdfParsing(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("Text extraction error: {0}")]
    TextExtraction(String),
    
    #[error("Metadata extraction error: {0}")]
    MetadataExtraction(String),
    
    #[error("Document splitting error: {0}")]
    DocumentSplitting(String),
    
    #[error("Invalid file format: {0}")]
    InvalidFormat(String),
    
    #[error("File not found: {0}")]
    FileNotFound(String),
    
    #[error("Processing error: {0}")]
    Processing(String),
}

/// Result type alias for doc_loader operations
pub type Result<T> = std::result::Result<T, DocLoaderError>;

impl From<lopdf::Error> for DocLoaderError {
    fn from(err: lopdf::Error) -> Self {
        DocLoaderError::PdfParsing(err.to_string())
    }
}
