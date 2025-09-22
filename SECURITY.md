# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 1.0.x   | :white_check_mark: |

## Reporting a Vulnerability

If you discover a security vulnerability, please report it responsibly:

1. **DO NOT** create a public GitHub issue
2. Email security details to: jin.wen@hisgarden.org
3. Include:
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact assessment
   - Suggested fix (if any)

## Security Measures

This project implements the following security practices:

### Dependency Management
- Regular dependency audits using `cargo audit`
- License compliance checking with `cargo deny`
- Automated security checks in CI/CD pipeline
- Pinned dependency versions in `Cargo.lock`

### Code Quality
- Rust's memory safety guarantees
- Compile-time type checking
- Input validation using serde schemas
- No unsafe code blocks
- No direct file system access outside designated directories

### Runtime Security
- No execution of user-provided code
- Input sanitization for all user inputs
- Resource access limited to predefined recipe data
- No network requests to external services
- Panic handling with abort strategy in release builds

### Development Security
- Pre-commit hooks for code quality
- Automated security scanning with cargo-audit
- Regular dependency updates
- Secure coding practices
- SBOM generation for supply chain security

### Build Security
- Link-time optimization (LTO) enabled
- Debug symbols stripped in release builds
- Single codegen unit for better optimization
- Panic abort strategy for smaller binaries

## Security Checklist

- [x] Input validation implemented
- [x] No code injection vulnerabilities
- [x] Dependency vulnerabilities monitored
- [x] Secure coding practices followed
- [x] Regular security audits scheduled
- [x] SBOM generation for supply chain security
- [x] License compliance verified
- [x] Memory safety guaranteed by Rust
- [x] No unsafe code blocks

## Security Commands

```bash
# Run security audit
cargo audit

# Check license compliance
cargo deny check

# Generate SBOM
cargo auditable build

# Security-focused build
cargo build --release
```

## Contact

For security-related questions or reports:
- Email: jin.wen@hisgarden.org
- Response time: Within 48 hours for security issues
