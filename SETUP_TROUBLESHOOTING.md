# Setup and Troubleshooting Guide

This guide helps you resolve common issues when setting up and developing the MCP Server - Meal Prep project.

## 🚀 Initial Setup

### Prerequisites Check
```bash
# Check Rust installation
rustc --version  # Should be 1.70+
cargo --version  # Should be 1.70+

# Check Git
git --version

# Check system dependencies
uname -a  # Check OS
```

### Fresh Installation
```bash
# Install Rust via rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Verify installation
rustup show
rustup component add rustfmt clippy
```

## 🔧 Common Setup Issues

### 1. Rust Installation Problems

#### Issue: "command not found: rustc"
```bash
# Solution: Add Rust to PATH
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc

# Or for zsh
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

#### Issue: "rustup: command not found"
```bash
# Reinstall rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source ~/.cargo/env
```

#### Issue: "toolchain not found"
```bash
# Install default toolchain
rustup install stable
rustup default stable
```

### 2. Cargo Build Issues

#### Issue: "failed to get `rmcp` as a dependency"
```bash
# Update cargo registry
cargo update

# Clear cargo cache
cargo clean
rm -rf ~/.cargo/registry/cache
rm -rf ~/.cargo/registry/src

# Rebuild
cargo build
```

#### Issue: "failed to compile `tokio`"
```bash
# Check Rust version
rustc --version

# Update to latest stable
rustup update stable

# Clean and rebuild
cargo clean
cargo build
```

#### Issue: "linker not found"
```bash
# Ubuntu/Debian
sudo apt-get update
sudo apt-get install build-essential

# macOS
xcode-select --install

# Windows
# Install Visual Studio Build Tools
```

### 3. Permission Issues

#### Issue: "Permission denied" on scripts
```bash
# Make scripts executable
chmod +x scripts/generate-sbom.sh
chmod +x scripts/security-check.sh
```

#### Issue: "cannot write to target directory"
```bash
# Check permissions
ls -la target/

# Fix permissions
sudo chown -R $USER:$USER target/
chmod -R 755 target/
```

## 🐛 Runtime Issues

### 1. MCP Server Connection Problems

#### Issue: "Server not responding"
```bash
# Check if server is running
ps aux | grep mcp-server-meal-prep

# Test server manually
echo '{"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {"protocolVersion": "2024-11-05", "capabilities": {}, "clientInfo": {"name": "test-client", "version": "1.0.0"}}}' | ./target/release/mcp-server-meal-prep
```

#### Issue: "Protocol version mismatch"
```bash
# Check MCP protocol version
grep -r "protocolVersion" src/

# Update to correct version
# Current version: "2024-11-05"
```

### 2. Recipe Data Issues

#### Issue: "No recipes found for cuisine"
```bash
# Check available cuisines
cargo test test_available_cuisines

# Verify recipe data
cargo test test_recipe_formatting
```

#### Issue: "Invalid cuisine name"
```bash
# Check cuisine names in code
grep -r "cuisine" src/recipes.rs

# Valid cuisines: French, Thai, Italian, Mexican, Chinese, Vietnamese
```

### 3. Memory and Performance Issues

#### Issue: "Out of memory"
```bash
# Check memory usage
cargo build --release  # Use release build for better performance

# Monitor memory
top -p $(pgrep mcp-server-meal-prep)
```

#### Issue: "Slow startup"
```bash
# Use release build
cargo build --release

# Check for debug symbols
file target/release/mcp-server-meal-prep
```

## 🔒 Security Issues

### 1. Audit Failures

#### Issue: "cargo audit failed"
```bash
# Update advisory database
cargo audit update

# Check specific vulnerabilities
cargo audit --deny warnings

# Fix vulnerabilities
cargo update
```

#### Issue: "license check failed"
```bash
# Check license configuration
cat deny.toml

# Update allowed licenses if needed
# Edit deny.toml to add new licenses
```

### 2. SBOM Generation Issues

#### Issue: "auditable build failed"
```bash
# Install auditable
cargo install cargo-auditable

# Try alternative SBOM generation
./scripts/generate-sbom.sh
```

#### Issue: "SBOM file not found"
```bash
# Check if sbom directory exists
ls -la sbom/

# Create directory if missing
mkdir -p sbom
```

## 🧪 Testing Issues

### 1. Test Failures

#### Issue: "test failed: assertion failed"
```bash
# Run specific test with output
cargo test test_name -- --nocapture

# Check test data
cargo test --test integration_tests
```

#### Issue: "tokio runtime not found"
```bash
# Add tokio test dependency
# Check Cargo.toml for tokio-test

# Run with tokio
cargo test --features "tokio"
```

### 2. Integration Test Issues

#### Issue: "integration test not found"
```bash
# Check test file location
ls -la tests/

# Run integration tests specifically
cargo test --test integration_tests
```

## 🔧 Development Environment Issues

### 1. IDE/Editor Issues

#### Issue: "Rust Analyzer not working"
```bash
# VS Code: Install Rust Analyzer extension
# Check rust-analyzer status
rust-analyzer --version

# Restart language server
# VS Code: Ctrl+Shift+P -> "Rust Analyzer: Restart Server"
```

#### Issue: "IntelliSense not working"
```bash
# Check project structure
cargo check

# Regenerate project files
cargo clean
cargo build
```

### 2. Git Issues

#### Issue: "pre-commit hook failed"
```bash
# Check hook permissions
ls -la .git/hooks/

# Make hooks executable
chmod +x .git/hooks/pre-commit

# Run hook manually
.git/hooks/pre-commit
```

#### Issue: "husky not found"
```bash
# Install husky
npm install husky --save-dev

# Setup husky
npx husky install
```

## 📊 Performance Troubleshooting

### 1. Build Performance

#### Issue: "Slow compilation"
```bash
# Use release build for production
cargo build --release

# Enable parallel compilation
export CARGO_BUILD_JOBS=4

# Use sccache for caching
cargo install sccache
export RUSTC_WRAPPER=sccache
```

#### Issue: "Large binary size"
```bash
# Check binary size
ls -lh target/release/mcp-server-meal-prep

# Strip debug symbols
strip target/release/mcp-server-meal-prep

# Use upx for compression (optional)
upx --best target/release/mcp-server-meal-prep
```

### 2. Runtime Performance

#### Issue: "High memory usage"
```bash
# Profile memory usage
cargo install cargo-profdata
cargo profdata --help

# Use release build
cargo build --release
```

## 🆘 Emergency Recovery

### 1. Complete Reset
```bash
# Clean everything
cargo clean
rm -rf target/
rm Cargo.lock

# Reinstall dependencies
cargo build
```

### 2. Dependency Issues
```bash
# Update all dependencies
cargo update

# Force reinstall
rm -rf ~/.cargo/registry/cache
cargo build
```

### 3. Corrupted Build
```bash
# Remove all build artifacts
cargo clean
rm -rf target/

# Rebuild from scratch
cargo build --release
```

## 📞 Getting Help

### When to Ask for Help
- After trying solutions in this guide
- When error messages are unclear
- For performance optimization advice
- For security-related questions

### How to Ask for Help
1. **Include system information**:
   ```bash
   rustc --version
   cargo --version
   uname -a
   ```

2. **Include error messages**: Copy the full error output

3. **Include steps to reproduce**: What commands did you run?

4. **Include relevant files**: Share relevant parts of Cargo.toml, error logs

### Contact Information
- **Email**: jin.wen@hisgarden.org
- **Repository**: https://github.com/hisgarden/mcp-server-meal-prep
- **Issues**: Create GitHub issue with detailed information

## 🔍 Debugging Tips

### 1. Enable Debug Logging
```bash
# Set log level
export RUST_LOG=debug
cargo run

# Or use specific module logging
export RUST_LOG=mcp_server_meal_prep=debug
```

### 2. Use Debugger
```bash
# Install debugger
cargo install cargo-gdb

# Debug with gdb
cargo gdb
```

### 3. Profile Code
```bash
# Install profiler
cargo install cargo-profdata

# Profile execution
cargo profdata run --release
```

---

**Remember**: Most issues have simple solutions. Start with the basics (clean build, update dependencies) before diving into complex debugging!
