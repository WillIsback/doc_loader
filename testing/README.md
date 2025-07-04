# 🧪 Testing Framework

This directory contains the complete testing and evaluation framework for the doc_loader project.

## 📁 Directory Structure

```
testing/
├── README.md           # This file - Testing framework overview
├── corpus/             # Test document collection
├── results/            # Generated JSON outputs from processing
├── tools/              # Testing and evaluation scripts
└── reports/            # Quality assessment reports
```

## 📄 Test Corpus (`corpus/`)

Curated collection of documents for comprehensive testing:

- **Text Files** (`.txt`):
  - `bible_kjv.txt` - Large text corpus (King James Bible)
  - `plato_republic.txt` - Classical literature (Plato's Republic)
  - `sample_document.txt` - Technical documentation sample
  - `corporate_report.txt` - Business document example

- **Structured Data** (`.csv`):
  - `books_dataset.csv` - Literary dataset
  - `companies_performance.csv` - Business performance data

- **JSON Documents** (`.json`):
  - `research_papers.json` - Academic papers collection
  - `scientific_articles.json` - Scientific literature

- **PDF Documents** (`.pdf`):
  - `sample_pdf.pdf` - W3C sample PDF document

## 🔧 Testing Tools (`tools/`)

- `evaluate_quality.py` - Automated quality assessment script
  - Analyzes chunk quality, metadata completeness
  - Generates statistical reports
  - Provides vector store readiness scoring

## 📊 Test Results (`results/`)

Generated JSON outputs from processing the corpus:
- One `.json` file per input document
- Standardized format for all document types
- Contains chunks, metadata, and processing information

## 📈 Quality Reports (`reports/`)

- `EVALUATION_REPORT.md` - Comprehensive quality assessment
  - Overall score: 8.9/10 (89%)
  - Format-specific performance analysis
  - Recommendations for production use

## 🚀 Usage

### Run Complete Test Suite
```bash
# From project root
./target/release/doc_loader --input testing/corpus/sample_document.txt --output testing/results/sample_output.json

# Evaluate quality
cd testing/tools
python3 evaluate_quality.py
```

### Individual Format Testing
```bash
# Test specific document types
./target/release/pdf_processor --input testing/corpus/sample_pdf.pdf
./target/release/json_processor --input testing/corpus/research_papers.json
```

## ✅ Quality Standards

- **Chunk Quality**: Proper segmentation with context preservation
- **Metadata Completeness**: Rich document and chunk-level metadata
- **Language Detection**: Automatic language identification
- **Vector Store Ready**: Optimized for embedding and indexing
- **Performance**: Fast processing with detailed timing information

## 🎯 Test Coverage

The testing framework covers:
- ✅ Multiple file formats (TXT, JSON, CSV, PDF)
- ✅ Various document sizes (small samples to large texts)
- ✅ Different content types (literature, technical, business)
- ✅ Edge cases and error handling
- ✅ Performance benchmarking
- ✅ Quality assessment automation
