# Contributing to Syster

Thank you for your interest in contributing to Syster! This guide will help you understand the development workflow and project conventions.

## Getting Started

### Prerequisites

- Rust 1.70+ (edition 2024)
- Git
- Familiarity with SysML v2 or willingness to learn

### Development Setup

```bash
# Clone the repository
git clone https://github.com/jade-codes/syster.git
cd syster

# Build the project
cargo build

# Run tests
cargo test

# Run with verbose output
cargo test -- --nocapture
```

## Development Workflow

### 1. Test-Driven Development (TDD) - MANDATORY

We follow strict TDD practices:

1. **Write a failing test first**
   ```rust
   #[test]
   fn test_concern_def_parsing() {
       let source = "concern def SafetyConcern;";
       let workspace = parse_workspace(source);
       assert!(workspace.symbol_table().lookup("SafetyConcern").is_some());
   }
   ```

2. **Run the test to confirm it fails**
   ```bash
   cargo test test_concern_def_parsing
   ```

3. **Implement minimal code to pass the test**

4. **Run tests again to verify success**

5. **Refactor while keeping tests green**

6. **Verify all guidelines pass **
   ```bash
   make run-guidelines
   ```

cargo test test_concern_def_parsing

### 2. Incremental Development - STRICT LIMITS

- **One function at a time:** Complete the full TDD cycle for one function before moving to the next
- **Small, focused changes:** Each change should be < 10-15 lines when possible
- **Single responsibility:** Each function/module should do one thing well

**⚠️ STOP signals:**
- Modifying multiple files simultaneously
- Changes spanning > 15 lines in a single function
- Scope growing beyond the original task

If you hit a STOP signal, break down the task into smaller pieces.

## Code Conventions

### Rust Style

Follow the project's `rustfmt.toml` configuration:

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt -- --check
```

### Error Handling

```rust
// ✅ Good: Use Result for fallible operations
pub fn resolve(&self, name: &str) -> Result<&Symbol, SemanticError> {
    self.symbol_table.lookup(name)
        .ok_or(SemanticError::undefined_symbol(name))
}

// ✅ Good: Use Option for optional values
pub fn get_parent(&self, symbol: &Symbol) -> Option<&Symbol> {
    // ...
}

// ❌ Bad: Avoid unwrap/expect in library code
let symbol = self.symbol_table.lookup(name).unwrap(); // Don't do this

// ✅ OK in tests
#[test]
fn test_something() {
    let symbol = workspace.lookup("Test").unwrap(); // OK in tests
}

// ❌ Bad: Loops in tests that assert on each iteration
#[test]
fn test_with_loop() {
    for item in items {
        assert!(item.is_valid()); // Fails on first invalid, hides rest
    }
}

// ✅ Good: Collect failures and assert once with full context
#[test]
fn test_with_filter() {
    let invalid: Vec<_> = items.iter().filter(|i| !i.is_valid()).collect();
    assert!(invalid.is_empty(), "Found invalid items: {:?}", invalid);
}
```

### Documentation

**Module-level documentation:**
```rust
//! # Import Resolver
//!
//! Resolves import statements across files in a workspace.
//!
//! ## Algorithm
//! Import resolution happens in three passes:
//! 1. Namespace imports (`Package::*`)
//! 2. Member imports (`Package::Member`)
//! 3. Recursive imports (`Package::*::**`)
//!
//! ## Key Invariants
//! - Symbols must exist before being imported
//! - Circular imports are detected and reported as errors
```

**Function documentation:**
```rust
/// Resolves a qualified name to its symbol definition.
///
/// # Arguments
///
/// * `qualified_name` - Fully qualified name like "Package::Subpackage::Element"
///
/// # Returns
///
/// The symbol if found, or `None` if the qualified name doesn't exist.
///
/// # Examples
///
/// ```
/// let resolver = Resolver::new(&symbol_table);
/// let symbol = resolver.resolve_qualified("MyPackage::MyClass");
/// ```
pub fn resolve_qualified(&self, qualified_name: &str) -> Option<&Symbol> {
    // Implementation...
}
```

**When to document:**
- All public APIs (functions, structs, enums)
- Complex algorithms or non-obvious logic
- Error conditions and panics
- Invariants and assumptions

**When NOT to document:**
- Self-explanatory private functions
- Trivial getters/setters
- Test functions (unless the test is complex)

### Naming Conventions

```rust
// ✅ Types: PascalCase
pub struct SymbolTable { }
pub enum SemanticErrorKind { }

// ✅ Functions/methods: snake_case
pub fn resolve_qualified(&self, name: &str) { }

// ✅ Constants: SCREAMING_SNAKE_CASE
pub const MAX_RECURSION_DEPTH: usize = 100;

// ✅ Type aliases with documentation
/// Fully qualified name like "Package::Subpackage::Element"
pub type QualifiedName = String;
```

## Adding New Features

See [ARCHITECTURE.md](../ARCHITECTURE.md) for detailed guides on:
- Adding a new SysML element type
- Adding a new semantic check
- Understanding the three-phase pipeline

### Quick Checklist

When adding a new SysML/KerML construct:

- [ ] Update grammar file (`kerml.pest` or `sysml.pest`)
- [ ] Define AST struct in `syntax/types.rs`
- [ ] Implement `FromPest` trait in `syntax/ast.rs`
- [ ] Add to parent enum in `syntax/enums.rs`
- [ ] Update populator to handle new construct
- [ ] Write syntax tests in `syntax/tests.rs`
- [ ] Write semantic tests in `tests/semantic/`
- [ ] Update documentation

## Testing Guidelines

### Test Organization

```
tests/
├── tests.rs                    # Integration test runner
├── parser/
│   ├── kerml_tests.rs         # KerML parsing tests
│   └── sysml_tests.rs         # SysML parsing tests
└── semantic/
    ├── import_tests.rs        # Import resolution tests
    ├── cross_file_tests.rs    # Multi-file tests
    └── sysml_graph_tests.rs   # Relationship graph tests
```

### Test Coverage Requirements

- **All new features must have tests**
- **Both positive and negative cases:**
  ```rust
  #[test]
  fn test_valid_namespace_import() {
      // Test that valid imports work
  }

  #[test]
  fn test_invalid_namespace_import() {
      // Test that invalid imports produce appropriate errors
  }
  ```
- **Edge cases:**
  - Empty inputs
  - Very large inputs
  - Circular references
  - Missing dependencies

### Test Naming

```rust
// ✅ Good: Descriptive test names
#[test]
fn test_namespace_import_makes_members_visible() { }

#[test]
fn test_circular_specialization_produces_error() { }

// ❌ Bad: Vague test names
#[test]
fn test_imports() { }

#[test]
fn test_error() { }
```

### Using Test Fixtures

```rust
use rstest::rstest;

#[rstest]
#[case("part def Vehicle;", "Vehicle")]
#[case("port def DataPort;", "DataPort")]
#[case("action def Move;", "Move")]
fn test_definition_parsing(#[case] source: &str, #[case] expected_name: &str) {
    let workspace = parse_workspace(source);
    assert!(workspace.symbol_table().lookup(expected_name).is_some());
}
```

## Pull Request Process

### Before Submitting

1. **All tests pass:** `cargo test`
2. **Code is formatted:** `cargo fmt`
3. **No clippy warnings:** `cargo clippy`
4. **Documentation is updated:** README.md, doc comments, ARCHITECTURE.md
5. **Commit messages are clear:**
   ```
   Add support for concern definitions

   - Update SysML grammar with concern_def rule
   - Implement ConcernDef AST node
   - Add symbol table population for concerns
   - Include tests for valid and invalid concerns

   Closes #123
   ```

### PR Template

```markdown
## Description
Brief description of what this PR does.

## Changes
- Change 1
- Change 2

## Testing
- [ ] Added unit tests
- [ ] Added integration tests
- [ ] Existing tests pass

## Documentation
- [ ] Updated doc comments
- [ ] Updated ARCHITECTURE.md if needed
- [ ] Updated README.md if needed

## Related Issues
Closes #123
```

### Review Process

- PRs require at least one approval
- CI must pass (tests, formatting, clippy)
- Address all review comments before merging
- Keep PR scope focused (one feature/fix per PR)

## Performance Considerations

### When to Profile

Profile before optimizing:
```bash
# Benchmark parsing
cargo bench --bench parse_benchmark

# Profile with flamegraph
cargo install flamegraph
cargo flamegraph --bench parse_benchmark
```

### Common Performance Pitfalls

```rust
// ❌ Bad: Cloning unnecessarily
for name in names {
    let qname = format!("{}::{}", prefix, name.clone()); // Unnecessary clone
}

// ✅ Good: Borrow instead
for name in &names {
    let qname = format!("{}::{}", prefix, name);
}

// ❌ Bad: String allocation in hot loop
for i in 0..1000 {
    let key = format!("key_{}", i); // Repeated allocations
}

// ✅ Good: Reuse buffer
let mut key = String::with_capacity(20);
for i in 0..1000 {
    key.clear();
    write!(&mut key, "key_{}", i).unwrap();
}
```

## Debugging Tips

### Enable Logging

```rust
// Add to Cargo.toml
[dependencies]
log = "0.4"
env_logger = "0.10"

// Use in code
use log::{debug, info, warn, error};

debug!("Resolving import: {}", import_path);
```

```bash
# Run with logging
RUST_LOG=debug cargo test
```

### Common Issues

**"Undefined symbol" errors:**
- Check that symbols are added to table before imports are resolved
- Verify qualified names are constructed correctly
- Ensure source file path is set correctly

**Grammar parsing failures:**
- Remember Pest is PEG (order matters)
- Check for missing `WHITESPACE` rules
- Use online Pest debugger: https://pest.rs/#editor

**Slow tests:**
- Profile with `cargo test --release`
- Check for O(n²) algorithms in hot paths
- Consider parallel test execution

## Getting Help

- **Documentation:** Read [ARCHITECTURE.md](../ARCHITECTURE.md) first
- **Code questions:** Open a discussion on GitHub
- **Bugs:** Open an issue with minimal reproduction
- **Features:** Open an issue to discuss before implementing

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
