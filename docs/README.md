# Documentation Index

Welcome to the Doc Loader documentation! This comprehensive guide will help you understand and use all features of the Doc Loader library.

## 📚 Documentation Structure

### Getting Started
- **[Quick Start Guide](quick_start.md)** - Get up and running in minutes
- **[API Reference](api.md)** - Complete API documentation
- **[Configuration](configuration.md)** - Configuration options and parameters

### Usage Examples
- **[Examples](examples.md)** - Practical usage examples for all formats
- **[Python Usage](python_usage.md)** - Complete Python integration guide

### Python Bindings
- **[Python Bindings](python_bindings.md)** - Implementation status and features

## 🚀 Quick Navigation

### By Use Case
- **Command Line Usage**: See [Quick Start Guide](quick_start.md)
- **Rust Library Integration**: See [API Reference](api.md)
- **Python Integration**: See [Python Usage](python_usage.md)
- **Custom Configuration**: See [Configuration](configuration.md)

### By File Format
- **PDF Processing**: Examples in [Examples](examples.md#pdf-processing)
- **Text Processing**: Examples in [Examples](examples.md#text-processing)
- **JSON Processing**: Examples in [Examples](examples.md#json-processing)
- **CSV Processing**: Examples in [Examples](examples.md#csv-processing)
- **DOCX Processing**: Examples in [Examples](examples.md#docx-processing)

## 🔧 Development

### Architecture
The Doc Loader follows a modular architecture with:
- **Core Types**: Universal data structures in `src/core/`
- **Processors**: Format-specific processors in `src/processors/`
- **CLI Tools**: Command-line interfaces in `src/bin/`
- **Python Bindings**: PyO3 bindings in `src/python.rs`

### Contributing
1. Read the [API Reference](api.md) to understand the architecture
2. Check [Examples](examples.md) for implementation patterns
3. Refer to [Configuration](configuration.md) for parameter options

## 📋 Quick Reference

### Supported Formats
- **PDF** (.pdf) - Text extraction with metadata
- **Text** (.txt) - Plain text processing with encoding detection
- **JSON** (.json) - Structured data analysis
- **CSV** (.csv) - Tabular data processing with statistics
- **DOCX** (.docx) - Microsoft Word document processing

### Key Features
- ✅ Universal JSON output format
- ✅ Intelligent text chunking
- ✅ Comprehensive metadata extraction
- ✅ Python bindings (production ready)
- ✅ CLI tools for all formats
- ✅ Configurable processing parameters
- ✅ Error handling and validation

### Build Requirements
- **Rust**: 1.70+ for library compilation
- **Python**: 3.13+ for Python bindings
- **Maturin**: For Python binding builds

## 🎯 Next Steps

1. **New Users**: Start with [Quick Start Guide](quick_start.md)
2. **Python Developers**: Go to [Python Usage](python_usage.md)
3. **Rust Developers**: Check [API Reference](api.md)
4. **Advanced Configuration**: See [Configuration](configuration.md)

---

**Doc Loader** - Making document processing simple, fast, and universal! 🚀
