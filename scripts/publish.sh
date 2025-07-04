#!/bin/bash

# Script de publication automatisée pour doc_loader
# Supporte PyPI et crates.io avec gestion sécurisée des tokens

set -e  # Arrêter en cas d'erreur

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
TEMP_VENV_NAME="publish_venv"
TEMP_VENV_PATH="$PROJECT_DIR/$TEMP_VENV_NAME"

# Couleurs pour l'affichage
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Variables globales pour les tokens
PYPI_TOKEN=""
CRATES_TOKEN=""

# Fonction d'affichage coloré
print_colored() {
    local color=$1
    local message=$2
    echo -e "${color}${message}${NC}"
}

print_success() {
    print_colored "$GREEN" "✅ $1"
}

print_error() {
    print_colored "$RED" "❌ $1"
}

print_warning() {
    print_colored "$YELLOW" "⚠️  $1"
}

print_info() {
    print_colored "$BLUE" "ℹ️  $1"
}

print_step() {
    print_colored "$PURPLE" "🔧 $1"
}

# Fonction pour afficher le logo
show_banner() {
    print_colored "$CYAN" "
╔══════════════════════════════════════════════════════════════╗
║                    🚀 DOC LOADER PUBLISHER                   ║
║              Script de publication automatisée               ║
╚══════════════════════════════════════════════════════════════╝
"
}

# Fonction pour afficher l'aide
show_help() {
    echo "
Usage: $0 [OPTION]

Options:
    -h, --help          Afficher cette aide
    -1, --pypi          Publier uniquement sur PyPI
    -2, --crates        Publier uniquement sur crates.io
    -3, --all           Publier sur les deux plateformes
    -c, --check         Vérifier la configuration sans publier
    --dry-run           Mode test (sans publication réelle)
    --force-dirty       Forcer la publication même avec des changements non-committés
    --skip-git-check    Ignorer complètement les vérifications Git

Variables d'environnement optionnelles:
    PYPI_TOKEN_NAME         Nom du secret PyPI dans smart-locker (défaut: PYPI_TOKEN)
    CRATES_TOKEN_NAME       Nom du secret crates.io dans smart-locker (défaut: CRATES_IO_TOKEN)

Exemples:
    $0                      # Mode interactif (menu)
    $0 --pypi              # Publication PyPI uniquement
    $0 --all               # Publication sur les deux plateformes
    $0 --check             # Vérification de la configuration
    $0 --all --force-dirty # Publication forcée même avec changements non-committés
"
}

# Fonction pour vérifier les prérequis
check_requirements() {
    print_step "Vérification des prérequis..."
    
    local missing_tools=()
    
    # Vérifier les outils requis
    if ! command -v smart-locker &> /dev/null; then
        missing_tools+=("smart-locker")
    fi
    
    if ! command -v python3 &> /dev/null; then
        missing_tools+=("python3")
    fi
    
    if ! command -v cargo &> /dev/null; then
        missing_tools+=("cargo")
    fi
    
    if ! command -v git &> /dev/null; then
        missing_tools+=("git")
    fi
    
    if [ ${#missing_tools[@]} -ne 0 ]; then
        print_error "Outils manquants: ${missing_tools[*]}"
        print_info "Veuillez installer les outils manquants avant de continuer"
        exit 1
    fi
    
    # Vérifier que nous sommes dans le bon répertoire
    if [ ! -f "$PROJECT_DIR/Cargo.toml" ] || [ ! -f "$PROJECT_DIR/pyproject.toml" ]; then
        print_error "Fichiers de configuration manquants (Cargo.toml ou pyproject.toml)"
        print_info "Assurez-vous d'être dans le répertoire racine du projet"
        exit 1
    fi
    
    # Vérifier que nous sommes dans un dépôt Git
    if ! git rev-parse --git-dir > /dev/null 2>&1; then
        print_error "Ce n'est pas un dépôt Git valide"
        print_info "Assurez-vous d'être dans un projet Git initialisé"
        exit 1
    fi
    
    print_success "Tous les prérequis sont satisfaits"
}

# Fonction pour vérifier l'état Git du projet
check_git_status() {
    print_step "Vérification de l'état Git du projet..."
    
    cd "$PROJECT_DIR"
    
    # Vérifier s'il y a des changements non-committés
    if ! git diff-index --quiet HEAD --; then
        print_warning "Attention: Des fichiers modifiés non-committés détectés"
        print_info "Fichiers concernés:"
        git status --porcelain | head -10
        echo
        
        print_info "Recommandations:"
        print_info "1. Committez vos changements avant publication"
        print_info "2. Ou continuez avec --allow-dirty (non recommandé)"
        echo
        
        read -p "Voulez-vous committer automatiquement tous les changements? (y/N): " commit_choice
        if [[ "$commit_choice" =~ ^[Yy]$ ]]; then
            print_info "Commit automatique en cours..."
            git add .
            read -p "Message de commit (ou Entrée pour un message par défaut): " commit_msg
            if [ -z "$commit_msg" ]; then
                commit_msg="chore: pre-publication commit"
            fi
            git commit -m "$commit_msg"
            print_success "Changements committés avec succès"
        else
            print_warning "Publication avec état Git non-propre (--allow-dirty sera utilisé)"
        fi
    else
        print_success "État Git propre - prêt pour publication"
    fi
}

# Fonction pour récupérer les tokens via smart-locker
get_tokens() {
    print_step "Récupération des tokens de publication..."
    
    # Récupération du token PyPI
    if [ -n "$PYPI_TOKEN_NAME" ]; then
        print_info "Décryptage du token PyPI..."
        PYPI_TOKEN=$(smart-locker decrypt --name "$PYPI_TOKEN_NAME" 2>/dev/null || echo "")
        if [ -z "$PYPI_TOKEN" ]; then
            print_error "Impossible de décrypter le token PyPI avec le nom: $PYPI_TOKEN_NAME"
            return 1
        fi
        print_success "Token PyPI récupéré"
    else
        # Fallback vers le nom par défaut
        print_info "Tentative de récupération du token PyPI avec le nom par défaut..."
        PYPI_TOKEN=$(smart-locker decrypt --name "PYPI_TOKEN" 2>/dev/null || echo "")
        if [ -z "$PYPI_TOKEN" ]; then
            print_warning "Token PyPI non trouvé (nom testé: PYPI_TOKEN)"
            print_info "Définissez PYPI_TOKEN_NAME avec le bon nom ou utilisez 'PYPI_TOKEN' dans smart-locker"
        else
            print_success "Token PyPI récupéré avec le nom par défaut"
        fi
    fi
    
    # Récupération du token crates.io
    if [ -n "$CRATES_TOKEN_NAME" ]; then
        print_info "Décryptage du token crates.io..."
        CRATES_TOKEN=$(smart-locker decrypt --name "$CRATES_TOKEN_NAME" 2>/dev/null || echo "")
        if [ -z "$CRATES_TOKEN" ]; then
            print_error "Impossible de décrypter le token crates.io avec le nom: $CRATES_TOKEN_NAME"
            return 1
        fi
        print_success "Token crates.io récupéré"
    else
        # Fallback vers le nom par défaut
        print_info "Tentative de récupération du token crates.io avec le nom par défaut..."
        CRATES_TOKEN=$(smart-locker decrypt --name "CRATES_IO_TOKEN" 2>/dev/null || echo "")
        if [ -z "$CRATES_TOKEN" ]; then
            print_warning "Token crates.io non trouvé (nom testé: CRATES_IO_TOKEN)"
            print_info "Définissez CRATES_TOKEN_NAME avec le bon nom ou utilisez 'CRATES_IO_TOKEN' dans smart-locker"
        else
            print_success "Token crates.io récupéré avec le nom par défaut"
        fi
    fi
}

# Fonction pour créer l'environnement virtuel temporaire
create_temp_venv() {
    print_step "Création de l'environnement virtuel temporaire..."
    
    # Supprimer l'ancien venv s'il existe
    if [ -d "$TEMP_VENV_PATH" ]; then
        print_info "Suppression de l'ancien environnement virtuel..."
        rm -rf "$TEMP_VENV_PATH"
    fi
    
    # Créer le nouvel environnement
    python3 -m venv "$TEMP_VENV_PATH"
    print_success "Environnement virtuel créé: $TEMP_VENV_NAME"
    
    # Activer l'environnement
    source "$TEMP_VENV_PATH/bin/activate"
    
    # Mettre à jour pip et installer les dépendances
    print_info "Installation des dépendances de publication..."
    pip install --upgrade pip
    pip install maturin twine build wheel
    
    print_success "Environnement virtuel configuré"
}

# Fonction pour nettoyer l'environnement temporaire
cleanup_temp_venv() {
    print_step "Nettoyage de l'environnement temporaire..."
    
    # Désactiver l'environnement virtuel
    if [ -n "$VIRTUAL_ENV" ]; then
        deactivate 2>/dev/null || true
    fi
    
    # Supprimer le répertoire
    if [ -d "$TEMP_VENV_PATH" ]; then
        rm -rf "$TEMP_VENV_PATH"
        print_success "Environnement temporaire supprimé"
    fi
}

# Fonction pour publier sur PyPI
publish_pypi() {
    local dry_run=${1:-false}
    print_step "Publication sur PyPI..."
    
    if [ -z "$PYPI_TOKEN" ]; then
        print_error "Token PyPI non disponible"
        return 1
    fi
    
    cd "$PROJECT_DIR"
    
    # Nettoyer les anciens artefacts
    print_info "Nettoyage des anciens artefacts Python..."
    rm -rf dist/ build/ *.egg-info/ target/wheels/
    
    # Construire le package avec maturin
    print_info "Construction du package Python avec maturin..."
    maturin build --release --features python
    
    if [ "$dry_run" = "true" ]; then
        print_warning "Mode dry-run: publication PyPI simulée"
        return 0
    fi
    
    # Publication avec twine
    print_info "Publication sur PyPI..."
    export TWINE_USERNAME="__token__"
    export TWINE_PASSWORD="$PYPI_TOKEN"
    
    python -m twine upload target/wheels/* --verbose
    
    # Nettoyage des variables sensibles
    unset TWINE_USERNAME
    unset TWINE_PASSWORD
    
    print_success "Package publié sur PyPI avec succès"
}

# Fonction pour publier sur crates.io
publish_crates() {
    local dry_run=${1:-false}
    print_step "Publication sur crates.io..."
    
    if [ -z "$CRATES_TOKEN" ]; then
        print_error "Token crates.io non disponible"
        return 1
    fi
    
    cd "$PROJECT_DIR"
    
    # Vérifier l'état Git avant publication
    print_info "Vérification de l'état Git..."
    if ! git diff-index --quiet HEAD --; then
        if [ "$FORCE_DIRTY" = "true" ]; then
            print_warning "État Git non-propre détecté mais ignoré (--force-dirty)"
            CARGO_PUBLISH_FLAGS="--allow-dirty"
        else
            print_warning "Des fichiers modifiés non-committés détectés"
            print_info "Fichiers modifiés:"
            git status --porcelain | head -5
            
            # Proposer de committer automatiquement ou utiliser --allow-dirty
            read -p "Voulez-vous committer automatiquement ces changements? (y/N): " auto_commit
            if [[ "$auto_commit" =~ ^[Yy]$ ]]; then
                print_info "Commit automatique des changements..."
                git add .
                git commit -m "fix: auto-commit before crates.io publication"
                print_success "Changements committés automatiquement"
                CARGO_PUBLISH_FLAGS=""
            else
                print_warning "Utilisation du flag --allow-dirty pour la publication"
                CARGO_PUBLISH_FLAGS="--allow-dirty"
            fi
        fi
    else
        print_success "État Git propre"
        CARGO_PUBLISH_FLAGS=""
    fi
    
    # Nettoyer les anciens artefacts
    print_info "Nettoyage des anciens artefacts Rust..."
    rm -rf target/package/
    
    # Configurer le token cargo
    export CARGO_REGISTRY_TOKEN="$CRATES_TOKEN"
    
    if [ "$dry_run" = "true" ]; then
        print_warning "Mode dry-run: publication crates.io simulée"
        print_info "Test de packaging..."
        cargo package --list $CARGO_PUBLISH_FLAGS
        unset CARGO_REGISTRY_TOKEN
        return 0
    fi
    
    # Tentative de packaging d'abord pour détecter les erreurs
    print_info "Test de packaging avant publication..."
    if ! cargo package $CARGO_PUBLISH_FLAGS; then
        print_error "Échec du packaging. Vérifiez les erreurs ci-dessus."
        unset CARGO_REGISTRY_TOKEN
        return 1
    fi
    
    # Publication
    print_info "Publication sur crates.io..."
    if cargo publish $CARGO_PUBLISH_FLAGS; then
        print_success "Package publié sur crates.io avec succès"
    else
        print_error "Échec de la publication sur crates.io"
        print_info "Vérifiez les logs ci-dessus pour plus de détails"
        unset CARGO_REGISTRY_TOKEN
        return 1
    fi
    
    # Nettoyage des variables sensibles
    unset CARGO_REGISTRY_TOKEN
}

# Fonction pour vérifier les packages publiés
verify_publication() {
    local platform=$1
    print_step "Vérification de la publication sur $platform..."
    
    case $platform in
        "pypi")
            print_info "Vérification PyPI..."
            sleep 15  # Attendre la propagation (augmenté)
            
            # Test d'installation dans un venv temporaire
            local verify_venv="$PROJECT_DIR/verify_venv"
            python3 -m venv "$verify_venv"
            source "$verify_venv/bin/activate"
            
            # Tentative d'installation avec retry
            local max_attempts=3
            local attempt=1
            while [ $attempt -le $max_attempts ]; do
                print_info "Tentative d'installation $attempt/$max_attempts..."
                if pip install doc-loader --no-cache-dir --upgrade; then
                    if python -c "import doc_loader; print('✅ Import réussi')"; then
                        print_success "Package PyPI vérifié avec succès"
                        break
                    fi
                else
                    if [ $attempt -eq $max_attempts ]; then
                        print_warning "Échec de vérification PyPI après $max_attempts tentatives"
                        print_info "Le package pourrait nécessiter plus de temps pour être disponible"
                    else
                        print_info "Nouvelle tentative dans 10 secondes..."
                        sleep 10
                    fi
                fi
                ((attempt++))
            done
            
            deactivate
            rm -rf "$verify_venv"
            ;;
        "crates")
            print_info "Vérification crates.io..."
            sleep 15  # Attendre la propagation (augmenté)
            
            # Vérification avec retry
            local max_attempts=3
            local attempt=1
            while [ $attempt -le $max_attempts ]; do
                print_info "Vérification crates.io - tentative $attempt/$max_attempts..."
                if cargo search doc_loader | grep -q "doc_loader.*0\.2\.0"; then
                    print_success "Package crates.io vérifié avec succès"
                    cargo search doc_loader | head -1
                    break
                else
                    if [ $attempt -eq $max_attempts ]; then
                        print_warning "Échec de vérification crates.io après $max_attempts tentatives"
                        print_info "Le package pourrait nécessiter plus de temps pour être indexé"
                    else
                        print_info "Nouvelle tentative dans 10 secondes..."
                        sleep 10
                    fi
                fi
                ((attempt++))
            done
            ;;
    esac
}

# Fonction pour afficher le menu interactif
show_menu() {
    echo
    print_colored "$CYAN" "📋 Choisissez une option de publication:"
    echo "   1) PyPI uniquement"
    echo "   2) crates.io uniquement"
    echo "   3) Les deux plateformes"
    echo "   4) Vérification de la configuration"
    echo "   5) Mode dry-run (test)"
    echo "   6) Aide"
    echo "   0) Quitter"
    echo
}

# Fonction principale pour gérer le menu
handle_menu_choice() {
    local choice=$1
    local dry_run=${2:-false}
    
    case $choice in
        1|"pypi")
            if [ -z "$PYPI_TOKEN" ]; then
                print_error "Token PyPI non disponible. Vérifiez la configuration smart-locker"
                print_info "Tokens disponibles: $(smart-locker list 2>/dev/null | grep -o '[A-Z_]*TOKEN' | tr '\n' ' ' || echo 'Erreur smart-locker')"
                return 1
            fi
            create_temp_venv
            publish_pypi "$dry_run"
            if [ "$dry_run" = "false" ]; then
                verify_publication "pypi"
            fi
            cleanup_temp_venv
            ;;
        2|"crates")
            if [ -z "$CRATES_TOKEN" ]; then
                print_error "Token crates.io non disponible. Vérifiez la configuration smart-locker"
                print_info "Tokens disponibles: $(smart-locker list 2>/dev/null | grep -o '[A-Z_]*TOKEN' | tr '\n' ' ' || echo 'Erreur smart-locker')"
                return 1
            fi
            publish_crates "$dry_run"
            if [ "$dry_run" = "false" ]; then
                verify_publication "crates"
            fi
            ;;
        3|"all")
            if [ -z "$PYPI_TOKEN" ] || [ -z "$CRATES_TOKEN" ]; then
                print_error "Tokens manquants. Vérifiez la configuration smart-locker"
                print_info "Tokens disponibles: $(smart-locker list 2>/dev/null | grep -o '[A-Z_]*TOKEN' | tr '\n' ' ' || echo 'Erreur smart-locker')"
                return 1
            fi
            create_temp_venv
            publish_pypi "$dry_run"
            cleanup_temp_venv
            publish_crates "$dry_run"
            if [ "$dry_run" = "false" ]; then
                verify_publication "pypi"
                verify_publication "crates"
            fi
            ;;
        4|"check")
            print_step "Vérification de la configuration..."
            print_info "Token PyPI: $([ -n "$PYPI_TOKEN" ] && echo "✅ Disponible" || echo "❌ Manquant")"
            print_info "Token crates.io: $([ -n "$CRATES_TOKEN" ] && echo "✅ Disponible" || echo "❌ Manquant")"
            print_info "Noms des secrets smart-locker:"
            print_info "  - PyPI: ${PYPI_TOKEN_NAME:-PYPI_TOKEN (par défaut)}"
            print_info "  - Crates.io: ${CRATES_TOKEN_NAME:-CRATES_IO_TOKEN (par défaut)}"
            print_info "Fichiers de configuration:"
            print_info "  - Cargo.toml: $([ -f "$PROJECT_DIR/Cargo.toml" ] && echo "✅" || echo "❌")"
            print_info "  - pyproject.toml: $([ -f "$PROJECT_DIR/pyproject.toml" ] && echo "✅" || echo "❌")"
            print_info "Secrets disponibles dans smart-locker:"
            smart-locker list 2>/dev/null | grep -E "(PYPI|CRATES)" || print_warning "Aucun token PyPI/Crates trouvé dans smart-locker"
            ;;
        5|"dry-run")
            print_warning "Mode dry-run activé"
            show_menu
            read -p "Votre choix (1-3): " sub_choice
            handle_menu_choice "$sub_choice" true
            ;;
        6|"help")
            show_help
            ;;
        0|"quit"|"exit")
            print_info "Au revoir!"
            exit 0
            ;;
        *)
            print_error "Option invalide: $choice"
            return 1
            ;;
    esac
}

# Fonction principale
main() {
    show_banner
    
    # Parser les arguments de ligne de commande
    local mode="interactive"
    local dry_run=false
    local force_dirty=false
    local skip_git_check=false
    
    while [[ $# -gt 0 ]]; do
        case $1 in
            -h|--help)
                show_help
                exit 0
                ;;
            -1|--pypi)
                mode="pypi"
                shift
                ;;
            -2|--crates)
                mode="crates"
                shift
                ;;
            -3|--all)
                mode="all"
                shift
                ;;
            -c|--check)
                mode="check"
                shift
                ;;
            --dry-run)
                dry_run=true
                shift
                ;;
            --force-dirty)
                force_dirty=true
                shift
                ;;
            --skip-git-check)
                skip_git_check=true
                shift
                ;;
            *)
                print_error "Option inconnue: $1"
                show_help
                exit 1
                ;;
        esac
    done
    
    # Vérifications initiales
    check_requirements
    if [ "$skip_git_check" = "false" ]; then
        check_git_status
    else
        print_warning "Vérifications Git ignorées (--skip-git-check)"
    fi
    get_tokens
    
    # Exporter les flags pour les fonctions de publication
    export FORCE_DIRTY="$force_dirty"
    
    # Gestion des erreurs avec nettoyage
    trap cleanup_temp_venv EXIT ERR
    
    # Exécution selon le mode
    if [ "$mode" = "interactive" ]; then
        while true; do
            show_menu
            read -p "Votre choix (0-6): " choice
            echo
            
            if ! handle_menu_choice "$choice" "$dry_run"; then
                print_warning "Opération échouée ou annulée"
            fi
            
            if [ "$choice" = "0" ] || [ "$choice" = "quit" ] || [ "$choice" = "exit" ]; then
                break
            fi
            
            echo
            read -p "Appuyez sur Entrée pour continuer..."
        done
    else
        handle_menu_choice "$mode" "$dry_run"
    fi
    
    print_success "Script terminé"
}

# Point d'entrée
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi
