#!/bin/bash

# 🔧 GitHub Repository Setup Script
# Configures GitHub repository with security best practices

set -e

# Configuration
REPO_URL="git@github.com:WillIsback/doc_loader.git"
REPO_NAME="doc_loader"
MAIN_BRANCH="main"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to check if we're in the right directory
check_project_directory() {
    if [ ! -f "Cargo.toml" ] || [ ! -f "pyproject.toml" ]; then
        print_error "Not in doc_loader project directory!"
        print_error "Please run this script from the project root."
        exit 1
    fi
    print_success "Project directory verified"
}

# Function to check for sensitive data
check_sensitive_data() {
    print_status "Checking for sensitive data..."
    
    # Check for potential secrets in tracked files
    if git ls-files | xargs grep -l -i "password\|secret\|token\|api[_-]key" 2>/dev/null; then
        print_warning "Found potential sensitive data in tracked files!"
        print_warning "Please review and remove before pushing to GitHub"
        read -p "Continue anyway? (y/N): " choice
        if [[ ! $choice =~ ^[Yy]$ ]]; then
            exit 1
        fi
    fi
    
    # Check for personal information
    if git ls-files | xargs grep -l "william@\|Will-Desktop\|/home/william" 2>/dev/null; then
        print_warning "Found personal information in tracked files"
        print_warning "Consider reviewing and anonymizing if needed"
    fi
    
    print_success "Sensitive data check completed"
}

# Function to setup git configuration
setup_git_config() {
    print_status "Setting up Git configuration..."
    
    # Check if remote already exists
    if git remote get-url origin 2>/dev/null; then
        print_warning "Origin remote already exists"
        print_status "Current origin: $(git remote get-url origin)"
        read -p "Replace with new GitHub repository? (y/N): " choice
        if [[ $choice =~ ^[Yy]$ ]]; then
            git remote remove origin
        else
            print_status "Keeping existing remote"
            return 0
        fi
    fi
    
    # Add GitHub remote
    git remote add origin "$REPO_URL"
    print_success "Added GitHub remote: $REPO_URL"
    
    # Set up branch
    if [ "$(git branch --show-current)" != "$MAIN_BRANCH" ]; then
        git branch -M "$MAIN_BRANCH"
        print_success "Renamed current branch to $MAIN_BRANCH"
    fi
}

# Function to create GitHub-specific files
create_github_files() {
    print_status "Creating GitHub-specific files..."
    
    # Create .github directory
    mkdir -p .github
    
    # Create GitHub issue template
    mkdir -p .github/ISSUE_TEMPLATE
    cat > .github/ISSUE_TEMPLATE/bug_report.md << 'EOF'
---
name: Bug report
about: Create a report to help us improve
title: '[BUG] '
labels: bug
assignees: ''
---

**Describe the bug**
A clear and concise description of what the bug is.

**To Reproduce**
Steps to reproduce the behavior:
1. Go to '...'
2. Click on '....'
3. Scroll down to '....'
4. See error

**Expected behavior**
A clear and concise description of what you expected to happen.

**File Information**
- File format: [e.g. PDF, TXT, CSV]
- File size: [e.g. 1MB]
- Sample file: [attach if possible]

**Environment**
- OS: [e.g. Ubuntu 22.04]
- Rust version: [e.g. 1.70]
- Python version: [if using Python bindings]

**Additional context**
Add any other context about the problem here.
EOF

    # Create feature request template
    cat > .github/ISSUE_TEMPLATE/feature_request.md << 'EOF'
---
name: Feature request
about: Suggest an idea for this project
title: '[FEATURE] '
labels: enhancement
assignees: ''
---

**Is your feature request related to a problem? Please describe.**
A clear and concise description of what the problem is.

**Describe the solution you'd like**
A clear and concise description of what you want to happen.

**Describe alternatives you've considered**
A clear and concise description of any alternative solutions or features you've considered.

**Additional context**
Add any other context or screenshots about the feature request here.
EOF

    # Create pull request template
    cat > .github/pull_request_template.md << 'EOF'
## Description

Brief description of changes made.

## Type of Change

- [ ] Bug fix (non-breaking change which fixes an issue)
- [ ] New feature (non-breaking change which adds functionality)
- [ ] Breaking change (fix or feature that would cause existing functionality to not work as expected)
- [ ] Documentation update

## Testing

- [ ] Tests pass locally
- [ ] Added tests for new functionality
- [ ] Tested with sample files

## Checklist

- [ ] Code follows project style guidelines
- [ ] Self-review completed
- [ ] Documentation updated if needed
- [ ] No sensitive data included
EOF

    # Create GitHub Actions workflow
    mkdir -p .github/workflows
    cat > .github/workflows/ci.yml << 'EOF'
name: CI

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

env:
  CARGO_TERM_COLOR: always

jobs:
  test:
    runs-on: ubuntu-latest
    
    steps:
    - uses: actions/checkout@v3
    
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        override: true
        components: rustfmt, clippy
    
    - name: Cache cargo registry
      uses: actions/cache@v3
      with:
        path: ~/.cargo/registry
        key: ${{ runner.os }}-cargo-registry-${{ hashFiles('**/Cargo.lock') }}
    
    - name: Cache cargo index
      uses: actions/cache@v3
      with:
        path: ~/.cargo/git
        key: ${{ runner.os }}-cargo-index-${{ hashFiles('**/Cargo.lock') }}
    
    - name: Cache cargo build
      uses: actions/cache@v3
      with:
        path: target
        key: ${{ runner.os }}-cargo-build-target-${{ hashFiles('**/Cargo.lock') }}
    
    - name: Check formatting
      run: cargo fmt -- --check
    
    - name: Run clippy
      run: cargo clippy -- -D warnings
    
    - name: Run tests
      run: cargo test --verbose
    
    - name: Build release
      run: cargo build --release --verbose

  python-test:
    runs-on: ubuntu-latest
    
    steps:
    - uses: actions/checkout@v3
    
    - name: Set up Python
      uses: actions/setup-python@v4
      with:
        python-version: '3.9'
    
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        override: true
    
    - name: Install maturin
      run: pip install maturin
    
    - name: Build Python package
      run: maturin build --features python
    
    - name: Install and test Python package
      run: |
        pip install target/wheels/*.whl
        python -c "import doc_loader; print('Python bindings work!')"
EOF

    print_success "GitHub files created"
}

# Function to update README badges
update_readme_badges() {
    print_status "Updating README badges..."
    
    # This would require the README.md to be updated with proper GitHub URLs
    # We'll skip automatic modification for now to avoid breaking the file
    
    print_warning "Please manually update README.md badges to point to the new repository"
    print_warning "Update any remaining references to the old repository URL"
}

# Function to perform final checks
final_checks() {
    print_status "Performing final checks..."
    
    # Check git status
    if [ -n "$(git status --porcelain)" ]; then
        print_warning "Working directory has uncommitted changes"
        git status --short
        print_status "Commit these changes before pushing to GitHub"
    fi
    
    # Verify remote
    if git remote get-url origin >/dev/null 2>&1; then
        print_success "Git remote configured: $(git remote get-url origin)"
    else
        print_error "Git remote not configured!"
        exit 1
    fi
    
    print_success "Final checks completed"
}

# Function to push to GitHub
push_to_github() {
    print_status "Ready to push to GitHub?"
    print_warning "This will upload your repository to: $REPO_URL"
    read -p "Continue? (y/N): " choice
    
    if [[ $choice =~ ^[Yy]$ ]]; then
        print_status "Pushing to GitHub..."
        
        # Push main branch
        git push -u origin "$MAIN_BRANCH"
        
        # Push tags if any
        if git tag -l | grep -q .; then
            git push origin --tags
            print_success "Tags pushed to GitHub"
        fi
        
        print_success "Repository successfully pushed to GitHub!"
        print_success "Visit: https://github.com/WillIsback/doc_loader"
    else
        print_status "Push cancelled. You can push manually later with:"
        print_status "git push -u origin $MAIN_BRANCH"
    fi
}

# Main execution
main() {
    echo "🚀 GitHub Repository Setup for doc_loader"
    echo "=========================================="
    
    check_project_directory
    check_sensitive_data
    setup_git_config
    create_github_files
    update_readme_badges
    final_checks
    
    echo ""
    echo "🎉 Setup completed!"
    echo ""
    
    push_to_github
}

# Run main function
main "$@"
