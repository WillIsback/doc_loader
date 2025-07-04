# Scripts de Publication

Ce dossier contient les scripts automatisés pour la publication du projet `doc_loader`.

## 📋 Contenu

- `publish.sh` - Script principal de publication avec menu interactif

## 🚀 publish.sh

Script bash automatisé pour publier le projet sur PyPI et/ou crates.io avec gestion sécurisée des tokens.

### ✨ Fonctionnalités

- **Menu interactif** avec options de publication
- **Gestion sécurisée des tokens** via `smart-locker`
- **Environnement virtuel temporaire** pour les publications Python
- **Vérification automatique** des packages publiés
- **Mode dry-run** pour tester sans publier
- **Nettoyage automatique** des artefacts temporaires
- **Gestion d'erreurs** avec nettoyage en cas d'échec

### 📋 Prérequis

#### Outils requis
```bash
# Outils système
sudo apt install python3 python3-venv git

# Rust et Cargo
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Smart-locker pour la gestion des tokens
# Installation selon votre méthode préférée
```

#### Variables d'environnement

Les tokens sont récupérés automatiquement depuis `smart-locker` avec les noms par défaut :

```bash
# Noms par défaut dans smart-locker
PYPI_TOKEN           # Token PyPI (format: pyp-...)
CRATES_IO_TOKEN      # Token crates.io
```

Si vos secrets ont des noms différents, vous pouvez les spécifier :

```bash
# Configuration personnalisée (optionnel)
export PYPI_TOKEN_NAME="mon_nom_secret_pypi"
export CRATES_TOKEN_NAME="mon_nom_secret_crates"
```

### 🎯 Utilisation

#### Mode interactif (recommandé)
```bash
./scripts/publish.sh
```

#### Options en ligne de commande
```bash
# Publication PyPI uniquement
./scripts/publish.sh --pypi

# Publication crates.io uniquement  
./scripts/publish.sh --crates

# Publication sur les deux plateformes
./scripts/publish.sh --all

# Vérification de la configuration
./scripts/publish.sh --check

# Mode test (sans publication réelle)
./scripts/publish.sh --dry-run

# Aide
./scripts/publish.sh --help
```

### 🎨 Menu interactif

```
╔══════════════════════════════════════════════════════════════╗
║                    🚀 DOC LOADER PUBLISHER                   ║
║              Script de publication automatisée               ║
╚══════════════════════════════════════════════════════════════╝

📋 Choisissez une option de publication:
   1) PyPI uniquement
   2) crates.io uniquement
   3) Les deux plateformes
   4) Vérification de la configuration
   5) Mode dry-run (test)
   6) Aide
   0) Quitter
```

### 🔧 Processus de publication

#### Pour PyPI
1. **Création d'un environnement virtuel temporaire**
2. **Installation des dépendances** (maturin, twine, etc.)
3. **Nettoyage des anciens artefacts**
4. **Construction** du package avec maturin
5. **Publication** avec twine
6. **Vérification** de l'installation
7. **Nettoyage** de l'environnement temporaire

#### Pour crates.io
1. **Nettoyage des anciens artefacts**
2. **Test de packaging** (si dry-run)
3. **Publication** avec cargo publish
4. **Vérification** via cargo search

### 🔒 Sécurité

- **Tokens chiffrés** : Utilisation de `smart-locker` pour stocker les tokens de manière sécurisée
- **Variables temporaires** : Les tokens sont chargés en mémoire uniquement pendant l'exécution
- **Nettoyage automatique** : Suppression des variables sensibles après utilisation
- **Environnements isolés** : Utilisation de venv temporaires pour éviter les conflits

### 📊 Vérifications automatiques

Le script vérifie automatiquement :
- ✅ Présence des outils requis
- ✅ Fichiers de configuration (Cargo.toml, pyproject.toml)
- ✅ Disponibilité des tokens
- ✅ Installation des packages publiés
- ✅ Import/utilisation des modules

### 🛠️ Gestion d'erreurs

- **Arrêt immédiat** en cas d'erreur critique (`set -e`)
- **Nettoyage automatique** via `trap` en cas d'interruption
- **Messages colorés** pour identifier rapidement les problèmes
- **Vérification des prérequis** avant exécution

### 📝 Logs et feedback

Le script fournit un feedback visuel complet :
- 🔧 Étapes en cours
- ✅ Succès
- ❌ Erreurs
- ⚠️ Avertissements  
- ℹ️ Informations

### 🔄 Exemple de workflow complet

```bash
# 1. Stockage des tokens dans smart-locker
smart-locker encrypt --name PYPI_TOKEN --value 'pyp-your-actual-token'
smart-locker encrypt --name CRATES_IO_TOKEN --value 'your-actual-crates-token'

# 2. Vérification de la configuration
./scripts/publish.sh --check

# 3. Test en mode dry-run
./scripts/publish.sh --dry-run

# 4. Publication réelle
./scripts/publish.sh --all
```

### ❗ Notes importantes

- **Incrémentez les versions** dans `Cargo.toml` et `pyproject.toml` avant publication
- **Committez vos changements** git avant de publier
- **Testez en mode dry-run** avant une publication réelle
- **Vérifiez la configuration** avec `--check` si vous rencontrez des problèmes

### 🆘 Dépannage

#### "smart-locker command not found"
```bash
# Installez smart-locker selon votre système
# Ou adaptez le script pour votre gestionnaire de secrets
```

#### "Token non disponible"
```bash
# Vérifiez les variables d'environnement
echo $PYPI_TOKEN_ENCRYPTED
echo $CRATES_TOKEN_ENCRYPTED

# Testez le déchiffrement manuellement
smart-locker decrypt "$PYPI_TOKEN_ENCRYPTED"
```

#### "Erreur de publication"
```bash
# Utilisez le mode dry-run pour identifier le problème
./scripts/publish.sh --dry-run

# Vérifiez les logs détaillés
./scripts/publish.sh --all
```
