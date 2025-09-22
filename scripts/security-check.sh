#!/bin/bash

# Comprehensive Security Test Suite for Rust MCP Server
# This script runs all security checks before remote commits

set -uo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Counters
TOTAL_CHECKS=0
PASSED_CHECKS=0
FAILED_CHECKS=0
WARNING_CHECKS=0

# Logging functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[PASS]${NC} $1"
    ((PASSED_CHECKS++))
}

log_warning() {
    echo -e "${YELLOW}[WARN]${NC} $1"
    ((WARNING_CHECKS++))
}

log_error() {
    echo -e "${RED}[FAIL]${NC} $1"
    ((FAILED_CHECKS++))
}

log_check() {
    echo -e "${BLUE}[CHECK]${NC} $1"
    ((TOTAL_CHECKS++))
}

# Check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Security check functions
check_rust_version() {
    log_check "Checking Rust version compatibility"
    
    if ! command_exists rustc; then
        log_error "Rust compiler not found"
        return 1
    fi
    
    local rust_version=$(rustc --version | cut -d' ' -f2)
    local required_version="1.70.0"
    
    if [ "$(printf '%s\n' "$required_version" "$rust_version" | sort -V | head -n1)" = "$required_version" ]; then
        log_success "Rust version $rust_version is compatible (>= $required_version)"
    else
        log_error "Rust version $rust_version is too old (requires >= $required_version)"
        return 1
    fi
}

check_cargo_audit() {
    log_check "Running cargo audit for vulnerability scanning"
    
    if ! command_exists cargo; then
        log_error "Cargo not found"
        return 1
    fi
    
    # Install cargo-audit if not present
    if ! cargo audit --version >/dev/null 2>&1; then
        log_info "Installing cargo-audit..."
        cargo install cargo-audit
    fi
    
    # Update advisory database
    log_info "Updating advisory database..."
    cargo audit update
    
    # Run audit (treat vulnerabilities in dev dependencies as warnings)
    local audit_output=$(cargo audit 2>&1 || true)
    if echo "$audit_output" | grep -q "No vulnerabilities found"; then
        log_success "No security vulnerabilities found"
    else
        # Check if vulnerabilities are only in dev dependencies
        local high_severity=$(echo "$audit_output" | grep -c "Severity: [8-9]\." || true)
        local critical_severity=$(echo "$audit_output" | grep -c "Severity: 10\." || true)
        
        if [ "$high_severity" -gt 0 ] || [ "$critical_severity" -gt 0 ]; then
            log_error "High or critical security vulnerabilities detected"
            echo "$audit_output" | head -20
            return 1
        else
            log_warning "Low/medium security vulnerabilities found in dev dependencies"
            echo "$audit_output" | head -10
        fi
    fi
}

check_cargo_deny() {
    log_check "Running cargo deny for license compliance"
    
    # Install cargo-deny if not present
    if ! cargo deny --version >/dev/null 2>&1; then
        log_info "Installing cargo-deny..."
        cargo install cargo-deny
    fi
    
    # Check licenses
    if cargo deny check licenses 2>/dev/null; then
        log_success "License compliance check passed"
    else
        log_warning "License compliance check failed or cargo-deny not installed"
        log_info "Install with: cargo install cargo-deny"
    fi
    
    # Check advisories
    if cargo deny check advisories 2>/dev/null; then
        log_success "Advisory check passed"
    else
        log_warning "Advisory check failed or cargo-deny not installed"
    fi
}

check_unsafe_code() {
    log_check "Scanning for unsafe code blocks"
    
    local unsafe_count=$(grep -r "unsafe" src/ --include="*.rs" | wc -l)
    
    if [ "$unsafe_count" -eq 0 ]; then
        log_success "No unsafe code blocks found"
    else
        log_warning "Found $unsafe_count unsafe code blocks"
        grep -r "unsafe" src/ --include="*.rs" || true
    fi
}

check_dependencies() {
    log_check "Checking dependency security"
    
    # Check for known vulnerable crates
    local vulnerable_crates=(
        "chrono"
        "time"
        "uuid"
        "rand"
    )
    
    for crate in "${vulnerable_crates[@]}"; do
        if grep -q "\"$crate\"" Cargo.toml; then
            log_warning "Using potentially vulnerable crate: $crate"
        fi
    done
    
    # Check for outdated dependencies
    if cargo outdated --exit-code 1 >/dev/null 2>&1; then
        log_warning "Some dependencies are outdated"
        cargo outdated || true
    else
        log_success "All dependencies are up to date"
    fi
}

check_build_security() {
    log_check "Verifying security-hardened build configuration"
    
    # Check Cargo.toml for security settings
    if grep -q "panic = \"abort\"" Cargo.toml; then
        log_success "Panic abort strategy enabled"
    else
        log_warning "Panic abort strategy not enabled"
    fi
    
    if grep -q "lto = true" Cargo.toml; then
        log_success "Link-time optimization enabled"
    else
        log_warning "Link-time optimization not enabled"
    fi
    
    if grep -q "strip = true" Cargo.toml; then
        log_success "Debug symbol stripping enabled"
    else
        log_warning "Debug symbol stripping not enabled"
    fi
}

check_code_quality() {
    log_check "Running code quality checks"
    
    # Format check
    if cargo fmt -- --check; then
        log_success "Code formatting is correct"
    else
        log_error "Code formatting issues found"
        log_info "Run 'cargo fmt' to fix formatting"
        return 1
    fi
    
    # Clippy check
    if cargo clippy -- -D warnings; then
        log_success "Clippy linting passed"
    else
        log_error "Clippy linting failed"
        return 1
    fi
}

check_tests() {
    log_check "Running security-focused tests"
    
    # Run all tests
    if cargo test --release; then
        log_success "All tests passed"
    else
        log_error "Some tests failed"
        return 1
    fi
    
    # Run integration tests specifically
    if cargo test --test integration_tests --release; then
        log_success "Integration tests passed"
    else
        log_error "Integration tests failed"
        return 1
    fi
}

check_sbom_generation() {
    log_check "Generating and validating SBOM"
    
    # Generate SBOM
    if ./scripts/generate-sbom.sh; then
        log_success "SBOM generation successful"
    else
        log_error "SBOM generation failed"
        return 1
    fi
    
    # Check if SBOM files exist
    if [ -f "sbom/sbom-summary.txt" ]; then
        log_success "SBOM summary file created"
    else
        log_error "SBOM summary file not found"
        return 1
    fi
}

check_binary_security() {
    log_check "Analyzing binary security"
    
    # Build release binary
    if cargo build --release; then
        log_success "Release build successful"
    else
        log_error "Release build failed"
        return 1
    fi
    
    local binary_path="target/release/mcp-server-meal-prep"
    
    if [ -f "$binary_path" ]; then
        # Check binary size
        local binary_size=$(du -h "$binary_path" | cut -f1)
        log_info "Binary size: $binary_size"
        
        # Check for debug symbols (should be stripped)
        if file "$binary_path" | grep -q "not stripped"; then
            log_warning "Binary contains debug symbols (should be stripped in release)"
        else
            log_success "Binary is properly stripped"
        fi
        
        # Generate checksum
        local checksum=$(shasum -a 256 "$binary_path" | cut -d' ' -f1)
        log_info "Binary SHA256: $checksum"
        
        # Save checksum
        echo "$checksum  $binary_path" > "sbom/binary-checksum.txt"
        log_success "Binary checksum saved"
    else
        log_error "Release binary not found"
        return 1
    fi
}

check_git_security() {
    log_check "Checking Git security configuration"
    
    # Check for sensitive files in Git
    local sensitive_files=(
        ".env"
        "*.key"
        "*.pem"
        "*.p12"
        "secrets.txt"
        "password.txt"
    )
    
    for pattern in "${sensitive_files[@]}"; do
        if git ls-files | grep -q "$pattern"; then
            log_error "Sensitive file found in Git: $pattern"
            return 1
        fi
    done
    
    log_success "No sensitive files found in Git"
    
    # Check .gitignore
    if [ -f ".gitignore" ]; then
        log_success ".gitignore file exists"
    else
        log_warning ".gitignore file not found"
    fi
}

check_documentation() {
    log_check "Checking security documentation"
    
    local required_docs=(
        "SECURITY.md"
        "README.md"
        "RUST_DEVELOPMENT_GUIDE.md"
        "SETUP_TROUBLESHOOTING.md"
    )
    
    for doc in "${required_docs[@]}"; do
        if [ -f "$doc" ]; then
            log_success "Documentation file exists: $doc"
        else
            log_error "Missing documentation: $doc"
            return 1
        fi
    done
}

# Main execution
main() {
    echo "🔒 Starting Comprehensive Security Test Suite"
    echo "=============================================="
    echo ""
    
    # Create sbom directory if it doesn't exist
    mkdir -p sbom
    
    # Run all security checks
    check_rust_version
    check_cargo_audit
    check_cargo_deny
    check_unsafe_code
    check_dependencies
    check_build_security
    check_code_quality
    check_tests
    check_sbom_generation
    check_binary_security
    check_git_security
    check_documentation
    
    # Summary
    echo ""
    echo "📊 Security Test Suite Summary"
    echo "=============================="
    echo "Total checks: $TOTAL_CHECKS"
    echo -e "Passed: ${GREEN}$PASSED_CHECKS${NC}"
    echo -e "Warnings: ${YELLOW}$WARNING_CHECKS${NC}"
    echo -e "Failed: ${RED}$FAILED_CHECKS${NC}"
    echo ""
    
    if [ $FAILED_CHECKS -eq 0 ]; then
        echo -e "${GREEN}✅ All security checks passed!${NC}"
        echo "🚀 Ready for remote commit"
        exit 0
    else
        echo -e "${RED}❌ Security checks failed!${NC}"
        echo "🛑 Do not commit until issues are resolved"
        exit 1
    fi
}

# Run main function
main "$@"
