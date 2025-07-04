# 🧪 Testing Configuration

This file contains configuration settings for the doc_loader testing framework.

## Test Parameters

### Processing Settings
- **Default Chunk Size:** 1200 characters
- **Default Chunk Overlap:** 120 characters (10% of chunk size)
- **Language Detection:** Enabled by default
- **Output Format:** Pretty-printed JSON

### Performance Thresholds
- **Maximum Processing Time:** 120 seconds per file
- **Minimum Success Rate:** 90%
- **Target Processing Speed:** 1000+ words/second

### Quality Standards
- **Chunk Quality Score:** ≥ 8.0/10
- **Metadata Completeness:** ≥ 9.0/10
- **Content Preservation:** ≥ 9.5/10
- **Vector Store Readiness:** ≥ 8.5/10

## Supported File Formats

### Text Documents
- `.txt` - Plain text files
- Size range: 1KB - 10MB
- Expected chunk count: 10-1000

### Structured Data
- `.csv` - Comma-separated values
- `.json` - JavaScript Object Notation
- Size range: 1KB - 5MB
- Expected processing: Schema-aware chunking

### Document Formats
- `.pdf` - Portable Document Format
- Size range: 10KB - 50MB
- Expected processing: Text extraction + OCR fallback

### Future Support
- `.docx` - Microsoft Word documents
- `.md` - Markdown files
- `.html` - HTML documents

## Test Corpus Requirements

### Diversity
- Multiple file formats
- Various content types (technical, business, academic)
- Different file sizes (small, medium, large)
- Multiple languages (English primary, others secondary)

### Quality
- Valid file formats
- Readable content
- No corrupted files
- Representative of real-world usage

### Maintenance
- Regular updates to corpus
- Removal of broken/corrupted files
- Addition of edge cases
- Version control for test data

## Reporting Standards

### Automated Reports
- JSON detailed results
- Markdown summary reports
- Performance benchmarks
- Quality assessments

### Manual Review
- Edge case analysis
- Quality spot checks
- Performance regression testing
- User acceptance validation
