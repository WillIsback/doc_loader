use crate::core::{UniversalOutput, DocumentType, ProcessingParams};
use crate::error::DocLoaderError;
use std::path::Path;

pub mod pdf;
pub mod txt;
pub mod json;
pub mod csv;
pub mod docx;

/// Trait pour tous les processeurs de documents
pub trait DocumentProcessor {
    /// Type de document que ce processeur peut traiter
    fn supported_type(&self) -> DocumentType;
    
    /// Traite un fichier et retourne le résultat universel
    fn process_file(&self, file_path: &Path, params: &ProcessingParams) -> Result<UniversalOutput, DocLoaderError>;
    
    /// Traite du contenu brut et retourne le résultat universel
    fn process_content(&self, content: &[u8], filename: &str, params: &ProcessingParams) -> Result<UniversalOutput, DocLoaderError>;
    
    /// Retourne la version du processeur
    fn version(&self) -> &'static str {
        "1.0.0"
    }
}

/// Processeur universel qui délègue aux processeurs spécialisés
pub struct UniversalProcessor {
    pdf_processor: pdf::PdfProcessor,
    txt_processor: txt::TxtProcessor,
    json_processor: json::JsonProcessor,
    csv_processor: csv::CsvProcessor,
    docx_processor: docx::DocxProcessor,
}

impl UniversalProcessor {
    pub fn new() -> Self {
        Self {
            pdf_processor: pdf::PdfProcessor::new(),
            txt_processor: txt::TxtProcessor::new(),
            json_processor: json::JsonProcessor::new(),
            csv_processor: csv::CsvProcessor::new(),
            docx_processor: docx::DocxProcessor::new(),
        }
    }
    
    pub fn process_file(&self, file_path: &Path, params: Option<ProcessingParams>) -> Result<UniversalOutput, DocLoaderError> {
        let params = params.unwrap_or_default();
        
        let extension = file_path
            .extension()
            .and_then(|ext| ext.to_str())
            .ok_or_else(|| DocLoaderError::UnsupportedFormat("No file extension".to_string()))?;
        
        let doc_type = DocumentType::from_extension(extension)
            .ok_or_else(|| DocLoaderError::UnsupportedFormat(format!("Unsupported extension: {}", extension)))?;
        
        match doc_type {
            DocumentType::PDF => self.pdf_processor.process_file(file_path, &params),
            DocumentType::TXT => self.txt_processor.process_file(file_path, &params),
            DocumentType::JSON => self.json_processor.process_file(file_path, &params),
            DocumentType::CSV => self.csv_processor.process_file(file_path, &params),
            DocumentType::DOCX => self.docx_processor.process_file(file_path, &params),
        }
    }
    
    pub fn get_processor(&self, doc_type: &DocumentType) -> Box<&dyn DocumentProcessor> {
        match doc_type {
            DocumentType::PDF => Box::new(&self.pdf_processor),
            DocumentType::TXT => Box::new(&self.txt_processor),
            DocumentType::JSON => Box::new(&self.json_processor),
            DocumentType::CSV => Box::new(&self.csv_processor),
            DocumentType::DOCX => Box::new(&self.docx_processor),
        }
    }
    
    /// Get list of supported file extensions
    pub fn supported_extensions() -> &'static [&'static str] {
        &["pdf", "txt", "json", "csv", "docx"]
    }
}

impl Default for UniversalProcessor {
    fn default() -> Self {
        Self::new()
    }
}
