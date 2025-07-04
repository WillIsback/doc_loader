#!/usr/bin/env python3
"""
Script d'évaluation de la qualité des résultats du doc_loader
Analyse les fichiers de sortie pour évaluer leur pertinence pour un vector store
"""

import json
import statistics
from pathlib import Path
from typing import Dict, List, Any

def analyze_document_output(filepath: str) -> Dict[str, Any]:
    """Analyse un fichier de sortie du doc_loader"""
    with open(filepath, 'r', encoding='utf-8') as f:
        data = json.load(f)
    
    # Extraction des métadonnées principales
    metadata = data['document_metadata']
    chunks = data['chunks']
    processing_info = data['processing_info']
    
    # Calculs statistiques sur les chunks
    chunk_sizes = [chunk['metadata']['size'] for chunk in chunks]
    chunk_word_counts = [chunk['metadata']['format_specific']['word_count'] for chunk in chunks if 'word_count' in chunk['metadata']['format_specific']]
    
    analysis = {
        'filename': metadata['filename'],
        'document_type': metadata['document_type'],
        'file_size': metadata['file_size'],
        'total_chunks': processing_info['total_chunks'],
        'processing_time_ms': processing_info['processing_time_ms'],
        'total_content_size': processing_info['total_content_size'],
        'chunks_analysis': {
            'count': len(chunks),
            'avg_size': statistics.mean(chunk_sizes) if chunk_sizes else 0,
            'median_size': statistics.median(chunk_sizes) if chunk_sizes else 0,
            'min_size': min(chunk_sizes) if chunk_sizes else 0,
            'max_size': max(chunk_sizes) if chunk_sizes else 0,
            'size_std_dev': statistics.stdev(chunk_sizes) if len(chunk_sizes) > 1 else 0,
            'avg_word_count': statistics.mean(chunk_word_counts) if chunk_word_counts else 0,
        },
        'performance': {
            'chars_per_ms': processing_info['total_content_size'] / processing_info['processing_time_ms'] if processing_info['processing_time_ms'] > 0 else 0,
            'chunks_per_ms': processing_info['total_chunks'] / processing_info['processing_time_ms'] if processing_info['processing_time_ms'] > 0 else 0,
        },
        'vector_store_quality': {
            'optimal_chunk_size_ratio': sum(1 for size in chunk_sizes if 500 <= size <= 1500) / len(chunk_sizes) if chunk_sizes else 0,
            'content_variety': len(set(chunk['content'][:100] for chunk in chunks)) / len(chunks) if chunks else 0,
            'metadata_richness': calculate_metadata_richness(chunks),
        }
    }
    
    return analysis

def calculate_metadata_richness(chunks: List[Dict]) -> float:
    """Calcule la richesse des métadonnées (entre 0 et 1)"""
    if not chunks:
        return 0
    
    metadata_fields = ['size', 'language', 'confidence', 'format_specific']
    total_fields = len(metadata_fields) * len(chunks)
    filled_fields = 0
    
    for chunk in chunks:
        for field in metadata_fields:
            if field in chunk['metadata'] and chunk['metadata'][field] is not None:
                filled_fields += 1
    
    return filled_fields / total_fields if total_fields > 0 else 0

def evaluate_vector_store_readiness(analysis: Dict[str, Any]) -> Dict[str, Any]:
    """Évalue la pertinence pour un vector store"""
    score = 0
    max_score = 10
    recommendations = []
    
    # Critère 1: Taille des chunks (2 points)
    optimal_ratio = analysis['vector_store_quality']['optimal_chunk_size_ratio']
    if optimal_ratio >= 0.8:
        score += 2
    elif optimal_ratio >= 0.6:
        score += 1.5
    elif optimal_ratio >= 0.4:
        score += 1
    else:
        recommendations.append("Améliorer la taille des chunks (optimal: 500-1500 caractères)")
    
    # Critère 2: Variété du contenu (2 points)
    content_variety = analysis['vector_store_quality']['content_variety']
    if content_variety >= 0.9:
        score += 2
    elif content_variety >= 0.7:
        score += 1.5
    elif content_variety >= 0.5:
        score += 1
    else:
        recommendations.append("Améliorer la diversité du contenu dans les chunks")
    
    # Critère 3: Richesse des métadonnées (2 points)
    metadata_richness = analysis['vector_store_quality']['metadata_richness']
    if metadata_richness >= 0.8:
        score += 2
    elif metadata_richness >= 0.6:
        score += 1.5
    elif metadata_richness >= 0.4:
        score += 1
    else:
        recommendations.append("Enrichir les métadonnées des chunks")
    
    # Critère 4: Performance de traitement (2 points)
    chars_per_ms = analysis['performance']['chars_per_ms']
    if chars_per_ms >= 1000:
        score += 2
    elif chars_per_ms >= 500:
        score += 1.5
    elif chars_per_ms >= 100:
        score += 1
    else:
        recommendations.append("Optimiser les performances de traitement")
    
    # Critère 5: Consistance des chunks (2 points)
    size_std_dev = analysis['chunks_analysis']['size_std_dev']
    avg_size = analysis['chunks_analysis']['avg_size']
    consistency_ratio = 1 - (size_std_dev / avg_size) if avg_size > 0 else 0
    
    if consistency_ratio >= 0.8:
        score += 2
    elif consistency_ratio >= 0.6:
        score += 1.5
    elif consistency_ratio >= 0.4:
        score += 1
    else:
        recommendations.append("Améliorer la consistance de la taille des chunks")
    
    # Classification qualitative
    if score >= 8:
        quality_level = "Excellent"
        color = "🟢"
    elif score >= 6:
        quality_level = "Bon"
        color = "🟡"
    elif score >= 4:
        quality_level = "Acceptable"
        color = "🟠"
    else:
        quality_level = "À améliorer"
        color = "🔴"
    
    return {
        'score': score,
        'max_score': max_score,
        'percentage': (score / max_score) * 100,
        'quality_level': quality_level,
        'color': color,
        'recommendations': recommendations
    }

def main():
    """Fonction principale d'analyse"""
    results_dir = Path("test_results")
    
    if not results_dir.exists():
        print("❌ Dossier test_results non trouvé")
        return
    
    print("📊 ÉVALUATION DE LA QUALITÉ DES RÉSULTATS DOC_LOADER")
    print("=" * 60)
    
    total_analysis = []
    
    for json_file in results_dir.glob("*.json"):
        print(f"\n📄 Analyse de {json_file.name}")
        print("-" * 40)
        
        try:
            analysis = analyze_document_output(json_file)
            evaluation = evaluate_vector_store_readiness(analysis)
            
            total_analysis.append({
                'file': json_file.name,
                'analysis': analysis,
                'evaluation': evaluation
            })
            
            # Affichage des résultats
            print(f"📋 Type: {analysis['document_type']}")
            print(f"📏 Taille fichier: {analysis['file_size']:,} bytes")
            print(f"🧩 Chunks générés: {analysis['total_chunks']:,}")
            print(f"⏱️  Temps de traitement: {analysis['processing_time_ms']} ms")
            print(f"⚡ Performance: {analysis['performance']['chars_per_ms']:.1f} chars/ms")
            
            print(f"\n📊 Statistiques des chunks:")
            print(f"   • Taille moyenne: {analysis['chunks_analysis']['avg_size']:.1f} caractères")
            print(f"   • Taille médiane: {analysis['chunks_analysis']['median_size']:.1f} caractères")
            print(f"   • Écart-type: {analysis['chunks_analysis']['size_std_dev']:.1f}")
            print(f"   • Mots par chunk: {analysis['chunks_analysis']['avg_word_count']:.1f}")
            
            print(f"\n🎯 Qualité Vector Store:")
            print(f"   • Chunks optimaux: {analysis['vector_store_quality']['optimal_chunk_size_ratio']:.1%}")
            print(f"   • Variété contenu: {analysis['vector_store_quality']['content_variety']:.1%}")
            print(f"   • Richesse métadonnées: {analysis['vector_store_quality']['metadata_richness']:.1%}")
            
            print(f"\n{evaluation['color']} ÉVALUATION GLOBALE: {evaluation['quality_level']}")
            print(f"   Score: {evaluation['score']:.1f}/{evaluation['max_score']} ({evaluation['percentage']:.1f}%)")
            
            if evaluation['recommendations']:
                print(f"\n💡 Recommandations:")
                for rec in evaluation['recommendations']:
                    print(f"   • {rec}")
        
        except Exception as e:
            print(f"❌ Erreur lors de l'analyse de {json_file.name}: {e}")
    
    # Résumé global
    if total_analysis:
        print(f"\n{'='*60}")
        print("📈 RÉSUMÉ GLOBAL")
        print(f"{'='*60}")
        
        avg_score = statistics.mean([item['evaluation']['score'] for item in total_analysis])
        avg_performance = statistics.mean([item['analysis']['performance']['chars_per_ms'] for item in total_analysis])
        total_chunks = sum([item['analysis']['total_chunks'] for item in total_analysis])
        total_processing_time = sum([item['analysis']['processing_time_ms'] for item in total_analysis])
        
        print(f"📊 Score moyen: {avg_score:.1f}/10 ({(avg_score/10)*100:.1f}%)")
        print(f"⚡ Performance moyenne: {avg_performance:.1f} chars/ms")
        print(f"🧩 Total chunks générés: {total_chunks:,}")
        print(f"⏱️  Temps total de traitement: {total_processing_time:,} ms")
        
        # Meilleur et moins bon
        best = max(total_analysis, key=lambda x: x['evaluation']['score'])
        worst = min(total_analysis, key=lambda x: x['evaluation']['score'])
        
        print(f"\n🏆 Meilleur résultat: {best['file']} ({best['evaluation']['score']:.1f}/10)")
        print(f"⚠️  À améliorer: {worst['file']} ({worst['evaluation']['score']:.1f}/10)")
        
        # Recommandations globales
        all_recommendations = []
        for item in total_analysis:
            all_recommendations.extend(item['evaluation']['recommendations'])
        
        if all_recommendations:
            print(f"\n💡 Recommandations principales:")
            from collections import Counter
            common_recommendations = Counter(all_recommendations).most_common(3)
            for rec, count in common_recommendations:
                print(f"   • {rec} (concerné: {count} fichier{'s' if count > 1 else ''})")
    
    print(f"\n✅ Analyse terminée!")

if __name__ == "__main__":
    main()
