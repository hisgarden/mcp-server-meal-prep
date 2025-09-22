# Makefile for Rust MCP Server - Meal Prep
# Provides convenient commands for development and security

.PHONY: help build test clean security audit sbom install-tools format lint

# Default target
help:
	@echo "🦀 Rust MCP Server - Meal Prep"
	@echo "=============================="
	@echo ""
	@echo "Development Commands:"
	@echo "  make build          - Build the project (debug)"
	@echo "  make build-release  - Build optimized release binary"
	@echo "  make build-security - Build security-hardened binary"
	@echo "  make test           - Run all tests"
	@echo "  make test-integration - Run integration tests only"
	@echo "  make clean          - Clean build artifacts"
	@echo ""
	@echo "Code Quality Commands:"
	@echo "  make format         - Format code with rustfmt"
	@echo "  make lint           - Run clippy linter"
	@echo "  make check          - Check code without building"
	@echo ""
	@echo "Security Commands:"
	@echo "  make security       - Run full security test suite"
	@echo "  make audit          - Run cargo audit"
	@echo "  make audit-update   - Update audit database"
	@echo "  make deny           - Run cargo deny checks"
	@echo "  make sbom           - Generate SBOM"
	@echo "  make install-tools  - Install security tools"
	@echo ""
	@echo "Documentation Commands:"
	@echo "  make docs           - Generate documentation"
	@echo "  make docs-open      - Generate and open documentation"
	@echo ""

# Build commands
build:
	@echo "🔨 Building project (debug)..."
	cargo build

build-release:
	@echo "🔨 Building optimized release binary..."
	cargo build --release

build-security:
	@echo "🔒 Building security-hardened binary..."
	cargo build --profile security

# Test commands
test:
	@echo "🧪 Running all tests..."
	cargo test

test-integration:
	@echo "🧪 Running integration tests..."
	cargo test --test integration_tests

test-release:
	@echo "🧪 Running tests in release mode..."
	cargo test --release

# Code quality commands
format:
	@echo "🎨 Formatting code..."
	cargo fmt

lint:
	@echo "🔍 Running clippy linter..."
	cargo clippy -- -D warnings

check:
	@echo "✅ Checking code..."
	cargo check

# Security commands
security:
	@echo "🔒 Running comprehensive security test suite..."
	./scripts/security-check.sh

audit:
	@echo "🔍 Running cargo audit..."
	cargo audit

audit-update:
	@echo "🔄 Updating audit database..."
	cargo audit update

deny:
	@echo "🚫 Running cargo deny checks..."
	cargo deny check

sbom:
	@echo "📋 Generating SBOM..."
	./scripts/generate-sbom.sh

install-tools:
	@echo "🛠️ Installing security tools..."
	cargo install cargo-audit cargo-deny cargo-auditable
	@echo "✅ Security tools installed"

# Documentation commands
docs:
	@echo "📚 Generating documentation..."
	cargo doc --no-deps

docs-open:
	@echo "📚 Generating and opening documentation..."
	cargo doc --no-deps --open

# Utility commands
clean:
	@echo "🧹 Cleaning build artifacts..."
	cargo clean

run:
	@echo "🚀 Running the server..."
	cargo run

run-release:
	@echo "🚀 Running the server (release mode)..."
	cargo run --release

# Development workflow
dev-setup: install-tools
	@echo "🔧 Setting up development environment..."
	rustup component add rustfmt clippy
	@echo "✅ Development environment ready"

pre-commit: format lint test security
	@echo "✅ Pre-commit checks completed"

# CI/CD commands
ci-test: test-release
	@echo "✅ CI tests completed"

ci-security: audit deny security
	@echo "✅ CI security checks completed"

ci-full: ci-test ci-security
	@echo "✅ Full CI pipeline completed"

# Release commands
release-check: clean build-security test-release security
	@echo "✅ Release checks completed"

release: release-check
	@echo "🚀 Creating release..."
	@echo "Binary location: target/security/mcp-server-meal-prep"
	@echo "Size: $$(du -h target/security/mcp-server-meal-prep | cut -f1)"
	@echo "SHA256: $$(shasum -a 256 target/security/mcp-server-meal-prep | cut -d' ' -f1)"

# Help for specific commands
help-security:
	@echo "🔒 Security Commands Help"
	@echo "========================"
	@echo ""
	@echo "make security       - Run full security test suite"
	@echo "make audit          - Check for known vulnerabilities"
	@echo "make deny           - Check license compliance"
	@echo "make sbom           - Generate Software Bill of Materials"
	@echo "make install-tools  - Install required security tools"
	@echo ""
	@echo "Security test suite includes:"
	@echo "  - Rust version compatibility"
	@echo "  - Vulnerability scanning"
	@echo "  - License compliance"
	@echo "  - Unsafe code detection"
	@echo "  - Dependency security"
	@echo "  - Build security configuration"
	@echo "  - Code quality checks"
	@echo "  - Test execution"
	@echo "  - SBOM generation"
	@echo "  - Binary security analysis"
	@echo "  - Git security checks"
	@echo "  - Documentation verification"

help-dev:
	@echo "🦀 Development Commands Help"
	@echo "==========================="
	@echo ""
	@echo "make dev-setup      - Set up development environment"
	@echo "make build          - Build debug version"
	@echo "make build-release  - Build optimized version"
	@echo "make test           - Run all tests"
	@echo "make format         - Format code"
	@echo "make lint           - Run linter"
	@echo "make check          - Check code without building"
	@echo "make run            - Run the server"
	@echo "make clean          - Clean build artifacts"
	@echo ""
	@echo "For new Rust developers, see:"
	@echo "  - RUST_DEVELOPMENT_GUIDE.md"
	@echo "  - SETUP_TROUBLESHOOTING.md"
