# 🚀 Guide de Démarrage Rapide - Doc Loader

## Installation et Build

```bash
# Cloner et builder le projet
git clone <votre-repo>
cd doc_loader
cargo build --release
```

## Tests Rapides

```bash
# Vérifier que tout fonctionne
cargo test

# Test avec les fichiers d'exemple
cargo run --bin doc_loader -- --input samples/test_sample.txt
```

## Utilisation des Outils

### 1. Processeur Universel
```bash
# Traiter n'importe quel format supporté
./target/release/doc_loader --input document.pdf --output output.json

# Avec options personnalisées
./target/release/doc_loader \
    --input document.pdf \
    --chunk-size 1000 \
    --overlap 100 \
    --output result.json
```

### 2. Processeurs Spécialisés
```bash
# PDF
./target/release/pdf_processor document.pdf

# Texte brut
./target/release/txt_processor document.txt

# JSON
./target/release/json_processor data.json

# CSV
./target/release/csv_processor data.csv

# DOCX
./target/release/docx_processor document.docx
```

## Intégration Python

```python
# Utiliser le wrapper Python fourni
from python_wrapper import DocLoaderPy

loader = DocLoaderPy()
result = loader.extract_text("document.pdf", max_chunk_size=800)
print(f"Extracted {len(result['chunks'])} chunks")
```

## Structure de Sortie JSON

Le format de sortie est optimisé pour les vector stores :

```json
{
  "metadata": {
    "title": "Document Title",
    "page_count": 25,
    "file_size": 2048576
  },
  "chunks": [
    {
      "content": "Texte du chunk...",
      "metadata": {
        "chunk_index": 0,
        "word_count": 85,
        "chunk_hash": "abc123"
      }
    }
  ],
  "source_file": "document.pdf",
  "processing_timestamp": "2024-01-01T12:00:00Z"
}
```

Voir le README principal pour la documentation complète.
