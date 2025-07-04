# Doc Loader Documentation

Welcome to the comprehensive documentation for Doc Loader, a powerful toolkit for extracting and processing documentation from multiple file formats.

## 🚀 Overview

Doc Loader is a comprehensive Rust toolkit with Python bindings that allows you to:

- **Extract** text and metadata from PDF, TXT, JSON, CSV, and DOCX files
- **Convert** documents into a universal JSON format optimized for vector stores
- **Process** intelligent text chunking and cleaning
- **Detect** document language automatically
- **Optimize** output for RAG (Retrieval-Augmented Generation) systems

## 📦 Installation

### Python Package

```bash
pip install extracteur-docs-rs
```

### Rust Crate

```bash
cargo add doc_loader
```

## 🔧 Quick Start

### Python Usage

```python
import extracteur_docs_rs as doc_loader

# Process any supported file format
result = doc_loader.process_file("document.pdf")
print(f"Extracted {result.chunk_count()} chunks")

# Advanced usage with custom parameters
processor = doc_loader.PyUniversalProcessor()
params = doc_loader.PyProcessingParams(
    chunk_size=1000,
    overlap=100,
    clean_text=True,
    detect_language=True
)

result = processor.process_file("document.txt", params)
json_output = result.to_json()
```

### Rust Usage

```rust
use doc_loader::{UniversalProcessor, ProcessingParams};

let processor = UniversalProcessor::new();
let params = ProcessingParams::default()
    .with_chunk_size(1000)
    .with_language_detection(true);

let result = processor.process_file("document.pdf", Some(params))?;
println!("Extracted {} chunks", result.chunks.len());
```

## 📚 Documentation Sections

- [Python Usage Guide](python_usage.md) - Complete Python API documentation
- [API Reference](api.md) - Detailed API reference for all functions
- [Usage Examples](examples.md) - Practical examples and integration patterns

## 📊 Supported Formats

| Format | Features |
|--------|----------|
| **PDF** | Text extraction, metadata, page tracking |
| **TXT** | Encoding detection, language detection |
| **JSON** | Hierarchical analysis, schema inference |
| **CSV** | Header detection, column analysis |
| **DOCX** | Document structure, style preservation |

## 🔗 Links

- [GitHub Repository](https://github.com/WillIsback/doc_loader)
- [PyPI Package](https://pypi.org/project/extracteur-docs-rs/)
- [Crates.io Package](https://crates.io/crates/doc_loader)
- [Issue Tracker](https://github.com/WillIsback/doc_loader/issues)

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](https://github.com/WillIsback/doc_loader/blob/master/LICENSE) file for details.
