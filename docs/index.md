# Doc Loader Documentation

Welcome to the documentation for Doc Loader, a comprehensive toolkit for extracting and processing documentation from multiple file formats.

## 📚 Table of Contents

- [Quick Start](quick_start.md) - Get started quickly with Doc Loader
- [Python Usage](python_usage.md) - Detailed Python API documentation
- [Configuration](configuration.md) - Configuration options and settings
- [API Reference](api.md) - Complete API reference
- [Examples](examples.md) - Usage examples and code samples

## 🚀 Overview

Doc Loader is a powerful toolkit that allows you to:

- Extract text and metadata from PDF, TXT, JSON, CSV, and DOCX files
- Convert documents into a universal JSON format
- Perform intelligent text chunking and cleaning
- Detect document language automatically
- Optimize output for vector stores and RAG systems

## 📦 Installation

### Python Package

```bash
pip install doc-loader
```

### Rust Crate

```bash
cargo add doc_loader
```

## 🔧 Quick Usage

### Python

```python
import doc_loader

# Process any supported file format
result = doc_loader.process_file("document.pdf")
print(f"Extracted {result.chunk_count()} chunks")
```

### Rust

```rust
use doc_loader::{UniversalProcessor, ProcessingParams};

let processor = UniversalProcessor::new();
let result = processor.process_file("document.pdf", None)?;
println!("Extracted {} chunks", result.chunks.len());
```

## 🔗 Links

- [GitHub Repository](https://github.com/WillIsback/doc_loader)
- [PyPI Package](https://pypi.org/project/doc-loader/)
- [Crates.io Package](https://crates.io/crates/doc_loader)
- [Issue Tracker](https://github.com/WillIsback/doc_loader/issues)

## 📄 License

This project is licensed under the MIT License.
