# Contributing to Syster

## Development Setup

### Prerequisites
- Rust 1.70+ (edition 2024)
- Node.js 18+ (for VS Code extension)
- VS Code (recommended)

### Build Commands

```bash
# Build everything
cargo build

# Run all tests
cargo test

# Run specific crate tests
cargo test -p syster-base
cargo test -p syster-cli
cargo test -p syster-lsp

# Format code
cargo fmt

# Lint (must pass CI)
cargo clippy --all-targets -- -D warnings

# Full validation (do this before PR)
cargo fmt && cargo clippy --all-targets -- -D warnings && cargo test
```

## Development Workflow

### Test-Driven Development (REQUIRED)

Every change must follow TDD:

1. **Write a failing test first**
   ```rust
   #[test]
   fn test_new_feature() {
       let result = new_function();
       assert_eq!(result, expected);
   }
   ```

2. **Run test, confirm it fails**
   ```bash
   cargo test test_new_feature
   ```

3. **Implement minimal code to pass**
   - Only write enough code to make the test pass
   - Don't add extra features

4. **Run test, confirm it passes**
   ```bash
   cargo test test_new_feature
   ```

5. **Refactor if needed**
   - Keep tests passing
   - Improve code quality

### Incremental Development

- **One function at a time** - Complete TDD cycle before moving on
- **Small changes** - Keep modifications < 15 lines when possible
- **Stop signals:**
  - Modifying multiple files at once? Break it down
  - Scope growing? Split into smaller tasks

## Code Style

### Naming Conventions

| Element | Convention | Example |
|---------|------------|---------|
| Types | `PascalCase` | `SymbolTable`, `SemanticError` |
| Functions | `snake_case` | `resolve_qualified`, `get_symbol` |
| Variables | `snake_case` | `symbol_name`, `file_path` |
| Constants | `SCREAMING_SNAKE_CASE` | `MAX_DEPTH` |
| Modules | `snake_case` | `symbol_table`, `resolver` |

### Error Handling

```rust
// ✅ Good - Use Result for fallible operations
fn load_file(path: &str) -> Result<String, IoError> {
    std::fs::read_to_string(path)
}

// ✅ Good - Propagate with ?
fn process_file(path: &str) -> Result<Model, Error> {
    let content = load_file(path)?;
    parse_content(&content)
}

// ❌ Bad - Don't unwrap in production code
fn load_file(path: &str) -> String {
    std::fs::read_to_string(path).unwrap()  // Will panic!
}
```

### Documentation

```rust
/// Resolves a qualified name to a symbol.
///
/// # Arguments
/// * `name` - Fully qualified name like "Package::Element"
///
/// # Returns
/// The resolved symbol, or None if not found.
///
/// # Example
/// ```
/// let symbol = resolver.resolve("MyPackage::MyClass");
/// ```
pub fn resolve(&self, name: &QualifiedName) -> Option<&Symbol> {
    // ...
}
```

## Architecture Rules

### Phase Separation

```
Parser (Pest)  →  Syntax (AST)  →  Semantic (Analysis)
```

**Never mix phases:**
- ❌ Don't add semantic logic to parser
- ❌ Don't add parsing logic to semantic
- ✅ Each phase has clear inputs/outputs

### Symbol Table

- **Global:** All symbols in centralized table
- **Immutable AST:** Nodes don't have resolved references
- **Qualified names:** Use full paths for cross-file refs

```rust
// ✅ Good - Look up in symbol table
let symbol = symbol_table.get("Package::Element");

// ❌ Bad - Storing resolved ref in AST
struct ClassNode {
    parent: Option<Symbol>,  // Don't do this!
}
```

### Relationships

- **Use graphs:** Don't store in Symbol enum
- **Located in:** `semantic/graphs/`

```rust
// ✅ Good - Separate relationship graph
let parents = relationship_graph.get_specializations("MyClass");

// ❌ Bad - Relationships in symbol
enum Symbol {
    Class { specializations: Vec<String> }  // Don't!
}
```

## Common Tasks

### Adding a New AST Node

1. **Define type** in `syntax/{kerml,sysml}/syntax/types.rs`:
   ```rust
   pub struct NewElement {
       pub name: Option<String>,
       pub span: Span,
   }
   ```

2. **Add to parent enum** in `syntax/{kerml,sysml}/syntax/enums.rs`:
   ```rust
   pub enum Element {
       // ...
       NewElement(NewElement),
   }
   ```

3. **Update populator** in `syntax/{kerml,sysml}/populator.rs`:
   ```rust
   fn populate_new_element(pair: Pair) -> NewElement {
       // ...
   }
   ```

4. **Add tests**

### Adding a Semantic Check

1. **Define error** in `semantic/types/errors.rs`:
   ```rust
   pub enum SemanticErrorKind {
       // ...
       NewValidationError { detail: String },
   }
   ```

2. **Implement check** in appropriate processor

3. **Add tests**

### Adding an LSP Feature

1. Implement in `syster-lsp`
2. Wire up in request handlers
3. Test with VS Code extension

## Pull Request Checklist

- [ ] Tests pass: `cargo test`
- [ ] Lints pass: `cargo clippy --all-targets -- -D warnings`
- [ ] Formatted: `cargo fmt`
- [ ] Documentation updated if needed
- [ ] No `.unwrap()` in production code
- [ ] Follows TDD process
- [ ] Small, focused changes

## Project Structure Reference

```
crates/
├── syster-base/src/
│   ├── parser/        # Pest grammars
│   ├── syntax/        # AST types
│   ├── semantic/      # Analysis
│   ├── core/          # Utilities
│   └── project/       # Workspace
├── syster-cli/        # CLI
└── syster-lsp/        # LSP server

editors/vscode-lsp/    # VS Code extension
packages/              # TypeScript packages
```
