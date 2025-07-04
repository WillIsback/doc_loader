use crate::error::{Result};
use serde::{Deserialize, Serialize};

/// Options for document splitting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SplitOptions {
    /// Maximum chunk size in characters
    pub max_chunk_size: usize,
    /// Overlap between chunks in characters
    pub chunk_overlap: usize,
    /// Split by sentences instead of arbitrary chunks
    pub split_by_sentences: bool,
    /// Minimum chunk size (chunks smaller than this will be merged)
    pub min_chunk_size: usize,
    /// Preserve paragraph boundaries
    pub preserve_paragraphs: bool,
}

impl Default for SplitOptions {
    fn default() -> Self {
        Self {
            max_chunk_size: 1000,
            chunk_overlap: 100,
            split_by_sentences: true,
            min_chunk_size: 50,
            preserve_paragraphs: true,
        }
    }
}

/// A text chunk with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextChunk {
    pub content: String,
    pub chunk_index: usize,
    pub start_char: usize,
    pub end_char: usize,
    pub word_count: usize,
    pub char_count: usize,
}

/// Document splitter for breaking text into manageable chunks
pub struct DocumentSplitter {}

impl DocumentSplitter {
    pub fn new() -> Self {
        Self {}
    }

    /// Split text into chunks based on options
    pub fn split_text(&self, text: &str, options: &SplitOptions) -> Result<Vec<TextChunk>> {
        if text.is_empty() {
            return Ok(vec![]);
        }

        let chunks = if options.split_by_sentences {
            self.split_by_sentences(text, options)?
        } else {
            self.split_by_size(text, options)?
        };

        Ok(chunks)
    }

    /// Split text by sentences while respecting chunk size limits
    fn split_by_sentences(&self, text: &str, options: &SplitOptions) -> Result<Vec<TextChunk>> {
        let sentences = self.extract_sentences(text);
        let mut chunks = Vec::new();
        let mut current_chunk = String::new();
        let mut current_start = 0;
        let mut chunk_index = 0;

        for sentence in sentences {
            let sentence_trimmed = sentence.trim();
            if sentence_trimmed.is_empty() {
                continue;
            }

            // Check if adding this sentence would exceed the chunk size
            let potential_size = current_chunk.len() + sentence_trimmed.len() + 1; // +1 for space
            
            if potential_size > options.max_chunk_size && !current_chunk.is_empty() {
                // Create chunk from current content
                if current_chunk.len() >= options.min_chunk_size {
                    let chunk = self.create_chunk(
                        &current_chunk,
                        chunk_index,
                        current_start,
                        current_start + current_chunk.len(),
                    );
                    chunks.push(chunk);
                    chunk_index += 1;
                }

                // Start new chunk with overlap
                current_chunk = self.apply_overlap(&current_chunk, options.chunk_overlap);
                current_start = if chunks.is_empty() { 0 } else {
                    chunks.last().unwrap().end_char - options.chunk_overlap
                };
            }

            // Add sentence to current chunk
            if !current_chunk.is_empty() {
                current_chunk.push(' ');
            }
            current_chunk.push_str(sentence_trimmed);
        }

        // Add final chunk if it has content
        if !current_chunk.is_empty() && current_chunk.len() >= options.min_chunk_size {
            let chunk = self.create_chunk(
                &current_chunk,
                chunk_index,
                current_start,
                current_start + current_chunk.len(),
            );
            chunks.push(chunk);
        }

        Ok(chunks)
    }

    /// Split text by fixed size chunks
    fn split_by_size(&self, text: &str, options: &SplitOptions) -> Result<Vec<TextChunk>> {
        let mut chunks = Vec::new();
        let mut start = 0;
        let mut chunk_index = 0;

        while start < text.len() {
            let end = std::cmp::min(start + options.max_chunk_size, text.len());
            let mut chunk_text = text[start..end].to_string();

            // Try to break at word boundary if not at end of text
            if end < text.len() {
                if let Some(last_space) = chunk_text.rfind(' ') {
                    chunk_text = chunk_text[..last_space].to_string();
                }
            }

            if chunk_text.len() >= options.min_chunk_size {
                let chunk = self.create_chunk(
                    &chunk_text,
                    chunk_index,
                    start,
                    start + chunk_text.len(),
                );
                chunks.push(chunk);
                chunk_index += 1;
            }

            // Move start position with overlap
            start = if start + chunk_text.len() >= text.len() {
                text.len()
            } else {
                start + chunk_text.len() - options.chunk_overlap
            };
        }

        Ok(chunks)
    }

    /// Extract sentences from text
    fn extract_sentences<'a>(&self, text: &'a str) -> Vec<&'a str> {
        // Simple sentence boundary detection
        // This could be enhanced with more sophisticated NLP
        let sentence_endings = regex::Regex::new(r"[.!?]+\s+").unwrap();
        let mut sentences = Vec::new();
        let mut last_end = 0;

        for mat in sentence_endings.find_iter(text) {
            let sentence = &text[last_end..mat.end()];
            if !sentence.trim().is_empty() {
                sentences.push(sentence.trim());
            }
            last_end = mat.end();
        }

        // Add remaining text as final sentence
        if last_end < text.len() {
            let remaining = &text[last_end..];
            if !remaining.trim().is_empty() {
                sentences.push(remaining.trim());
            }
        }

        sentences
    }

    /// Apply overlap to chunk content
    fn apply_overlap(&self, content: &str, overlap_size: usize) -> String {
        if content.len() <= overlap_size {
            return content.to_string();
        }

        let overlap_start = content.len() - overlap_size;
        content[overlap_start..].to_string()
    }

    /// Create a text chunk with metadata
    fn create_chunk(&self, content: &str, index: usize, start: usize, end: usize) -> TextChunk {
        let word_count = content.split_whitespace().count();
        let char_count = content.chars().count();

        TextChunk {
            content: content.to_string(),
            chunk_index: index,
            start_char: start,
            end_char: end,
            word_count,
            char_count,
        }
    }
}

impl Default for DocumentSplitter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_splitting() {
        let splitter = DocumentSplitter::new();
        let text = "This is the first sentence. This is the second sentence. This is the third sentence.";
        let options = SplitOptions {
            max_chunk_size: 50,
            chunk_overlap: 10,
            split_by_sentences: true,
            min_chunk_size: 10,
            preserve_paragraphs: true,
        };

        let chunks = splitter.split_text(text, &options).unwrap();
        assert!(!chunks.is_empty());
        
        for chunk in &chunks {
            assert!(chunk.content.len() <= options.max_chunk_size);
            assert!(chunk.word_count > 0);
        }
    }

    #[test]
    fn test_split_options_default() {
        let options = SplitOptions::default();
        assert_eq!(options.max_chunk_size, 1000);
        assert_eq!(options.chunk_overlap, 100);
        assert!(options.split_by_sentences);
    }
}
