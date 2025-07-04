# 📚 API Documentation - Doc Loader

## Core Structures

### UniversalOutput
Structure JSON universelle pour tous les types de documents :

```rust
pub struct UniversalOutput {
    pub document_id: String,
    pub document_type: DocumentType,
    pub metadata: DocumentMetadata,
    pub chunks: Vec<DocumentChunk>,
    pub processing_time_ms: u64,
    pub created_at: String,
}
```

### DocumentChunk
Représente un segment de texte avec ses métadonnées :

```rust
pub struct DocumentChunk {
    pub id: String,
    pub content: String,
    pub word_count: usize,
    pub char_count: usize,
    pub position: ChunkPosition,
    pub metadata: ChunkMetadata,
}
```

## Processors

### UniversalProcessor
Processeur principal capable de traiter tous les formats :

```rust
impl UniversalProcessor {
    pub fn new() -> Self
    pub fn process_file(&self, file_path: &Path, params: &ProcessingParams) -> Result<UniversalOutput>
    pub fn get_supported_extensions() -> Vec<&'static str>
}
```

### Format-Specific Processors
- `PdfProcessor` - Traitement des fichiers PDF avec lopdf
- `TxtProcessor` - Traitement des fichiers texte brut
- `JsonProcessor` - Traitement et normalisation des fichiers JSON
- `CsvProcessor` - Traitement des fichiers CSV avec métadonnées colonnes
- `DocxProcessor` - Traitement des documents Word (placeholder)

## Utilities

### Text Processing
Fonctions centralisées pour le traitement de texte :

```rust
pub fn clean_text(text: &str) -> String
pub fn chunk_text(text: &str, chunk_size: usize, overlap: usize) -> Vec<String>
pub fn detect_language(text: &str) -> String
pub fn count_words(text: &str) -> usize
```

## Error Handling

### DocLoaderError
Enum couvrant tous les types d'erreurs :

```rust
pub enum DocLoaderError {
    IoError(std::io::Error),
    PdfError(String),
    JsonError(serde_json::Error),
    CsvError(csv::Error),
    ProcessingError(String),
    UnsupportedFormat(String),
}
```

## Usage Examples

### Basic Processing
```rust
use doc_loader::{UniversalProcessor, ProcessingParams};

let processor = UniversalProcessor::new();
let params = ProcessingParams::default();
let result = processor.process_file(Path::new("document.pdf"), &params)?;
```

### Custom Parameters
```rust
let params = ProcessingParams {
    chunk_size: 1000,
    overlap: 100,
    preserve_formatting: true,
    extract_metadata: true,
    clean_text: true,
};
```
