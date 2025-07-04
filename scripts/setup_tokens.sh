#!/bin/bash

# Script d'exemple pour configurer les tokens de publication
# Ce script montre comment configurer les variables d'environnement pour publish.sh

echo "🔐 Configuration des tokens de publication pour doc_loader"
echo

echo "📋 Étapes à suivre :"
echo

echo "1️⃣  Stockage sécurisé des tokens avec smart-locker"
echo "   # Stocker le token PyPI"
echo "   smart-locker encrypt --name PYPI_TOKEN --value 'pyp-your-token-here'"
echo
echo "   # Stocker le token crates.io"
echo "   smart-locker encrypt --name CRATES_IO_TOKEN --value 'your-crates-token-here'"
echo

echo "2️⃣  Configuration des variables d'environnement (optionnel)"
echo "   # Si vous utilisez des noms différents dans smart-locker"
echo "   export PYPI_TOKEN_NAME='your_pypi_secret_name'"
echo "   export CRATES_TOKEN_NAME='your_crates_secret_name'"
echo

echo "3️⃣  Vérification des secrets stockés"
echo "   smart-locker list"
echo

echo "4️⃣  Test de récupération"
echo "   smart-locker decrypt --name PYPI_TOKEN"
echo "   smart-locker decrypt --name CRATES_IO_TOKEN"
echo

echo "5️⃣  Vérification avec le script"
echo "   ./scripts/publish.sh --check"
echo

echo "📚 Génération des tokens :"
echo

echo "🐍 PyPI Token :"
echo "   1. Aller sur https://pypi.org/manage/account/token/"
echo "   2. Créer un nouveau token API"
echo "   3. Nommer le token (ex: 'doc_loader_publishing')"
echo "   4. Sélectionner le scope approprié"
echo "   5. Copier le token (format: pyp-...)"
echo

echo "🦀 Crates.io Token :"
echo "   1. Aller sur https://crates.io/settings/tokens"
echo "   2. Créer un nouveau token"
echo "   3. Nommer le token (ex: 'doc_loader_publishing')"
echo "   4. Copier le token"
echo

echo "⚠️  Sécurité importante :"
echo "   - Ne jamais committer les tokens en dur dans le code"
echo "   - Utiliser smart-locker avec les noms PYPI_TOKEN et CRATES_IO_TOKEN"
echo "   - Révoquer et régénérer les tokens si nécessaire"
echo "   - Les noms par défaut: PYPI_TOKEN et CRATES_IO_TOKEN"
echo

echo "🚀 Une fois configuré, utilisez :"
echo "   ./scripts/publish.sh          # Menu interactif"
echo "   ./scripts/publish.sh --all    # Publication complète"
echo "   ./scripts/publish.sh --dry-run # Test sans publication"
echo

echo "💡 Configuration personnalisée :"
echo "   # Si vos secrets ont des noms différents"
echo "   export PYPI_TOKEN_NAME='mon_token_pypi'"
echo "   export CRATES_TOKEN_NAME='mon_token_crates'"
