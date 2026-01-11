# Syster

**Status: Alpha** - Active development, APIs may change

A Rust-based parser and tooling for SysML v2 (Systems Modeling Language) and KerML (Kernel Modeling Language).

## Project Structure

Syster is organized as a Cargo workspace with three crates:

- **syster-base** - Core library with parser, AST, and semantic analysis
- **syster-cli** - Command-line tool for analyzing SysML/KerML files
- **syster-lsp** - Language Server Protocol implementation with full IDE support

## Documentation

- **[ARCHITECTURE.md](ARCHITECTURE.md)** ⭐ Core patterns and rules (start here)
- **[docs/CONTRIBUTING.md](docs/CONTRIBUTING.md)** - Development workflow
- **[docs/SYSML_PRIMER.md](docs/SYSML_PRIMER.md)** - SysML v2 concepts for humans

## Overview

Syster provides a comprehensive implementation of parsers and Abstract Syntax Trees (AST) for both SysML v2 and its foundation language KerML. The project follows a three-layer architecture:

1. **Parse Layer** - Grammar-based parsing using Pest
2. **Syntax Layer** - Language-specific Abstract Syntax Trees (AST)
3. **Model Layer** - Semantic model representation (planned)

## Features

### Semantic Analysis
- **Symbol Table**: Cross-file symbol resolution with qualified names
- **Import Resolution**: 
  - Namespace imports (`import Package::*`)
  - Member imports (`import Package::Member`)
  - Recursive imports (`import Package::*::**`)
  - Alias support (`alias NewName for Target`)
- **Relationship Graph**: Tracks specialization, typing, subsetting, redefinition, and satisfaction
- **Workspace Management**: Multi-file projects with shared symbol table
- **Standard Library Support**: Optional stdlib loading via `Workspace::with_stdlib()`

### KerML Support
- Complete grammar implementation for Kernel Modeling Language
- AST representations for all classifier types:
  - Type, Classifier, DataType, Class, Structure, Behavior, Function
  - Association, AssociationStructure, Metaclass
- Feature support with modifiers (readonly, derived, abstract)
- Direction qualifiers (in, out, inout)
- Package and namespace management
- Import and annotation support
- Comprehensive test coverage (28 tests)

### SysML v2 Support
- Grammar implementation for Systems Modeling Language v2
- AST representations for all definition types:
  - Part, Port, Action, Item, Attribute, Requirement
  - Concern, Case, AnalysisCase, VerificationCase, UseCase
  - View, Viewpoint, Rendering
- Usage types for all major elements
- Package, comment, and import support
- Visitor pattern for AST traversal
- Comprehensive test coverage (29 tests)

### Code Formatter
- **Rowan-based**: Uses Concrete Syntax Tree (CST) for lossless formatting
- **Comment Preservation**: Line comments, block comments, and doc comments are preserved
- **Whitespace Normalization**: Collapses multiple spaces, normalizes indentation
- **Configurable**: Supports `tab_size`, `insert_spaces`, and `print_width` settings
- **Async with Cancellation**: Can be cancelled mid-operation for responsive editing
- **Idempotent**: Formatting is stable (formatting twice produces same result)
- **75 unit tests**: Covers all SysML constructs, edge cases, and idempotency

### Language Server Protocol (LSP)
Full IDE integration support with:
- **Navigation**:
  - Go to Definition
  - Find References
  - Document Symbols (outline view)
  - Hover information with symbol details
- **Editing**:
  - Document Formatting (preserves comments, normalizes whitespace)
  - Rename Symbol (with prepare rename validation)
  - Completion (context-aware suggestions)
- **Analysis**:
  - Semantic Tokens (syntax highlighting for definitions, usages, types, and properties)
  - Feature chain highlighting (each part of `a.b.c` highlighted individually)
  - Inlay Hints (type annotations)
  - Diagnostics (parse errors)
- **Code Structure**:
  - Folding Ranges (collapse blocks)
  - Selection Ranges (smart select expand/shrink)
- **Async Architecture**: Uses async-lsp for proper request cancellation and ordering
- **Standard Library**: Optional stdlib loading with configurable paths
- **Incremental Updates**: Efficient document synchronization

## Architecture

```
crates/
├── syster-base/             # Core library
│   ├── src/
│   │   ├── parser/         # Pest grammars
│   │   │   ├── kerml.pest  # KerML grammar
│   │   │   └── sysml.pest  # SysML grammar
│   │   ├── syntax/         # AST and formatter
│   │   │   ├── kerml/      # KerML AST nodes
│   │   │   ├── sysml/      # SysML AST nodes
│   │   │   ├── formatter/  # Rowan-based code formatter
│   │   │   │   ├── lexer.rs    # Token stream generation
│   │   │   │   ├── options.rs  # Formatting configuration
│   │   │   │   └── syntax_kind.rs  # Token types
│   │   │   ├── file.rs     # File abstraction
│   │   │   └── parser.rs   # AST construction
│   │   ├── semantic/       # Cross-file analysis
│   │   │   ├── symbol_table.rs  # Global symbol registry
│   │   │   ├── graph.rs         # Relationship graphs
│   │   │   ├── resolver.rs      # Name resolution
│   │   │   ├── analyzer.rs      # Validation passes
│   │   │   └── workspace.rs     # Multi-file coordination
│   │   ├── project/        # File loading
│   │   │   ├── workspace_loader.rs  # User files
│   │   │   └── stdlib_loader.rs     # Standard library
│   │   └── core/           # Shared traits and utilities
│   └── sysml.library/      # Standard library files
├── syster-cli/              # Command-line tool
│   ├── src/
│   │   ├── main.rs         # CLI entry point
│   │   └── lib.rs          # Testable analysis logic
│   └── tests/
│       └── cli_tests.rs    # Integration tests
└── syster-lsp/              # LSP server with full feature support
    ├── src/
    │   ├── main.rs          # async-lsp server setup
    │   ├── server.rs        # LSP router and lifecycle
    │   └── server/          # Feature implementations
    │       ├── core.rs           # Server capabilities
    │       ├── document.rs       # Document lifecycle
    │       ├── diagnostics.rs    # Error reporting
    │       ├── hover.rs          # Hover information
    │       ├── definition.rs     # Go to definition
    │       ├── references.rs     # Find all references
    │       ├── rename.rs         # Symbol renaming
    │       ├── completion.rs     # Code completion
    │       ├── document_symbols.rs  # Document outline
    │       ├── semantic_tokens.rs   # Syntax highlighting
    │       ├── inlay_hints.rs    # Inline type hints
    │       ├── formatting.rs     # Code formatting (Rowan)
    │       ├── folding_range.rs  # Code folding
    │       ├── selection_range.rs # Smart selection
    │       ├── position.rs       # Position utilities
    │       └── helpers.rs        # Common utilities
    └── tests/
        └── integration_tests.rs  # LSP protocol tests
```

## Building

```bash
# Build all crates
cargo build

# Build in release mode (for production use)
cargo build --release

# Build specific crate
cargo build -p syster-base
cargo build -p syster-cli
cargo build -p syster-lsp
```

## VS Code Extension

The LSP server integrates with VS Code for a full IDE experience:

```bash
# Build and install the extension
cd editors/vscode
npm install
npm run compile
```

Features in VS Code:
- Syntax highlighting with semantic tokens
- IntelliSense with hover and completion
- Go to definition and find all references
- Symbol outline and document symbols
- Code formatting with comment preservation
- Smart rename with validation
- Inlay hints for types
- Code folding and smart selection
- Real-time diagnostics

Configure stdlib path in VS Code settings:
```json
{
  "sysml.stdlibEnabled": true,
  "sysml.stdlibPath": "/path/to/sysml.library"
}
```

## Command-Line Usage

The `syster` CLI tool analyzes SysML and KerML files:

```bash
# Analyze a single file
syster my_model.sysml

# Analyze a directory (recursive)
syster src/models/

# Verbose output
syster --verbose my_model.sysml

# Skip standard library
syster --no-stdlib my_model.sysml

# Use custom standard library
syster --stdlib-path ./custom_stdlib my_model.sysml
```

### CLI Options

- `--verbose, -v` - Enable verbose output
- `--no-stdlib` - Skip loading the standard library
- `--stdlib-path <PATH>` - Path to custom standard library (default: `sysml.library`)

## Library Usage

### Basic Usage

```rust
use syster::parser::{SysMLParser, sysml::Rule};
use syster::language::sysml::syntax::SysMLFile;
use syster::semantic::Workspace;
use from_pest::FromPest;
use pest::Parser;

// Parse a SysML file
let source = r#"
    package MyPackage {
        part def Vehicle;
    }
"#;

let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
let file = SysMLFile::from_pest(&mut pairs).unwrap();

// Create a workspace (without stdlib for tests, with stdlib for real projects)
let mut workspace = Workspace::new();
workspace.add_file("example.sysml".into(), file);
workspace.populate_all().unwrap();

// Look up symbols
let vehicle = workspace.symbol_table().lookup_qualified("MyPackage::Vehicle");
```

### Using the Standard Library

For real projects, load the standard library once at startup, then load project files on demand:

```rust
use syster::semantic::Workspace;
use syster::project::{StdLibLoader, WorkspaceLoader};

// Create workspace
let mut workspace = Workspace::new();

// Load standard library once at startup (from sysml.library/)
let stdlib_loader = StdLibLoader::new();
stdlib_loader.load(&mut workspace).unwrap();

// Load project files on demand
let workspace_loader = WorkspaceLoader::new();
workspace_loader.load_file("my_model.sysml", &mut workspace).unwrap();
// Or load an entire directory
workspace_loader.load_directory("src/models", &mut workspace).unwrap();

// Populate symbols and resolve references
workspace.populate_all().unwrap();

// Now query the symbol table
let symbol = workspace.symbol_table().lookup_qualified("MyPackage::MyElement");
```

**StdLibLoader** - Load once at startup from `sysml.library/`  
**WorkspaceLoader** - Load project files on demand as they're opened or referenced

### Working with Imports

```rust
let source = r#"
    package Base {
        part def Vehicle;
    }
    
    package Derived {
        // Namespace import - imports all members
        public import Base::*;
        
        // Member import - imports specific member
        import Base::Vehicle;
        
        // Use imported types
        part myCar : Vehicle;
    }
"#;
```

## Testing

Run all tests:
```bash
cargo test
```

Run tests for specific crate:
```bash
cargo test -p syster-base
cargo test -p syster-cli
cargo test -p syster-lsp
```

Run tests for specific module:
```bash
cargo test -p syster-base language::kerml::syntax::tests
cargo test -p syster-base language::sysml::syntax::tests
cargo test -p syster-base semantic
```

## Development Guidelines

See [docs/CONTRIBUTING.md](docs/CONTRIBUTING.md) for full guidelines. Key points:

- **Test-Driven Development**: Write tests first, then implement
- **One function at a time**: Complete full TDD cycle before moving to next function
- **Format & Lint**: Run `make run-guidelines` before committing
- **Small commits**: Commit after each completed todo with descriptive messages

### Development Commands

```bash
# Run complete validation (format, lint, test)
make run-guidelines

# Format code
make fmt

# Run linter
make lint

# Run all tests
make test

# Build project
make build

# Install syster-lsp and VS Code extension
make install

# Clean build artifacts
make clean
```

### Key Design Patterns

1. **Language-Specific AST**: Each language (KerML, SysML) has its own syntax module with specific node types
2. **Shared Core**: Common traits and patterns are defined in `src/core/`
3. **Macro-Based FromPest**: Uses `impl_from_pest!` macro to reduce boilerplate in CST→AST conversion
4. **Recursive Name Finding**: Handles various grammar patterns for extracting identifications
5. **Async-LSP**: Uses async-lsp library for proper request cancellation and synchronous notifications

### Grammar Rules

- Grammar rules use named markers (e.g., `abstract_marker`) instead of string literals to expose them in the parse tree
- This allows AST layer to properly extract flags like `is_abstract`, `is_readonly`, etc.

## Contributing

When adding new AST node types:

1. Define the type in `types.rs`
2. Add enum variants in `enums.rs` if needed
3. Implement `FromPest` in `ast.rs` using the `impl_from_pest!` macro
4. Add comprehensive unit tests in `tests.rs`
5. Update the `Element` enum to handle the new type

## License

[Add license information]

## References

- [SysML v2 Specification](https://www.omg.org/spec/SysML)
- [Pest Parser](https://pest.rs/)
