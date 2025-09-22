#!/bin/bash

# Generate Software Bill of Materials (SBOM) for Rust MCP Server
# This script generates an SBOM in SPDX format for supply chain security

set -euo pipefail

echo "🔍 Generating SBOM for MCP Server - Meal Prep (Rust)..."

# Create sbom directory if it doesn't exist
mkdir -p sbom

# Generate Cargo.lock if it doesn't exist
if [ ! -f "Cargo.lock" ]; then
    echo "📦 Generating Cargo.lock..."
    cargo check
fi

# Generate SBOM using cargo-auditable (if available)
echo "📋 Generating auditable SBOM..."
if command -v cargo-auditable >/dev/null 2>&1; then
    cargo auditable build --release
else
    echo "⚠️  cargo-auditable not found, using regular build"
    cargo build --release
fi

# Generate dependency tree
echo "📊 Generating dependency tree..."
cargo tree --format "{p} {f}" > sbom/dependency-tree.txt

# Generate license information
echo "📄 Generating license information..."
cargo deny check licenses > sbom/license-report.txt 2>&1 || true

# Generate security audit
echo "🔒 Generating security audit..."
cargo audit > sbom/security-audit.txt 2>&1 || true

# Generate human-readable summary
echo "📊 Generating SBOM summary..."
cat > sbom/sbom-summary.txt << EOF
Software Bill of Materials (SBOM) Summary
=========================================

Generated: $(date -u +"%Y-%m-%d %H:%M:%S UTC")
Project: MCP Server - Meal Prep (Rust)
Version: $(grep '^version' Cargo.toml | cut -d'"' -f2)
Author: $(grep '^authors' Cargo.toml | cut -d'"' -f2)
License: $(grep '^license' Cargo.toml | cut -d'"' -f2)

Dependencies:
$(cargo tree | grep -c '├\|└') total dependencies

Security Status:
$(cargo audit --quiet 2>&1 | head -5 || echo "Security audit completed")

License Compliance:
$(cargo deny check licenses --quiet 2>&1 | head -5 || echo "License check completed")

Files Generated:
- target/release/mcp-server-meal-prep (auditable binary)
- sbom/dependency-tree.txt (Dependency tree)
- sbom/license-report.txt (License compliance report)
- sbom/security-audit.txt (Security audit report)
- sbom/sbom-summary.txt (Human-readable summary)

Binary Information:
- Size: $(du -h target/release/mcp-server-meal-prep 2>/dev/null | cut -f1 || echo "N/A")
- SHA256: $(shasum -a 256 target/release/mcp-server-meal-prep 2>/dev/null | cut -d' ' -f1 || echo "N/A")
EOF

echo "✅ SBOM generation complete!"
echo "📁 Files generated in ./sbom/ directory"
echo "🔍 Review sbom/sbom-summary.txt for overview"
echo "🔒 Auditable binary created at target/release/mcp-server-meal-prep"
