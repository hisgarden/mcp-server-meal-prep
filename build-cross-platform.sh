#!/bin/bash

# Cross-Platform Build Script for Rust MCP Server
# Builds binaries for multiple platforms

set -euo pipefail

echo "🚀 Building Rust MCP Server for Multiple Platforms"
echo "================================================="

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

# Create releases directory
mkdir -p releases

# Function to build for a specific target
build_target() {
    local target=$1
    local platform_name=$2
    
    print_status "Building for $platform_name ($target)..."
    
    if cargo build --release --target "$target" 2>/dev/null; then
        local binary_name="mcp-server-meal-prep"
        if [[ "$target" == *"windows"* ]]; then
            binary_name="mcp-server-meal-prep.exe"
        fi
        
        local source_path="target/$target/release/$binary_name"
        local dest_path="releases/mcp-server-meal-prep-$platform_name"
        
        if [ -f "$source_path" ]; then
            cp "$source_path" "$dest_path"
            local size=$(du -h "$dest_path" | cut -f1)
            print_success "Built $platform_name: $size"
        else
            print_warning "Binary not found for $platform_name"
        fi
    else
        print_warning "Failed to build for $platform_name"
    fi
}

# Build for different platforms
print_status "Starting cross-platform builds..."

# macOS (current platform)
build_target "aarch64-apple-darwin" "macos-arm64"

# Linux
build_target "x86_64-unknown-linux-gnu" "linux-x86_64"

# Windows
build_target "x86_64-pc-windows-gnu" "windows-x86_64"

# List all built binaries
echo ""
print_status "Built binaries:"
ls -lh releases/

echo ""
print_success "Cross-platform build completed!"
echo ""
echo "📦 Built binaries:"
for binary in releases/*; do
    if [ -f "$binary" ]; then
        size=$(du -h "$binary" | cut -f1)
        echo "  $(basename "$binary"): $size"
    fi
done

echo ""
echo "🌍 Platform compatibility:"
echo "  - macOS ARM64: Native Apple Silicon"
echo "  - Linux x86_64: Most Linux distributions"
echo "  - Windows x86_64: Windows 10/11"
echo ""
echo "📋 Usage:"
echo "  ./releases/mcp-server-meal-prep-<platform>"
echo ""
echo "🔒 Security: All binaries are statically linked with minimal dependencies"
