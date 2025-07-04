# 📖 Exemples d'Usage - Doc Loader

Ce fichier contient des exemples pratiques d'utilisation des outils Doc Loader.

## Fichiers d'Exemple

Le répertoire `samples/` contient des fichiers de test :
- `test_sample.txt` - Fichier texte simple
- `test_sample.json` - Document JSON structuré  
- `test_sample.csv` - Données tabulaires CSV
- `test_sample.pdf.txt` - Contenu extrait d'un PDF

## Exemples de Base

### Traitement d'un Fichier Texte
```bash
# Traitement simple
./target/release/txt_processor samples/test_sample.txt

# Avec sauvegarde
./target/release/txt_processor samples/test_sample.txt > output.json
```

### Traitement d'un Fichier JSON
```bash
# Extraction et normalisation
./target/release/json_processor samples/test_sample.json

# Avec processeur universel
./target/release/doc_loader --input samples/test_sample.json --output result.json
```

### Traitement d'un Fichier CSV
```bash
# Analyse des colonnes et données
./target/release/csv_processor samples/test_sample.csv

# Chunking personnalisé
./target/release/doc_loader --input samples/test_sample.csv --chunk-size 500
```

## Exemples Avancés

### Script de Traitement par Lot
```bash
#!/bin/bash
# process_all.sh
SAMPLES_DIR="samples"
OUTPUT_DIR="processed"

mkdir -p "$OUTPUT_DIR"

for file in "$SAMPLES_DIR"/*; do
    if [ -f "$file" ]; then
        filename=$(basename "$file")
        echo "Processing: $filename"
        ./target/release/doc_loader \
            --input "$file" \
            --output "$OUTPUT_DIR/${filename%.*}.json" \
            --chunk-size 600 \
            --overlap 80
    fi
done

echo "Traitement terminé. Résultats dans $OUTPUT_DIR/"
```

### Validation et Tests
```bash
# Test de tous les formats
echo "=== Test TXT ==="
./target/release/txt_processor samples/test_sample.txt | jq '.chunks | length'

echo "=== Test JSON ==="  
./target/release/json_processor samples/test_sample.json | jq '.metadata.file_name'

echo "=== Test CSV ==="
./target/release/csv_processor samples/test_sample.csv | jq '.chunks[0].content'

echo "=== Test avec processeur universel ==="
./target/release/doc_loader --input samples/test_sample.txt --chunk-size 200
```

### Pipeline de Traitement pour RAG
```bash
#!/bin/bash
# rag_pipeline.sh

INPUT_FILE="$1"
OUTPUT_DIR="rag_chunks"
CHUNK_SIZE=400
OVERLAP=60

if [ -z "$INPUT_FILE" ]; then
    echo "Usage: $0 <input_file>"
    exit 1
fi

mkdir -p "$OUTPUT_DIR"

# Traitement principal
echo "Traitement de $INPUT_FILE pour RAG..."
./target/release/doc_loader \
    --input "$INPUT_FILE" \
    --chunk-size $CHUNK_SIZE \
    --overlap $OVERLAP \
    --output "$OUTPUT_DIR/$(basename "$INPUT_FILE" | cut -d. -f1)_chunks.json"

# Extraction des chunks pour embedding
echo "Extraction des chunks..."
jq -r '.chunks[] | .content' "$OUTPUT_DIR/$(basename "$INPUT_FILE" | cut -d. -f1)_chunks.json" \
    > "$OUTPUT_DIR/$(basename "$INPUT_FILE" | cut -d. -f1)_text_only.txt"

echo "Pipeline terminé:"
echo "- JSON structuré: $OUTPUT_DIR/$(basename "$INPUT_FILE" | cut -d. -f1)_chunks.json"
echo "- Texte pur: $OUTPUT_DIR/$(basename "$INPUT_FILE" | cut -d. -f1)_text_only.txt"
```

## Intégration avec d'Autres Outils

### Avec jq pour le Post-traitement
```bash
# Extraire seulement les métadonnées
./target/release/doc_loader --input samples/test_sample.txt | jq '.metadata'

# Compter le nombre de chunks
./target/release/doc_loader --input samples/test_sample.txt | jq '.chunks | length'

# Filtrer les chunks par taille
./target/release/doc_loader --input samples/test_sample.txt | \
    jq '.chunks[] | select(.word_count > 50)'
```

### Avec curl pour API REST
```bash
# Exemple d'envoi vers une API d'embedding
./target/release/doc_loader --input samples/test_sample.txt | \
    jq '.chunks[].content' | \
    curl -X POST https://api.openai.com/v1/embeddings \
         -H "Content-Type: application/json" \
         -H "Authorization: Bearer $OPENAI_API_KEY" \
         -d @-
```

## Benchmarking et Performance

### Test de Performance
```bash
#!/bin/bash
# benchmark.sh

echo "=== Benchmark Doc Loader ==="
for file in samples/*; do
    echo "File: $(basename "$file")"
    time ./target/release/doc_loader --input "$file" > /dev/null
    echo "---"
done
```

### Monitoring Mémoire
```bash
# Avec GNU time pour stats détaillées
/usr/bin/time -v ./target/release/doc_loader --input large_document.pdf 2>&1 | \
    grep -E "(Maximum resident|User time|System time)"
```
