# 🐍 Doc Loader - Python Package

[![Python](https://img.shields.io/badge/python-3.9%2B-blue.svg)](https://www.python.org/)
[![PyPI](https://img.shields.io/pypi/v/doc-loader.svg)](https://pypi.org/project/doc-loader/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Documentation](https://img.shields.io/badge/docs-GitHub%20Pages-blue.svg)](https://willisback.github.io/doc_loader/)

A comprehensive Python toolkit for extracting and processing documentation from multiple file formats (PDF, TXT, JSON, CSV, DOCX) into a universal JSON structure, optimized for vector stores and RAG (Retrieval-Augmented Generation) systems.

## 🚀 Features

- **✅ Universal JSON Output**: Consistent format across all document types
- **✅ Multiple Format Support**: PDF, TXT, JSON, CSV, DOCX
- **✅ Native Performance**: Rust backend with PyO3 bindings
- **✅ Intelligent Text Processing**: Smart chunking, cleaning, and metadata extraction
- **✅ Vector Store Ready**: Optimized output for embedding and indexing
- **✅ Rich Metadata**: Comprehensive document and chunk-level metadata
- **✅ Language Detection**: Automatic language detection capabilities

## 📦 Installation

```bash
pip install doc-loader
```

## 🔧 Usage

### Quick Start

```python
import doc_loader

# Process any supported file format
result = doc_loader.process_file("document.pdf", chunk_size=500)

print(f"Chunks: {result.chunk_count()}")
print(f"Words: {result.total_word_count()}")
print(f"Supported formats: {doc_loader.supported_extensions()}")
```

### Advanced Usage

```python
import doc_loader

# Create a processor with custom parameters
processor = doc_loader.PyUniversalProcessor()
params = doc_loader.PyProcessingParams(
    chunk_size=400,
    overlap=60,
    clean_text=True,
    extract_metadata=True
)

# Process a file
result = processor.process_file("document.txt", params)

# Process text content directly
text_result = processor.process_text_content("Your text here...", params)

# Export to JSON
json_output = result.to_json()
```

## 📋 Output Format

All processors generate a standardized JSON structure:

```json
{
  "document_metadata": {
    "filename": "document.pdf",
    "document_type": "PDF",
    "file_size": 1024000,
    "title": "Document Title",
    "author": "Author Name"
  },
  "chunks": [
    {
      "id": "pdf_chunk_0",
      "content": "Extracted text content...",
      "chunk_index": 0,
      "metadata": {
        "size": 1000,
        "language": "en",
        "confidence": 0.95
      }
    }
  ],
  "processing_info": {
    "processor": "PdfProcessor",
    "processing_time_ms": 150,
    "total_chunks": 5,
    "total_content_size": 5000
  }
}
```

## 📊 Supported Formats

- **PDF**: Text extraction with metadata, page tracking
- **TXT**: Encoding detection, language detection, smart chunking
- **JSON**: Hierarchical analysis, key extraction, schema inference
- **CSV**: Header detection, column analysis, data completeness
- **DOCX**: Document structure, style preservation, metadata

## 🔧 API Reference

### Main Functions

```python
# Process any file format
result = doc_loader.process_file(file_path, **options)

# Get supported file extensions
extensions = doc_loader.supported_extensions()
```

### PyUniversalProcessor Class

```python
processor = doc_loader.PyUniversalProcessor()

# Process file with custom parameters
result = processor.process_file(file_path, params)

# Process text content directly
result = processor.process_text_content(text, params)
```

### PyProcessingParams Class

```python
params = doc_loader.PyProcessingParams(
    chunk_size=1000,          # Maximum chunk size
    overlap=100,              # Overlap between chunks
    clean_text=True,          # Enable text cleaning
    extract_metadata=True,    # Extract rich metadata
    detect_language=True      # Enable language detection
)
```

### Result Methods

```python
# Get chunk count
count = result.chunk_count()

# Get total word count
words = result.total_word_count()

# Export to JSON string
json_str = result.to_json()

# Get processing info
info = result.get_processing_info()
```

## 🔗 Integration Examples

### RAG/Embedding Pipeline

```python
import doc_loader
from sentence_transformers import SentenceTransformer

# Process document
result = doc_loader.process_file("document.pdf")

# Extract chunks for embedding
chunks = [chunk["content"] for chunk in result.to_json()["chunks"]]

# Generate embeddings
model = SentenceTransformer('all-MiniLM-L6-v2')
embeddings = model.encode(chunks)
```

### Batch Processing

```python
import doc_loader
import os

def process_directory(directory_path):
    results = []
    for filename in os.listdir(directory_path):
        if any(filename.endswith(ext) for ext in doc_loader.supported_extensions()):
            file_path = os.path.join(directory_path, filename)
            result = doc_loader.process_file(file_path)
            results.append(result)
    return results

# Process all documents in a directory
results = process_directory("./documents/")
```

### REST API Integration

```python
from flask import Flask, request, jsonify
import doc_loader

app = Flask(__name__)

@app.route('/process', methods=['POST'])
def process_document():
    if 'file' not in request.files:
        return jsonify({'error': 'No file provided'}), 400
    
    file = request.files['file']
    if file.filename == '':
        return jsonify({'error': 'No file selected'}), 400
    
    # Save temporary file
    temp_path = f"/tmp/{file.filename}"
    file.save(temp_path)
    
    try:
        # Process document
        result = doc_loader.process_file(temp_path)
        return jsonify(result.to_json())
    finally:
        os.remove(temp_path)

if __name__ == '__main__':
    app.run()
```

## 📈 Performance

- **Fast Processing**: Rust backend for optimal performance
- **Memory Efficient**: Streaming processing for large files
- **Concurrent Support**: Thread-safe processors
- **Scalable**: Suitable for production workloads

## 🔗 Links

- **Documentation**: https://willisback.github.io/doc_loader/
- **Source Code**: https://github.com/WillIsback/doc_loader
- **Issue Tracker**: https://github.com/WillIsback/doc_loader/issues

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](https://github.com/WillIsback/doc_loader/blob/master/LICENSE) file for details.
