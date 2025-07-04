# 🐍 Utilisation Python - Doc Loader

Doc Loader peut être utilisé comme une bibliothèque Python native grâce aux bindings PyO3. Cette section explique comment installer et utiliser la bibliothèque Python.

## 📦 Installation

### Via PyPI (Recommandé)
```bash
pip install extracteur-docs-rs
```

## 🔧 Construction depuis les Sources

### Prérequis
```bash
# Installation de maturin (outil de build PyO3)
pip install maturin

# Optionnel: environnement virtuel
python -m venv venv
source venv/bin/activate  # Linux/macOS
# ou
venv\Scripts\activate     # Windows
```

### Build et Installation
```bash
# Depuis le répertoire du projet
cd /path/to/doc_loader

# Build et installation en mode développement
maturin develop --features python

# Ou build pour distribution
maturin build --features python --release
```

## 📚 API Python

### Classe Principale: UniversalProcessor

```python
import doc_loader

# Créer un processeur
processor = doc_loader.PyUniversalProcessor()

# Traitement d'un fichier
result = processor.process_file("document.pdf")
print(f"Nombre de chunks: {result.chunk_count()}")
print(f"Mots totaux: {result.total_word_count()}")

# Extensions supportées
extensions = processor.get_supported_extensions()
print("Formats supportés:", extensions)
```

### Configuration des Paramètres

```python
# Paramètres personnalisés
params = doc_loader.PyProcessingParams(
    chunk_size=500,
    overlap=50,
    clean_text=True,
    extract_metadata=True,
    preserve_formatting=False
)

# Traitement avec paramètres
result = processor.process_file("document.pdf", params)
```

### Fonctions de Commodité

```python
# Traitement simple d'un fichier
result = doc_loader.process_file("document.txt", chunk_size=600)

# Traitement de texte direct
result = doc_loader.process_text("Votre texte ici...", chunk_size=400)

# Extensions supportées
extensions = doc_loader.supported_extensions()
```

## 🔍 Exploration des Résultats

### Structure des Données

```python
result = doc_loader.process_file("samples/test_sample.txt")

# Métadonnées du document
metadata = result.document_metadata
print(f"Nom: {metadata.filename}")
print(f"Type: {metadata.document_type}")
print(f"Taille: {metadata.file_size} bytes")
print(f"Auteur: {metadata.author}")

# Chunks individuels
for chunk in result.chunks:
    print(f"Chunk {chunk.chunk_index}:")
    print(f"  Contenu: {chunk.content[:100]}...")
    print(f"  Mots: {chunk.word_count}")
    print(f"  Caractères: {chunk.char_count}")

# Export JSON
json_output = result.to_json()
with open("output.json", "w") as f:
    f.write(json_output)
```

## 📄 Exemples Pratiques

### 1. Traitement par Lot

```python
import os
import doc_loader

def process_directory(directory_path, output_dir):
    """Traite tous les fichiers supportés d'un répertoire"""
    processor = doc_loader.PyUniversalProcessor()
    supported_exts = processor.get_supported_extensions()
    
    os.makedirs(output_dir, exist_ok=True)
    
    for filename in os.listdir(directory_path):
        file_path = os.path.join(directory_path, filename)
        if os.path.isfile(file_path):
            # Vérifier l'extension
            ext = os.path.splitext(filename)[1].lower().lstrip('.')
            if ext in supported_exts:
                print(f"Traitement: {filename}")
                try:
                    result = processor.process_file(file_path)
                    
                    # Sauvegarder le résultat
                    output_file = os.path.join(output_dir, f"{filename}.json")
                    with open(output_file, "w", encoding="utf-8") as f:
                        f.write(result.to_json())
                    
                    print(f"  ✅ {result.chunk_count()} chunks, {result.total_word_count()} mots")
                except Exception as e:
                    print(f"  ❌ Erreur: {e}")

# Utilisation
process_directory("./documents", "./processed")
```

### 2. Pipeline d'Embedding

```python
import doc_loader
import numpy as np
from sentence_transformers import SentenceTransformer

def create_embeddings_pipeline(file_path, model_name="all-MiniLM-L6-v2"):
    """Pipeline complet: extraction + embedding"""
    
    # 1. Traitement du document
    processor = doc_loader.PyUniversalProcessor()
    params = doc_loader.PyProcessingParams(
        chunk_size=400,  # Optimal pour embedding
        overlap=60,
        clean_text=True
    )
    
    result = processor.process_file(file_path, params)
    
    # 2. Extraction du texte des chunks
    text_chunks = result.get_text_chunks()
    
    # 3. Génération des embeddings
    model = SentenceTransformer(model_name)
    embeddings = model.encode(text_chunks)
    
    # 4. Résultat structuré
    return {
        "metadata": {
            "filename": result.document_metadata.filename,
            "total_chunks": result.chunk_count(),
            "total_words": result.total_word_count()
        },
        "chunks": [
            {
                "text": chunk.content,
                "embedding": embeddings[i].tolist(),
                "metadata": {
                    "chunk_id": chunk.id,
                    "word_count": chunk.word_count,
                    "char_count": chunk.char_count
                }
            }
            for i, chunk in enumerate(result.chunks)
        ]
    }

# Utilisation
embeddings_data = create_embeddings_pipeline("document.pdf")
print(f"Généré {len(embeddings_data['chunks'])} embeddings")
```

### 3. Intégration avec Pandas

```python
import doc_loader
import pandas as pd

def document_to_dataframe(file_path):
    """Convertit un document en DataFrame pandas"""
    
    result = doc_loader.process_file(file_path)
    
    # Création du DataFrame
    data = []
    for chunk in result.chunks:
        data.append({
            "chunk_id": chunk.id,
            "chunk_index": chunk.chunk_index,
            "content": chunk.content,
            "word_count": chunk.word_count,
            "char_count": chunk.char_count,
            "size": chunk.size,
            "source_file": result.document_metadata.filename,
            "document_type": result.document_metadata.document_type
        })
    
    df = pd.DataFrame(data)
    
    # Ajout de métadonnées comme attributs
    df.attrs["metadata"] = {
        "filename": result.document_metadata.filename,
        "file_size": result.document_metadata.file_size,
        "total_chunks": result.chunk_count(),
        "total_words": result.total_word_count()
    }
    
    return df

# Utilisation
df = document_to_dataframe("samples/test_sample.txt")
print(df.head())
print(f"Métadonnées: {df.attrs['metadata']}")

# Analyse
print(f"Chunk moyen: {df['word_count'].mean():.1f} mots")
print(f"Chunk le plus long: {df['word_count'].max()} mots")
```

### 4. API Flask Simple

```python
from flask import Flask, request, jsonify
import doc_loader
import tempfile
import os

app = Flask(__name__)

@app.route('/process', methods=['POST'])
def process_document():
    """Endpoint pour traiter un document uploadé"""
    
    if 'file' not in request.files:
        return jsonify({"error": "Aucun fichier fourni"}), 400
    
    file = request.files['file']
    if file.filename == '':
        return jsonify({"error": "Nom de fichier vide"}), 400
    
    # Paramètres optionnels
    chunk_size = request.form.get('chunk_size', 800, type=int)
    overlap = request.form.get('overlap', 100, type=int)
    
    # Sauvegarde temporaire
    with tempfile.NamedTemporaryFile(delete=False, suffix=os.path.splitext(file.filename)[1]) as tmp:
        file.save(tmp.name)
        
        try:
            # Traitement
            processor = doc_loader.PyUniversalProcessor()
            params = doc_loader.PyProcessingParams(
                chunk_size=chunk_size,
                overlap=overlap
            )
            
            result = processor.process_file(tmp.name, params)
            
            # Réponse
            response = {
                "success": True,
                "document": {
                    "filename": result.document_metadata.filename,
                    "type": result.document_metadata.document_type,
                    "size": result.document_metadata.file_size
                },
                "processing": {
                    "total_chunks": result.chunk_count(),
                    "total_words": result.total_word_count(),
                    "total_chars": result.total_char_count()
                },
                "chunks": [
                    {
                        "id": chunk.id,
                        "content": chunk.content,
                        "word_count": chunk.word_count
                    }
                    for chunk in result.chunks
                ]
            }
            
            return jsonify(response)
            
        except Exception as e:
            return jsonify({"error": str(e)}), 500
        
        finally:
            os.unlink(tmp.name)

@app.route('/supported_formats', methods=['GET'])
def get_supported_formats():
    """Liste des formats supportés"""
    extensions = doc_loader.supported_extensions()
    return jsonify({"supported_extensions": extensions})

if __name__ == '__main__':
    app.run(debug=True)
```

## 🚀 Script d'Installation Automatique

```bash
#!/bin/bash
# install_python_bindings.sh

set -e

echo "🐍 Installation des bindings Python pour Doc Loader"

# Vérification de Python
if ! command -v python3 &> /dev/null; then
    echo "❌ Python 3 requis"
    exit 1
fi

# Vérification de Rust
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust/Cargo requis"
    exit 1
fi

# Installation de maturin
echo "📦 Installation de maturin..."
pip install maturin

# Build et installation
echo "🔨 Build des bindings Python..."
maturin develop --features python

# Test d'importation
echo "🧪 Test d'importation..."
python3 -c "
import doc_loader
processor = doc_loader.PyUniversalProcessor()
print('✅ Import réussi!')
print('📋 Extensions supportées:', processor.get_supported_extensions())
"

echo "🎉 Installation terminée avec succès!"
echo ""
echo "Utilisation:"
echo "  import doc_loader"
echo "  result = doc_loader.process_file('document.pdf')"
```

## 🔍 Debugging et Troubleshooting

### Problèmes Courants

1. **Import Error**: Vérifiez que maturin a bien compilé avec `--features python`
2. **Version Python**: PyO3 nécessite Python 3.7+
3. **Rust Version**: Utilisez Rust 1.70+

### Debug Build

```bash
# Build en mode debug pour plus d'infos d'erreur
maturin develop --features python

# Logs détaillés
RUST_LOG=debug python script.py
```

Cette documentation couvre l'intégration Python complète de Doc Loader, avec des exemples pratiques pour différents cas d'usage.
