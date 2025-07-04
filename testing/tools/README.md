# 🔧 Testing Tools

This directory contains automated testing and evaluation scripts for the doc_loader project.

## 📋 Available Tools

### `evaluate_quality.py` - Quality Assessment Script

Comprehensive automated evaluation tool that analyzes the quality of doc_loader outputs.

#### Features:
- **Statistical Analysis**: Word count, chunk distribution, metadata completeness
- **Quality Scoring**: Automated scoring system (0-10 scale)
- **Vector Store Readiness**: Assessment for embedding and indexing use
- **Format-Specific Analysis**: Tailored evaluation for each document type
- **Performance Metrics**: Processing time and efficiency analysis

#### Usage:
```bash
cd testing/tools
python3 evaluate_quality.py
```

#### Output:
- Console report with detailed statistics
- Quality scores for each processed document
- Overall project assessment
- Recommendations for production deployment

#### Requirements:
- Python 3.9+
- Standard library only (no external dependencies)

## 🎯 Quality Metrics

The evaluation script measures:

1. **Chunk Quality (0-10)**
   - Proper text segmentation
   - Context preservation
   - Optimal chunk sizes

2. **Metadata Completeness (0-10)**
   - Document-level metadata
   - Chunk-level information
   - Processing statistics

3. **Content Preservation (0-10)**
   - Text integrity
   - Formatting retention
   - Character encoding handling

4. **Vector Store Readiness (0-10)**
   - Chunk uniformity
   - Metadata richness
   - Embedding compatibility

## 📊 Scoring System

- **9.0 - 10.0**: Excellent - Production ready
- **8.0 - 8.9**: Very Good - Minor optimizations recommended
- **7.0 - 7.9**: Good - Some improvements needed
- **6.0 - 6.9**: Fair - Significant improvements required
- **< 6.0**: Poor - Major issues to address

## 🚀 Adding New Tools

To add new testing tools:

1. Create your script in this directory
2. Follow the naming convention: `[purpose]_[tool].py`
3. Add documentation to this README
4. Include usage examples
5. Specify requirements and dependencies
