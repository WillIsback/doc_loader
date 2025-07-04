use regex::Regex;
use unicode_segmentation::UnicodeSegmentation;

/// Nettoie le texte en supprimant les caractères indésirables
pub fn clean_text(text: &str) -> String {
    let mut cleaned = text.to_string();
    
    // Supprimer les caractères de contrôle
    cleaned = cleaned.chars()
        .filter(|c| !c.is_control() || *c == '\n' || *c == '\t')
        .collect();
    
    // Normaliser les espaces
    let whitespace_regex = Regex::new(r"\s+").unwrap();
    cleaned = whitespace_regex.replace_all(&cleaned, " ").to_string();
    
    // Supprimer les espaces en début et fin
    cleaned.trim().to_string()
}

/// Découpe le texte en chunks de taille maximale avec chevauchement
pub fn chunk_text(text: &str, max_size: usize, overlap: usize) -> Vec<String> {
    if text.len() <= max_size {
        return vec![text.to_string()];
    }
    
    let mut chunks = Vec::new();
    let mut start = 0;
    
    while start < text.len() {
        let end = (start + max_size).min(text.len());
        let chunk = &text[start..end];
        
        // Essayer de couper à un espace pour éviter de couper les mots
        let final_chunk = if end < text.len() {
            if let Some(last_space) = chunk.rfind(' ') {
                &chunk[..last_space]
            } else {
                chunk
            }
        } else {
            chunk
        };
        
        chunks.push(final_chunk.to_string());
        
        // Calculer la prochaine position de début avec chevauchement
        if end >= text.len() {
            break;
        }
        
        start = if final_chunk.len() > overlap {
            start + final_chunk.len() - overlap
        } else {
            start + final_chunk.len()
        };
    }
    
    chunks
}

/// Compte le nombre de mots dans un texte
pub fn count_words(text: &str) -> usize {
    text.unicode_words().count()
}

/// Estime le nombre de tokens (approximation: 1 token ≈ 0.75 mots)
pub fn estimate_tokens(text: &str) -> usize {
    (count_words(text) as f64 * 1.33) as usize
}

/// Détecte la langue du texte (implémentation basique)
pub fn detect_language(text: &str) -> Option<String> {
    // Implémentation basique basée sur des mots-clés communs
    let text_lower = text.to_lowercase();
    
    // Français
    if text_lower.contains("le ") || text_lower.contains("la ") || 
       text_lower.contains("les ") || text_lower.contains("des ") ||
       text_lower.contains("pour ") || text_lower.contains("avec ") {
        return Some("fr".to_string());
    }
    
    // Anglais
    if text_lower.contains("the ") || text_lower.contains("and ") || 
       text_lower.contains("for ") || text_lower.contains("with ") ||
       text_lower.contains("from ") || text_lower.contains("this ") {
        return Some("en".to_string());
    }
    
    None
}

/// Normalise les retours à la ligne
pub fn normalize_line_breaks(text: &str) -> String {
    text.replace("\r\n", "\n").replace('\r', "\n")
}

/// Supprime les lignes vides consécutives
pub fn remove_empty_lines(text: &str) -> String {
    let empty_lines_regex = Regex::new(r"\n\s*\n").unwrap();
    empty_lines_regex.replace_all(text, "\n\n").to_string()
}

/// Extrait les métadonnées basiques d'un texte
pub fn extract_text_metadata(text: &str) -> TextMetadata {
    TextMetadata {
        character_count: text.len(),
        word_count: count_words(text),
        line_count: text.lines().count(),
        estimated_tokens: estimate_tokens(text),
        detected_language: detect_language(text),
    }
}

#[derive(Debug, Clone)]
pub struct TextMetadata {
    pub character_count: usize,
    pub word_count: usize,
    pub line_count: usize,
    pub estimated_tokens: usize,
    pub detected_language: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_text() {
        let dirty_text = "  Hello\t\tWorld  \n\n  ";
        let cleaned = clean_text(dirty_text);
        assert_eq!(cleaned, "Hello World");
    }

    #[test]
    fn test_chunk_text() {
        let text = "Hello world this is a test";
        let chunks = chunk_text(text, 10, 2);
        assert!(!chunks.is_empty());
        assert!(chunks[0].len() <= 10);
    }

    #[test]
    fn test_word_count() {
        let text = "Hello world, this is a test!";
        assert_eq!(count_words(text), 6);
    }

    #[test]
    fn test_language_detection() {
        let french_text = "Bonjour le monde, ceci est un test";
        let english_text = "Hello the world, this is a test";
        
        assert_eq!(detect_language(french_text), Some("fr".to_string()));
        assert_eq!(detect_language(english_text), Some("en".to_string()));
    }
}
