# MCP Server - Meal Prep (Rust)

A high-performance MCP (Model Context Protocol) server built in Rust for meal preparation, featuring cuisine-based recipes and intelligent meal planning capabilities.

## 🚀 Features

- **Multi-Cuisine Recipe Database**: Traditional recipes from French, Thai, Italian, Mexican, Chinese, and Vietnamese cuisines
- **Intelligent Meal Planning**: Generate weekly meal plans with optimized shopping lists
- **Ingredient Overlap Analysis**: Minimize food waste by identifying common ingredients
- **Resource Templates**: Dynamic recipe access via `file://recipes/{cuisine}` URIs
- **High Performance**: Built with Rust for excellent performance and memory safety
- **Type Safety**: Compile-time guarantees with zero-cost abstractions
- **Comprehensive Security**: Pre-commit hooks, vulnerability scanning, SBOM generation
- **Cross-Platform**: Portable binary that runs anywhere without dependencies

## 🏗️ Architecture

### Rust Implementation Benefits

- **Memory Safety**: No null pointer exceptions or memory leaks
- **Performance**: Near C-level performance with excellent concurrency
- **Type System**: Powerful compile-time type checking
- **Macro System**: Code generation for MCP protocol compliance
- **Async Runtime**: Tokio-based async/await for high throughput

### Project Structure

```
src/
├── main.rs           # MCP server implementation with tools and prompts
├── recipes.rs        # Recipe data structures and database
├── meal_planner.rs   # Meal planning logic and optimization
├── lib.rs            # Library entry point
tests/
├── integration_tests.rs # Comprehensive test suite
scripts/
├── security-check.sh    # Security validation script
├── generate-sbom.sh     # SBOM generation script
benches/
├── benchmark.rs         # Performance benchmarking
Cargo.toml              # Dependencies and project configuration
deny.toml               # License and security policy
Makefile                # Development commands
```

## 🛠️ Setup

### Prerequisites

- Rust 1.70+ (install via [rustup.rs](https://rustup.rs/))
- Cargo (comes with Rust)
- Git (for version control)

### Installation

```sh
# Clone or navigate to the project directory
cd mcp-server-meal-prep

# Build the project
cargo build --release

# Run the server
cargo run --release
```

### Development

```sh
# Run in development mode
cargo run

# Run tests
cargo test

# Check code formatting
cargo fmt

# Lint the code
cargo clippy

# Run security checks
make security-check

# Generate SBOM
make sbom

# Run benchmarks
cargo bench
```

## 🎯 Available Features

### Tools

1. **`get_cuisines`**: List all available cuisines
2. **`get_recipes`**: Get recipes for a specific cuisine
3. **`generate_meal_plan`**: Create a meal plan with shopping list
4. **`analyze_ingredient_overlap`**: Find common ingredients across recipes
5. **`weekly_meal_planner`**: Comprehensive weekly meal planning with cuisine selection

### Prompts

- **`weekly_meal_planner`**: Comprehensive weekly meal planning with cuisine selection

### Resources

- **`file://recipes/{cuisine}`**: Access recipes by cuisine (French, Thai, Italian, Mexican, Chinese, Vietnamese)

## 📋 Usage Examples

### MCP Client Configuration

Add to your MCP client configuration:

```json
{
  "mcpServers": {
    "meal-prep": {
      "command": "/path/to/mcp-server-meal-prep/target/release/mcp-server-meal-prep",
      "args": [],
      "env": {}
    }
  }
}
```

### Example Tool Calls

```json
// Get available cuisines
{
  "method": "tools/call",
  "params": {
    "name": "get_cuisines",
    "arguments": {}
  }
}

// Get French recipes
{
  "method": "tools/call",
  "params": {
    "name": "get_recipes",
    "arguments": {
      "cuisine": "French"
    }
  }
}

// Generate 7-day meal plan for Italian cuisine
{
  "method": "tools/call",
  "params": {
    "name": "generate_meal_plan",
    "arguments": {
      "cuisine": "Italian",
      "days": 7,
      "servings": 4
    }
  }
}

// Analyze ingredient overlap for Mexican cuisine
{
  "method": "tools/call",
  "params": {
    "name": "analyze_ingredient_overlap",
    "arguments": {
      "cuisine": "Mexican"
    }
  }
}
```

### Example Prompt Usage

```json
// Weekly meal planner for Vietnamese cuisine
{
  "method": "prompts/get",
  "params": {
    "name": "weekly_meal_planner",
    "arguments": {
      "cuisine": "Vietnamese"
    }
  }
}
```

## 🍳 Supported Cuisines

### French Cuisine
- **Coq au Vin**: Classic French chicken in wine
- **French Onion Soup**: Traditional onion soup with gruyère
- **Crème Brûlée**: Vanilla custard with caramelized sugar

### Thai Cuisine
- **Pad Thai**: Stir-fried rice noodles with shrimp and vegetables
- **Tom Yum Goong**: Spicy and sour shrimp soup
- **Mango Sticky Rice**: Sweet dessert with coconut milk

### Italian Cuisine
- **Spaghetti Carbonara**: Pasta with eggs, cheese, and pancetta
- **Margherita Pizza**: Classic tomato, mozzarella, and basil pizza
- **Tiramisu**: Coffee-flavored dessert with mascarpone

### Mexican Cuisine
- **Tacos al Pastor**: Spiced pork tacos with pineapple
- **Guacamole**: Avocado dip with lime and cilantro
- **Churros**: Fried dough pastries with cinnamon sugar

### Chinese Cuisine
- **Kung Pao Chicken**: Spicy stir-fried chicken with peanuts and vegetables
- **Char Siu (Chinese BBQ Pork)**: Sweet and savory roasted pork
- **Egg Tarts (Dan Tat)**: Creamy custard tarts with flaky pastry

### Vietnamese Cuisine
- **Pho Bo (Beef Noodle Soup)**: Traditional Vietnamese noodle soup
- **Banh Mi**: Vietnamese sandwich with pickled vegetables
- **Che Ba Mau (Three Color Dessert)**: Layered dessert with beans and coconut

## 🧪 Testing

```sh
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_meal_plan_generation
```

## 📊 Performance Characteristics

- **Startup Time**: 0.020s (5.5x faster than TypeScript)
- **Memory Usage**: ~10-20MB (native binary efficiency)
- **Binary Size**: 1.6M (self-contained executable)
- **Build Time**: 1.97s (release build)
- **Throughput**: Excellent for CPU-intensive meal planning
- **Latency**: Minimal overhead for recipe access

## 🔧 Development

### Adding New Cuisines

1. Edit `src/recipes.rs`
2. Add new recipes to the `get_recipe_database()` function
3. Rebuild: `cargo build --release`
4. Restart MCP client

### Adding New Tools

1. Add tool function to `MealPrepServer` implementation
2. Use `#[tool(description = "...")]` attribute
3. Implement error handling with `McpError`
4. Add tests for the new functionality

### Adding New Prompts

1. Add prompt function to `MealPrepServer` implementation
2. Use `#[prompt(name = "...")]` attribute
3. Return `GetPromptResult` with messages
4. Add argument validation

## 🚀 Deployment

### Building for Production

```sh
# Build optimized release binary
cargo build --release

# The binary will be at:
# target/release/mcp-server-meal-prep
```

### Cross-Platform Compilation

```sh
# Install cross-compilation target
rustup target add x86_64-unknown-linux-gnu

# Build for Linux from macOS
cargo build --release --target x86_64-unknown-linux-gnu
```

## 🔍 Troubleshooting

### Common Issues

1. **Build Errors**: Ensure Rust 1.70+ is installed
2. **Runtime Errors**: Check that all dependencies are available
3. **MCP Connection**: Verify the binary path in client configuration
4. **Permission Issues**: Ensure the binary is executable

### Debug Mode

```sh
# Run with debug logging
RUST_LOG=debug cargo run
```

## 📈 Performance vs TypeScript

| Metric | Rust | TypeScript | Winner |
|--------|------|------------|--------|
| **Startup Time** | 0.020s | 0.110s | 🏆 Rust |
| **Memory Usage** | ~10-20MB | ~50-100MB | 🏆 Rust |
| **Binary Size** | 1.6M | 4.0K | 🏆 TypeScript |
| **Build Time** | 1.97s | 1.15s | 🏆 TypeScript |
| **Dependencies** | 3.9G (target) | 78M (node_modules) | 🏆 TypeScript |
| **Type Safety** | Compile + Runtime | Compile-time | 🏆 Rust |
| **Concurrency** | Excellent (Tokio) | Good (Node.js) | 🏆 Rust |
| **Portability** | Universal binary | Node.js required | 🏆 Rust |

## 🔒 Security Features

This MCP server implements comprehensive security best practices:

### Security Tools
- **cargo audit**: Vulnerability scanning for dependencies
- **cargo deny**: License compliance and advisory checking
- **Pre-commit hooks**: Automated security validation
- **SBOM generation**: Software Bill of Materials for supply chain security

### Security Hardening
- **Memory safety**: Zero unsafe code blocks
- **Stripped binaries**: Debug symbols removed from release builds
- **Security profiles**: Hardened build configurations
- **Dependency auditing**: Automated vulnerability scanning

### Security Documentation
- **SECURITY.md**: Vulnerability reporting process
- **RUST_DEVELOPMENT_GUIDE.md**: Security-aware development practices
- **SETUP_TROUBLESHOOTING.md**: Security troubleshooting guide

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/new-cuisine`
3. Add your changes with tests
4. Run `cargo test` and `cargo clippy`
5. Run security checks: `make security-check`
6. Submit a pull request

## 📄 License

MIT License - see [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with the [RMCP](https://github.com/modelcontextprotocol/rust-sdk) Rust SDK
- Inspired by the TypeScript MCP server example
- Recipe data compiled from traditional cooking sources
- Security practices based on OWASP guidelines
- Author: Jin Wen <jin.wen@hisgarden.org>

---

**Ready to cook with Rust! 🦀🍳** Your high-performance MCP server is ready for meal planning and recipe access.
