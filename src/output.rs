use crate::metadata::DocumentMetadata;
use crate::splitter::TextChunk;
use serde::{Deserialize, Serialize};

/// Output format options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OutputFormat {
    Json,
    JsonPretty,
    Yaml,
    Csv,
}

/// Document chunk for JSON output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentChunk {
    pub content: String,
    pub metadata: ChunkMetadata,
}

/// Metadata for a document chunk
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkMetadata {
    pub chunk_index: usize,
    pub start_char: usize,
    pub end_char: usize,
    pub word_count: usize,
    pub char_count: usize,
    pub source_file: String,
    pub chunk_hash: String,
}

/// Complete JSON output structure ready for vector stores
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonOutput {
    pub metadata: DocumentMetadata,
    pub chunks: Vec<DocumentChunk>,
    pub source_file: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub processing_timestamp: chrono::DateTime<chrono::Utc>,
}

impl JsonOutput {
    /// Create JSON output from text chunks and metadata
    pub fn from_chunks(
        chunks: Vec<TextChunk>,
        metadata: DocumentMetadata,
        source_file: String,
    ) -> Self {
        let document_chunks = chunks
            .into_iter()
            .map(|chunk| DocumentChunk::from_text_chunk(chunk, &source_file))
            .collect();

        Self {
            metadata,
            chunks: document_chunks,
            source_file,
            processing_timestamp: chrono::Utc::now(),
        }
    }

    /// Serialize to JSON string
    pub fn to_json(&self, pretty: bool) -> crate::Result<String> {
        if pretty {
            Ok(serde_json::to_string_pretty(self)?)
        } else {
            Ok(serde_json::to_string(self)?)
        }
    }

    /// Serialize to YAML string (requires yaml feature)
    #[cfg(feature = "yaml")]
    pub fn to_yaml(&self) -> crate::Result<String> {
        Ok(serde_yaml::to_string(self)?)
    }

    /// Get total word count across all chunks
    pub fn total_word_count(&self) -> usize {
        self.chunks.iter().map(|chunk| chunk.metadata.word_count).sum()
    }

    /// Get total character count across all chunks
    pub fn total_char_count(&self) -> usize {
        self.chunks.iter().map(|chunk| chunk.metadata.char_count).sum()
    }

    /// Get summary statistics
    pub fn get_summary(&self) -> OutputSummary {
        let chunk_count = self.chunks.len();
        let total_words = self.total_word_count();
        let total_chars = self.total_char_count();
        let avg_chunk_size = if chunk_count > 0 { total_chars / chunk_count } else { 0 };

        OutputSummary {
            source_file: self.source_file.clone(),
            chunk_count,
            total_words,
            total_chars,
            avg_chunk_size,
            page_count: self.metadata.page_count,
            processing_timestamp: self.processing_timestamp,
        }
    }
}

impl DocumentChunk {
    /// Create a DocumentChunk from a TextChunk
    pub fn from_text_chunk(text_chunk: TextChunk, source_file: &str) -> Self {
        let chunk_hash = Self::calculate_hash(&text_chunk.content);
        
        Self {
            content: text_chunk.content,
            metadata: ChunkMetadata {
                chunk_index: text_chunk.chunk_index,
                start_char: text_chunk.start_char,
                end_char: text_chunk.end_char,
                word_count: text_chunk.word_count,
                char_count: text_chunk.char_count,
                source_file: source_file.to_string(),
                chunk_hash,
            },
        }
    }

    /// Calculate hash for chunk content (for deduplication)
    fn calculate_hash(content: &str) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        content.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }
}

/// Summary statistics for processing output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputSummary {
    pub source_file: String,
    pub chunk_count: usize,
    pub total_words: usize,
    pub total_chars: usize,
    pub avg_chunk_size: usize,
    pub page_count: u32,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub processing_timestamp: chrono::DateTime<chrono::Utc>,
}

impl std::fmt::Display for OutputSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "File: {}\nChunks: {}\nTotal words: {}\nTotal chars: {}\nAvg chunk size: {} chars\nPages: {}\nProcessed: {}",
            self.source_file,
            self.chunk_count,
            self.total_words,
            self.total_chars,
            self.avg_chunk_size,
            self.page_count,
            self.processing_timestamp.format("%Y-%m-%d %H:%M:%S UTC")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::splitter::TextChunk;
    use std::collections::HashMap;

    #[test]
    fn test_document_chunk_creation() {
        let text_chunk = TextChunk {
            content: "This is a test chunk.".to_string(),
            chunk_index: 0,
            start_char: 0,
            end_char: 21,
            word_count: 5,
            char_count: 21,
        };

        let doc_chunk = DocumentChunk::from_text_chunk(text_chunk, "test.pdf");
        assert_eq!(doc_chunk.content, "This is a test chunk.");
        assert_eq!(doc_chunk.metadata.chunk_index, 0);
        assert_eq!(doc_chunk.metadata.source_file, "test.pdf");
        assert!(!doc_chunk.metadata.chunk_hash.is_empty());
    }

    #[test]
    fn test_json_output_summary() {
        let metadata = DocumentMetadata {
            title: Some("Test".to_string()),
            author: None,
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

        let chunks = vec![
            DocumentChunk {
                content: "First chunk".to_string(),
                metadata: ChunkMetadata {
                    chunk_index: 0,
                    start_char: 0,
                    end_char: 11,
                    word_count: 2,
                    char_count: 11,
                    source_file: "test.pdf".to_string(),
                    chunk_hash: "abc123".to_string(),
                },
            },
        ];

        let output = JsonOutput {
            metadata,
            chunks,
            source_file: "test.pdf".to_string(),
            processing_timestamp: chrono::Utc::now(),
        };

        let summary = output.get_summary();
        assert_eq!(summary.chunk_count, 1);
        assert_eq!(summary.total_words, 2);
        assert_eq!(summary.total_chars, 11);
        assert_eq!(summary.page_count, 5);
    }
}
