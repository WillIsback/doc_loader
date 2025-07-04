#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
use crate::{UniversalProcessor, ProcessingParams, UniversalOutput, DocumentChunk, DocumentMetadata};
#[cfg(feature = "python")]
use crate::processors::DocumentProcessor;
#[cfg(feature = "python")]
use std::path::Path;

#[cfg(feature = "python")]
#[pyclass]
pub struct PyUniversalProcessor {
    // We'll create the processor on-demand instead of storing it
}

#[cfg(feature = "python")]
#[pymethods]
impl PyUniversalProcessor {
    #[new]
    pub fn new() -> Self {
        Self {}
    }
    
    /// Process any supported file format and return structured data
    #[pyo3(signature = (file_path, params=None))]
    pub fn process_file(&self, file_path: &str, params: Option<PyProcessingParams>) -> PyResult<PyUniversalOutput> {
        let processor = UniversalProcessor::new();
        let params = params.unwrap_or_default().into();
        
        let result = processor.process_file(Path::new(file_path), Some(params));
        
        match result {
            Ok(output) => Ok(PyUniversalOutput::from(output)),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string())),
        }
    }
    
    /// Get list of supported file extensions
    #[staticmethod]
    pub fn get_supported_extensions() -> Vec<String> {
        UniversalProcessor::supported_extensions().iter().map(|s| s.to_string()).collect()
    }
    
    /// Process text content directly
    #[pyo3(signature = (content, params=None))]
    pub fn process_text_content(&self, content: &str, params: Option<PyProcessingParams>) -> PyResult<PyUniversalOutput> {
        let params = params.unwrap_or_default().into();
        
        // Create a temporary txt processor to handle raw content
        let txt_processor = crate::processors::txt::TxtProcessor::new();
        let result = txt_processor.process_content(content.as_bytes(), "text_content", &params);
        
        match result {
            Ok(output) => Ok(PyUniversalOutput::from(output)),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string())),
        }
    }
}

#[cfg(feature = "python")]
#[pyclass]
#[derive(Clone)]
pub struct PyProcessingParams {
    #[pyo3(get, set)]
    pub chunk_size: usize,
    #[pyo3(get, set)]
    pub overlap: usize,
    #[pyo3(get, set)]
    pub clean_text: bool,
    #[pyo3(get, set)]
    pub extract_metadata: bool,
    #[pyo3(get, set)]
    pub preserve_formatting: bool,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyProcessingParams {
    #[new]
    #[pyo3(signature = (chunk_size=800, overlap=100, clean_text=true, extract_metadata=true, preserve_formatting=false))]
    pub fn new(
        chunk_size: usize,
        overlap: usize,
        clean_text: bool,
        extract_metadata: bool,
        preserve_formatting: bool,
    ) -> Self {
        Self {
            chunk_size,
            overlap,
            clean_text,
            extract_metadata,
            preserve_formatting,
        }
    }
}

#[cfg(feature = "python")]
impl Default for PyProcessingParams {
    fn default() -> Self {
        Self::new(800, 100, true, true, false)
    }
}

#[cfg(feature = "python")]
impl From<PyProcessingParams> for ProcessingParams {
    fn from(py_params: PyProcessingParams) -> Self {
        Self {
            max_chunk_size: py_params.chunk_size,
            chunk_overlap: py_params.overlap,
            text_cleaning: py_params.clean_text,
            language_detection: false, // Not exposed in Python interface yet
            format_specific: if py_params.preserve_formatting {
                serde_json::json!({"preserve_formatting": true})
            } else {
                serde_json::Value::Null
            },
        }
    }
}

#[cfg(feature = "python")]
#[pyclass]
#[derive(Clone)]
pub struct PyDocumentMetadata {
    #[pyo3(get)]
    pub filename: String,
    #[pyo3(get)]
    pub filepath: String,
    #[pyo3(get)]
    pub document_type: String,
    #[pyo3(get)]
    pub file_size: u64,
    #[pyo3(get)]
    pub created_at: String,
    #[pyo3(get)]
    pub modified_at: String,
    #[pyo3(get)]
    pub title: Option<String>,
    #[pyo3(get)]
    pub author: Option<String>,
}

#[cfg(feature = "python")]
impl From<DocumentMetadata> for PyDocumentMetadata {
    fn from(metadata: DocumentMetadata) -> Self {
        Self {
            filename: metadata.filename,
            filepath: metadata.filepath,
            document_type: metadata.document_type.to_string().to_string(),
            file_size: metadata.file_size,
            created_at: metadata.created_at
                .map(|dt| dt.to_rfc3339())
                .unwrap_or_else(|| "Unknown".to_string()),
            modified_at: metadata.modified_at
                .map(|dt| dt.to_rfc3339())
                .unwrap_or_else(|| "Unknown".to_string()),
            title: metadata.title,
            author: metadata.author,
        }
    }
}

#[cfg(feature = "python")]
#[pyclass]
#[derive(Clone)]
pub struct PyDocumentChunk {
    #[pyo3(get)]
    pub id: String,
    #[pyo3(get)]
    pub content: String,
    #[pyo3(get)]
    pub chunk_index: usize,
    #[pyo3(get)]
    pub size: usize,
    #[pyo3(get)]
    pub word_count: usize,
    #[pyo3(get)]
    pub char_count: usize,
}

#[cfg(feature = "python")]
impl From<DocumentChunk> for PyDocumentChunk {
    fn from(chunk: DocumentChunk) -> Self {
        // Try to extract word count from format_specific metadata
        let word_count = chunk.metadata.format_specific
            .as_ref()
            .and_then(|v| v.get("word_count"))
            .and_then(|v| v.as_u64())
            .unwrap_or_else(|| {
                // Fallback: estimate word count from content
                chunk.content.split_whitespace().count() as u64
            }) as usize;

        Self {
            id: chunk.id,
            content: chunk.content,
            chunk_index: chunk.chunk_index,
            size: chunk.metadata.size,
            word_count,
            char_count: chunk.metadata.size,
        }
    }
}

#[cfg(feature = "python")]
#[pyclass]
#[derive(Clone)]
pub struct PyUniversalOutput {
    #[pyo3(get)]
    pub document_metadata: PyDocumentMetadata,
    #[pyo3(get)]
    pub chunks: Vec<PyDocumentChunk>,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyUniversalOutput {
    /// Get total word count across all chunks
    pub fn total_word_count(&self) -> usize {
        self.chunks.iter().map(|chunk| chunk.word_count).sum()
    }
    
    /// Get total character count across all chunks
    pub fn total_char_count(&self) -> usize {
        self.chunks.iter().map(|chunk| chunk.char_count).sum()
    }
    
    /// Get number of chunks
    pub fn chunk_count(&self) -> usize {
        self.chunks.len()
    }
    
    /// Get chunks as a list of text content
    pub fn get_text_chunks(&self) -> Vec<String> {
        self.chunks.iter().map(|chunk| chunk.content.clone()).collect()
    }
    
    /// Export to JSON string
    pub fn to_json(&self) -> PyResult<String> {
        let json_value = serde_json::json!({
            "document_metadata": {
                "filename": self.document_metadata.filename,
                "filepath": self.document_metadata.filepath,
                "document_type": self.document_metadata.document_type,
                "file_size": self.document_metadata.file_size,
                "created_at": self.document_metadata.created_at,
                "modified_at": self.document_metadata.modified_at,
                "title": self.document_metadata.title,
                "author": self.document_metadata.author
            },
            "chunks": self.chunks.iter().map(|chunk| serde_json::json!({
                "id": chunk.id,
                "content": chunk.content,
                "chunk_index": chunk.chunk_index,
                "size": chunk.size,
                "word_count": chunk.word_count,
                "char_count": chunk.char_count
            })).collect::<Vec<_>>(),
            "total_chunks": self.chunks.len(),
            "total_word_count": self.total_word_count(),
            "total_char_count": self.total_char_count()
        });
        
        serde_json::to_string_pretty(&json_value)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
    }
}

#[cfg(feature = "python")]
impl From<UniversalOutput> for PyUniversalOutput {
    fn from(output: UniversalOutput) -> Self {
        Self {
            document_metadata: PyDocumentMetadata::from(output.document_metadata),
            chunks: output.chunks.into_iter().map(PyDocumentChunk::from).collect(),
        }
    }
}

/// Python module definition - MUST match library name in Cargo.toml
#[cfg(feature = "python")]
#[pymodule]
#[pyo3(name = "doc_loader")]
fn python_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyUniversalProcessor>()?;
    m.add_class::<PyProcessingParams>()?;
    m.add_class::<PyDocumentMetadata>()?;
    m.add_class::<PyDocumentChunk>()?;
    m.add_class::<PyUniversalOutput>()?;
    
    // Add convenience functions
    #[pyfn(m)]
    #[pyo3(signature = (file_path, chunk_size=None))]
    fn process_file(file_path: &str, chunk_size: Option<usize>) -> PyResult<PyUniversalOutput> {
        let processor = PyUniversalProcessor::new();
        let mut params = PyProcessingParams::default();
        if let Some(size) = chunk_size {
            params.chunk_size = size;
        }
        processor.process_file(file_path, Some(params))
    }
    
    #[pyfn(m)]
    #[pyo3(signature = (content, chunk_size=None))]
    fn process_text(content: &str, chunk_size: Option<usize>) -> PyResult<PyUniversalOutput> {
        let processor = PyUniversalProcessor::new();
        let mut params = PyProcessingParams::default();
        if let Some(size) = chunk_size {
            params.chunk_size = size;
        }
        processor.process_text_content(content, Some(params))
    }
    
    #[pyfn(m)]
    fn supported_extensions() -> Vec<String> {
        crate::processors::UniversalProcessor::supported_extensions().iter().map(|s| s.to_string()).collect()
    }
    
    Ok(())
}
