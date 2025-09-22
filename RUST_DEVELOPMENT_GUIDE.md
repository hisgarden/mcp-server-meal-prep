# Rust Development Guide for MCP Server

This guide is designed for developers new to Rust who want to contribute to the MCP Server - Meal Prep project.

## 🦀 Getting Started with Rust

### Prerequisites
- **Rust 1.70+**: Install from [rustup.rs](https://rustup.rs/)
- **Cargo**: Comes with Rust installation
- **Git**: For version control
- **VS Code** (recommended): With Rust Analyzer extension

### Quick Setup
```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Verify installation
rustc --version
cargo --version
```

## 📁 Project Structure

```
mcp-server-meal-prep/
├── src/
│   ├── main.rs          # Main MCP server implementation
│   ├── lib.rs           # Library entry point for tests
│   ├── recipes.rs       # Recipe data structures and logic
│   └── meal_planner.rs  # Meal planning algorithms
├── tests/
│   └── integration_tests.rs  # Integration tests
├── scripts/
│   └── generate-sbom.sh # Security SBOM generation
├── Cargo.toml           # Project configuration and dependencies
├── Cargo.lock           # Locked dependency versions
├── deny.toml            # Security and license configuration
└── README.md            # Project documentation
```

## 🛠️ Development Workflow

### 1. Building the Project
```bash
# Debug build (faster compilation, larger binary)
cargo build

# Release build (optimized, smaller binary)
cargo build --release

# Check code without building
cargo check
```

### 2. Running the Server
```bash
# Run in debug mode
cargo run

# Run in release mode
cargo run --release

# Run with specific arguments
cargo run -- --help
```

### 3. Testing
```bash
# Run all tests
cargo test

# Run specific test
cargo test test_available_cuisines

# Run tests with output
cargo test -- --nocapture

# Run integration tests only
cargo test --test integration_tests
```

### 4. Code Quality
```bash
# Format code
cargo fmt

# Lint code
cargo clippy

# Fix clippy suggestions
cargo clippy --fix
```

## 🔧 Key Rust Concepts for This Project

### 1. Ownership and Borrowing
```rust
// Ownership: data has one owner
let recipes = get_recipe_database();

// Borrowing: temporary access without ownership
let french_recipes = &recipes["French"];

// Mutable borrowing: temporary mutable access
let mut meal_plan = MealPlan::generate("Italian", 7, 4);
```

### 2. Error Handling
```rust
// Result type for operations that can fail
fn get_recipes(cuisine: &str) -> Result<CuisineRecipes, String> {
    let database = get_recipe_database();
    database.get(cuisine)
        .ok_or_else(|| format!("Cuisine '{}' not found", cuisine))
        .cloned()
}

// Using ? operator for error propagation
fn process_request() -> Result<(), Box<dyn std::error::Error>> {
    let recipes = get_recipes("French")?;
    // Process recipes...
    Ok(())
}
```

### 3. Pattern Matching
```rust
// Match on different cases
match cuisine {
    "French" => format_french_recipes(),
    "Italian" => format_italian_recipes(),
    _ => format!("Unknown cuisine: {}", cuisine),
}

// Destructuring
if let Some(recipes) = database.get(cuisine) {
    // Handle found recipes
}
```

### 4. Traits and Generics
```rust
// Trait for serialization
#[derive(Serialize, Deserialize)]
struct Recipe {
    name: String,
    ingredients: Vec<String>,
}

// Generic function
fn format_recipes<T: Display>(recipes: &[T]) -> String {
    recipes.iter()
        .map(|r| r.to_string())
        .collect::<Vec<_>>()
        .join("\n")
}
```

## 🧪 Testing in Rust

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recipe_creation() {
        let recipe = Recipe {
            name: "Test Recipe".to_string(),
            ingredients: vec!["ingredient1".to_string()],
            instructions: vec!["step1".to_string()],
        };
        assert_eq!(recipe.name, "Test Recipe");
    }
}
```

### Integration Tests
```rust
// In tests/integration_tests.rs
#[tokio::test]
async fn test_meal_plan_generation() {
    let meal_plan = MealPlan::generate("Italian", 7, 4);
    assert_eq!(meal_plan.cuisine, "Italian");
    assert_eq!(meal_plan.days.len(), 7);
}
```

### Testing Best Practices
- **Arrange-Act-Assert**: Structure tests clearly
- **Descriptive names**: Test names should explain what they test
- **One concept per test**: Each test should verify one specific behavior
- **Use `assert!` macros**: `assert_eq!`, `assert_ne!`, `assert!`

## 🔒 Security in Rust

### Memory Safety
Rust provides compile-time memory safety guarantees:
- **No null pointer dereferences**
- **No buffer overflows**
- **No use-after-free**
- **No data races**

### Input Validation
```rust
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct MealPlanRequest {
    cuisine: String,
    days: u8,
    servings: u8,
}

// Validation with serde
fn validate_request(request: &MealPlanRequest) -> Result<(), String> {
    if request.days == 0 || request.days > 30 {
        return Err("Days must be between 1 and 30".to_string());
    }
    if request.servings == 0 || request.servings > 20 {
        return Err("Servings must be between 1 and 20".to_string());
    }
    Ok(())
}
```

## 🚀 Performance Tips

### 1. Use `String` vs `&str`
```rust
// Use &str for function parameters (no allocation)
fn format_recipe(recipe: &Recipe) -> String {
    format!("Recipe: {}", recipe.name)
}

// Use String for owned data
let owned_string = "Hello".to_string();
```

### 2. Avoid Unnecessary Cloning
```rust
// Good: borrow when possible
fn process_recipes(recipes: &[Recipe]) {
    for recipe in recipes {
        // Process without cloning
    }
}

// Avoid: unnecessary cloning
fn process_recipes_bad(recipes: Vec<Recipe>) {
    for recipe in recipes.clone() { // Unnecessary clone!
        // Process
    }
}
```

### 3. Use Iterators
```rust
// Good: functional style with iterators
let total_ingredients: usize = recipes
    .iter()
    .map(|r| r.ingredients.len())
    .sum();

// Less efficient: imperative style
let mut total = 0;
for recipe in recipes {
    total += recipe.ingredients.len();
}
```

## 🐛 Common Issues and Solutions

### 1. Ownership Errors
```rust
// Error: value moved
let recipes = get_recipes();
let first = recipes[0]; // Error: recipes moved
let second = recipes[1]; // Error: recipes already moved

// Solution: borrow instead
let recipes = get_recipes();
let first = &recipes[0]; // Borrow
let second = &recipes[1]; // Borrow again
```

### 2. Lifetime Issues
```rust
// Error: lifetime mismatch
fn get_name(recipe: &Recipe) -> &str {
    &recipe.name // This works
}

// Error: returning reference to local data
fn get_name_bad() -> &str {
    let recipe = Recipe { name: "test".to_string() };
    &recipe.name // Error: recipe will be dropped
}
```

### 3. Async/Await
```rust
// Use tokio for async runtime
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Async code here
    Ok(())
}

// Async functions
async fn process_request() -> Result<String, String> {
    // Async operations
    Ok("result".to_string())
}
```

## 📚 Learning Resources

### Official Documentation
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)

### Tools and Extensions
- **VS Code**: Rust Analyzer extension
- **CLion**: Built-in Rust support
- **Vim/Neovim**: rust.vim plugin

### Community Resources
- [Rust Users Forum](https://users.rust-lang.org/)
- [r/rust](https://reddit.com/r/rust)
- [Rust Discord](https://discord.gg/rust-lang)

## 🔧 Development Tools

### Essential Cargo Commands
```bash
cargo new project_name     # Create new project
cargo init                 # Initialize in existing directory
cargo add dependency       # Add dependency
cargo update               # Update dependencies
cargo clean                # Clean build artifacts
cargo doc --open           # Generate and open documentation
```

### Useful Cargo Features
```bash
# Run with environment variables
RUST_LOG=debug cargo run

# Build with specific features
cargo build --features "feature_name"

# Run benchmarks
cargo bench

# Generate documentation
cargo doc --no-deps
```

## 🎯 Contributing Guidelines

### Code Style
- Follow `cargo fmt` formatting
- Use `cargo clippy` for linting
- Write comprehensive tests
- Document public APIs

### Commit Messages
```
feat: add Vietnamese cuisine support
fix: resolve memory leak in recipe processing
docs: update development guide
test: add integration tests for meal planning
```

### Pull Request Process
1. Fork the repository
2. Create a feature branch
3. Make changes with tests
4. Run security checks
5. Submit pull request

## 🆘 Getting Help

### When You're Stuck
1. **Check the compiler errors**: Rust has excellent error messages
2. **Read the documentation**: `cargo doc --open`
3. **Search Stack Overflow**: Many Rust questions answered
4. **Ask in Discord/Forum**: Friendly Rust community
5. **Check existing code**: Look at similar patterns in the codebase

### Common Error Messages
- **"borrow checker"**: Ownership/borrowing issue
- **"lifetime"**: Reference lifetime problem
- **"move"**: Value moved when you need it
- **"trait bound"**: Missing trait implementation

---

**Happy Rusting! 🦀** Remember: the Rust compiler is your friend, even when it seems strict. It's preventing bugs before they happen!
