#!/usr/bin/env python3
"""
Exemple d'utilisation des bindings Python de Doc Loader (extracteur-docs-rs)

Ce script démontre comment utiliser la bibliothèque Python pour traiter
différents types de documents et extraire des données structurées.

Installation: pip install extracteur-docs-rs
"""

import sys
import os
import json
from pathlib import Path

# Tentative d'import du module
try:
    import doc_loader
    print("✅ Module doc_loader (extracteur-docs-rs) importé avec succès")
except ImportError as e:
    print("❌ Erreur d'import du module doc_loader:")
    print(f"   {e}")
    print("\n💡 Pour résoudre ce problème:")
    print("   Option 1 (recommandée): pip install extracteur-docs-rs")
    print("   Option 2 (depuis les sources):")
    print("     1. Installez maturin: pip install maturin")
    print("     2. Compilez les bindings: maturin develop --features python")
    print("   3. Relancez ce script")
    sys.exit(1)


def demo_basic_usage():
    """Démonstration de l'usage de base"""
    print("\n🔧 === Démonstration Usage de Base ===")
    
    # Créer un processeur
    processor = doc_loader.PyUniversalProcessor()
    print("✅ Processeur créé")
    
    # Formats supportés
    extensions = processor.get_supported_extensions()
    print(f"📋 Formats supportés: {', '.join(extensions)}")
    
    # Test avec fichier d'exemple
    sample_file = "samples/test_sample.txt"
    if os.path.exists(sample_file):
        print(f"\n📄 Traitement du fichier: {sample_file}")
        
        # Traitement simple
        result = processor.process_file(sample_file)
        
        print(f"✅ Traitement terminé:")
        print(f"   📊 Chunks générés: {result.chunk_count()}")
        print(f"   📝 Mots totaux: {result.total_word_count()}")
        print(f"   🔤 Caractères totaux: {result.total_char_count()}")
        
        # Affichage des métadonnées
        metadata = result.document_metadata
        print(f"\n📋 Métadonnées:")
        print(f"   Nom: {metadata.filename}")
        print(f"   Type: {metadata.document_type}")
        print(f"   Taille: {metadata.file_size} bytes")
        
        # Affichage du premier chunk
        if result.chunks:
            first_chunk = result.chunks[0]
            print(f"\n📄 Premier chunk:")
            print(f"   ID: {first_chunk.id}")
            print(f"   Index: {first_chunk.chunk_index}")
            print(f"   Mots: {first_chunk.word_count}")
            print(f"   Contenu (50 premiers chars): {first_chunk.content[:50]}...")
    else:
        print(f"⚠️  Fichier d'exemple non trouvé: {sample_file}")


def demo_custom_parameters():
    """Démonstration avec paramètres personnalisés"""
    print("\n⚙️  === Démonstration Paramètres Personnalisés ===")
    
    # Paramètres personnalisés
    params = doc_loader.PyProcessingParams(
        chunk_size=400,
        overlap=50,
        clean_text=True,
        extract_metadata=True,
        preserve_formatting=False
    )
    
    print(f"⚙️  Paramètres configurés:")
    print(f"   Taille chunk: {params.chunk_size}")
    print(f"   Chevauchement: {params.overlap}")
    print(f"   Nettoyage texte: {params.clean_text}")
    
    # Test avec fichier d'exemple
    sample_file = "samples/test_sample.txt"
    if os.path.exists(sample_file):
        processor = doc_loader.PyUniversalProcessor()
        result = processor.process_file(sample_file, params)
        
        print(f"\n✅ Résultat avec paramètres personnalisés:")
        print(f"   📊 Chunks: {result.chunk_count()}")
        print(f"   📝 Mots: {result.total_word_count()}")
        
        # Analyse des tailles de chunks
        chunk_sizes = [len(chunk.content) for chunk in result.chunks]
        if chunk_sizes:
            print(f"   📏 Taille chunk moyenne: {sum(chunk_sizes) / len(chunk_sizes):.0f} chars")
            print(f"   📏 Taille min/max: {min(chunk_sizes)}/{max(chunk_sizes)} chars")


def demo_convenience_functions():
    """Démonstration des fonctions de commodité"""
    print("\n🛠️  === Démonstration Fonctions de Commodité ===")
    
    # Extensions supportées
    extensions = doc_loader.supported_extensions()
    print(f"📋 Extensions: {extensions}")
    
    # Traitement direct avec fonction de commodité
    sample_file = "samples/test_sample.txt"
    if os.path.exists(sample_file):
        print(f"\n📄 Traitement avec fonction de commodité:")
        result = doc_loader.process_file(sample_file, chunk_size=300)
        print(f"   ✅ {result.chunk_count()} chunks générés")
    
    # Traitement de texte direct
    sample_text = "Ceci est un exemple de texte à traiter. " * 10
    print(f"\n📝 Traitement de texte direct:")
    result = doc_loader.process_text(sample_text, chunk_size=100)
    print(f"   ✅ {result.chunk_count()} chunks générés")
    print(f"   📝 Texte original: {len(sample_text)} chars")
    print(f"   📝 Premier chunk: {result.chunks[0].content[:50]}...")


def demo_json_export():
    """Démonstration de l'export JSON"""
    print("\n💾 === Démonstration Export JSON ===")
    
    sample_file = "samples/test_sample.txt"
    if os.path.exists(sample_file):
        result = doc_loader.process_file(sample_file, chunk_size=200)
        
        # Export JSON
        json_output = result.to_json()
        
        # Sauvegarde
        output_file = "output_example.json"
        with open(output_file, "w", encoding="utf-8") as f:
            f.write(json_output)
        
        print(f"✅ JSON exporté vers: {output_file}")
        print(f"   📊 Taille du fichier: {len(json_output)} chars")
        
        # Affichage d'un extrait
        try:
            json_data = json.loads(json_output)
            print(f"   📋 Structure JSON:")
            print(f"      - document_metadata: {len(json_data.get('document_metadata', {}))} clés")
            print(f"      - chunks: {len(json_data.get('chunks', []))} éléments")
            print(f"      - total_chunks: {json_data.get('total_chunks', 'N/A')}")
        except json.JSONDecodeError:
            print("   ⚠️  Erreur de parsing JSON")


def demo_batch_processing():
    """Démonstration du traitement par lot"""
    print("\n🔄 === Démonstration Traitement par Lot ===")
    
    samples_dir = "samples"
    if os.path.exists(samples_dir):
        processor = doc_loader.PyUniversalProcessor()
        supported_exts = processor.get_supported_extensions()
        
        files_processed = 0
        total_chunks = 0
        total_words = 0
        
        print(f"📁 Traitement du répertoire: {samples_dir}")
        
        for filename in os.listdir(samples_dir):
            filepath = os.path.join(samples_dir, filename)
            if os.path.isfile(filepath):
                # Vérifier l'extension
                ext = os.path.splitext(filename)[1].lower().lstrip('.')
                if ext in supported_exts:
                    try:
                        print(f"   📄 Traitement: {filename}")
                        result = processor.process_file(filepath)
                        
                        files_processed += 1
                        total_chunks += result.chunk_count()
                        total_words += result.total_word_count()
                        
                        print(f"      ✅ {result.chunk_count()} chunks, {result.total_word_count()} mots")
                        
                    except Exception as e:
                        print(f"      ❌ Erreur: {e}")
                else:
                    print(f"   ⏭️  Ignoré (format non supporté): {filename}")
        
        print(f"\n📊 Résumé du traitement par lot:")
        print(f"   📁 Fichiers traités: {files_processed}")
        print(f"   🧩 Chunks totaux: {total_chunks}")
        print(f"   📝 Mots totaux: {total_words}")
    else:
        print(f"⚠️  Répertoire samples non trouvé: {samples_dir}")


def demo_error_handling():
    """Démonstration de la gestion d'erreurs"""
    print("\n⚠️  === Démonstration Gestion d'Erreurs ===")
    
    processor = doc_loader.PyUniversalProcessor()
    
    # Test avec fichier inexistant
    print("🧪 Test avec fichier inexistant:")
    try:
        result = processor.process_file("fichier_inexistant.txt")
        print("   ❌ Erreur: devrait lever une exception")
    except Exception as e:
        print(f"   ✅ Exception attendue: {type(e).__name__}: {e}")
    
    # Test avec format non supporté
    print("\n🧪 Test avec fichier non supporté:")
    # Créer un fichier temporaire avec extension non supportée
    temp_file = "temp_file.xyz"
    try:
        with open(temp_file, "w") as f:
            f.write("Test content")
        
        result = processor.process_file(temp_file)
        print("   ❌ Erreur: devrait lever une exception")
    except Exception as e:
        print(f"   ✅ Exception attendue: {type(e).__name__}: {e}")
    finally:
        if os.path.exists(temp_file):
            os.remove(temp_file)


def main():
    """Fonction principale"""
    print("🐍 === Démonstration des Bindings Python Doc Loader ===")
    print(f"📍 Répertoire de travail: {os.getcwd()}")
    
    # Exécution des démonstrations
    demo_basic_usage()
    demo_custom_parameters()
    demo_convenience_functions()
    demo_json_export()
    demo_batch_processing()
    demo_error_handling()
    
    print("\n🎉 === Démonstration Terminée ===")
    print("\n💡 Pour plus d'informations:")
    print("   📖 Consultez docs/python_usage.md")
    print("   🔧 Consultez docs/api.md")
    print("   📋 Consultez docs/examples.md")


if __name__ == "__main__":
    main()
