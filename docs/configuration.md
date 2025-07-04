# 🛠️ Configuration et Exemples

## Configuration par Défaut

Le projet utilise des paramètres optimisés par défaut :

```rust
ProcessingParams {
    chunk_size: 800,        // Taille optimale pour RAG
    overlap: 100,           // Chevauchement entre chunks
    preserve_formatting: false,
    extract_metadata: true,
    clean_text: true,
}
```

## Formats de Sortie

### Structure JSON Universelle
```json
{
  "document_id": "doc_123456",
  "document_type": "PDF",
  "metadata": {
    "file_name": "document.pdf",
    "file_size": 1024000,
    "created_at": "2025-07-04T10:30:00Z",
    "pages": 10,
    "author": "Author Name",
    "title": "Document Title"
  },
  "chunks": [
    {
      "id": "chunk_1",
      "content": "Contenu du premier chunk...",
      "word_count": 120,
      "char_count": 800,
      "position": {
        "start": 0,
        "end": 800,
        "page": 1
      },
      "metadata": {
        "language": "fr",
        "chunk_type": "paragraph"
      }
    }
  ],
  "processing_time_ms": 250,
  "created_at": "2025-07-04T10:30:00Z"
}
```

## Exemples d'Usage

### 1. Traitement Simple
```bash
# Traiter un PDF
./target/release/doc_loader --input document.pdf

# Traiter un fichier texte
./target/release/doc_loader --input notes.txt
```

### 2. Options Avancées
```bash
# Chunks plus petits pour embedding
./target/release/doc_loader \
    --input document.pdf \
    --chunk-size 500 \
    --overlap 50 \
    --output chunks.json

# Traitement par lot
find ./documents -name "*.pdf" -exec ./target/release/pdf_processor {} \;
```

### 3. Intégration dans Scripts
```bash
#!/bin/bash
for file in documents/*.pdf; do
    echo "Processing: $file"
    ./target/release/pdf_processor "$file" > "processed_$(basename "$file" .pdf).json"
done
```

## Optimisation pour Vector Stores

### Paramètres Recommandés
- **Chunk size**: 400-800 caractères pour embedding
- **Overlap**: 10-15% de la taille du chunk
- **Clean text**: Activé pour consistance
- **Extract metadata**: Activé pour enrichissement

### Formats Compatibles
- ✅ OpenAI Embeddings
- ✅ Sentence Transformers
- ✅ Elasticsearch
- ✅ Pinecone
- ✅ Weaviate
- ✅ Chroma

## Debugging et Logs

```bash
# Logs détaillés
RUST_LOG=debug ./target/release/doc_loader --input document.pdf

# Profiling des performances
time ./target/release/doc_loader --input large_document.pdf
```
