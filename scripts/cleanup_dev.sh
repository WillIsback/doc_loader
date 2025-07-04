#!/bin/bash

# 🧹 Development Environment Cleanup Script
# Removes development-specific files and sensitive data

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

print_status() {
    echo -e "${BLUE}[CLEANUP]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

cleanup_dev_files() {
    print_status "Cleaning development-specific files..."
    
    # Remove temporary environments
    rm -rf temp_env/ publish_venv/ verify_venv/ .venv/ venv/
    
    # Remove output files
    rm -f *.log output_*.json *_output.json
    
    # Remove temporary files
    rm -f *.tmp *.temp
    
    # Clean testing results (keeping structure)
    rm -f testing/results/*.json
    rm -f testing/reports/test_results_detailed.json
    
    print_success "Development files cleaned"
}

cleanup_build_artifacts() {
    print_status "Cleaning build artifacts..."
    
    # Clean Rust build
    cargo clean
    
    # Remove Python build artifacts
    rm -rf dist/ build/ *.egg-info/ target/wheels/
    
    # Remove Python cache
    find . -type d -name "__pycache__" -exec rm -rf {} + 2>/dev/null || true
    find . -name "*.pyc" -delete 2>/dev/null || true
    
    print_success "Build artifacts cleaned"
}

verify_no_sensitive_data() {
    print_status "Verifying no sensitive data..."
    
    # Check for potential sensitive patterns
    sensitive_patterns=(
        "william@"
        "/home/william"
        "Will-Desktop"
        "temp_env"
        "personal"
        "private"
    )
    
    for pattern in "${sensitive_patterns[@]}"; do
        if git ls-files | xargs grep -l "$pattern" 2>/dev/null; then
            print_warning "Found potential sensitive pattern: $pattern"
        fi
    done
    
    print_success "Sensitive data check completed"
}

main() {
    echo "🧹 Development Environment Cleanup"
    echo "=================================="
    
    cleanup_dev_files
    cleanup_build_artifacts
    verify_no_sensitive_data
    
    echo ""
    print_success "Cleanup completed! Ready for GitHub push."
}

main "$@"
