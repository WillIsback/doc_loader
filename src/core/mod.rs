use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Structure universelle pour un chunk de document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentChunk {
    /// Identifiant unique du chunk
    pub id: String,
    /// Contenu textuel du chunk
    pub content: String,
    /// Index du chunk dans le document
    pub chunk_index: usize,
    /// Position du chunk (page, ligne, etc.)
    pub position: ChunkPosition,
    /// Métadonnées du chunk
    pub metadata: ChunkMetadata,
}

/// Position d'un chunk dans le document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkPosition {
    /// Numéro de page (si applicable)
    pub page: Option<u32>,
    /// Numéro de ligne (si applicable)
    pub line: Option<u32>,
    /// Position de début dans le texte
    pub start_offset: Option<usize>,
    /// Position de fin dans le texte
    pub end_offset: Option<usize>,
}

/// Métadonnées d'un chunk
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkMetadata {
    /// Taille du chunk en caractères
    pub size: usize,
    /// Langue détectée (optionnel)
    pub language: Option<String>,
    /// Score de confiance pour l'extraction
    pub confidence: Option<f32>,
    /// Informations additionnelles spécifiques au format
    pub format_specific: Option<serde_json::Value>,
}

/// Structure universelle de sortie pour tous les types de documents
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalOutput {
    /// Métadonnées du document
    pub document_metadata: DocumentMetadata,
    /// Liste des chunks extraits
    pub chunks: Vec<DocumentChunk>,
    /// Informations sur le traitement
    pub processing_info: ProcessingInfo,
}

/// Métadonnées du document source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentMetadata {
    /// Nom du fichier
    pub filename: String,
    /// Chemin complet du fichier
    pub filepath: String,
    /// Type de document (PDF, TXT, JSON, CSV, DOCX)
    pub document_type: DocumentType,
    /// Taille du fichier en bytes
    pub file_size: u64,
    /// Date de création du fichier
    pub created_at: Option<DateTime<Utc>>,
    /// Date de modification du fichier
    pub modified_at: Option<DateTime<Utc>>,
    /// Titre du document (si disponible)
    pub title: Option<String>,
    /// Auteur du document (si disponible)
    pub author: Option<String>,
    /// Métadonnées spécifiques au format
    pub format_metadata: serde_json::Value,
}

/// Types de documents supportés
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DocumentType {
    PDF,
    TXT,
    JSON,
    CSV,
    DOCX,
}

/// Informations sur le traitement effectué
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingInfo {
    /// Processeur utilisé
    pub processor: String,
    /// Version du processeur
    pub processor_version: String,
    /// Date et heure du traitement
    pub processed_at: DateTime<Utc>,
    /// Temps de traitement en millisecondes
    pub processing_time_ms: u64,
    /// Nombre total de chunks générés
    pub total_chunks: usize,
    /// Taille totale du contenu extrait
    pub total_content_size: usize,
    /// Paramètres de traitement utilisés
    pub processing_params: ProcessingParams,
}

/// Paramètres de traitement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingParams {
    /// Taille maximale des chunks
    pub max_chunk_size: usize,
    /// Chevauchement entre chunks
    pub chunk_overlap: usize,
    /// Nettoyage du texte activé
    pub text_cleaning: bool,
    /// Détection de langue activée
    pub language_detection: bool,
    /// Paramètres spécifiques au format
    pub format_specific: serde_json::Value,
}

impl Default for ProcessingParams {
    fn default() -> Self {
        Self {
            max_chunk_size: 1000,
            chunk_overlap: 100,
            text_cleaning: true,
            language_detection: false,
            format_specific: serde_json::Value::Null,
        }
    }
}

impl ProcessingParams {
    pub fn with_chunk_size(mut self, size: usize) -> Self {
        self.max_chunk_size = size;
        self
    }

    pub fn with_chunk_overlap(mut self, overlap: usize) -> Self {
        self.chunk_overlap = overlap;
        self
    }

    pub fn with_text_cleaning(mut self, enabled: bool) -> Self {
        self.text_cleaning = enabled;
        self
    }

    pub fn with_language_detection(mut self, enabled: bool) -> Self {
        self.language_detection = enabled;
        self
    }

    pub fn with_format_specific(mut self, metadata: serde_json::Value) -> Self {
        self.format_specific = metadata;
        self
    }
}

impl DocumentType {
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "pdf" => Some(DocumentType::PDF),
            "txt" => Some(DocumentType::TXT),
            "json" => Some(DocumentType::JSON),
            "csv" => Some(DocumentType::CSV),
            "docx" => Some(DocumentType::DOCX),
            _ => None,
        }
    }

    pub fn to_string(&self) -> &'static str {
        match self {
            DocumentType::PDF => "PDF",
            DocumentType::TXT => "TXT",
            DocumentType::JSON => "JSON",
            DocumentType::CSV => "CSV",
            DocumentType::DOCX => "DOCX",
        }
    }
}
