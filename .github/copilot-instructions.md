# Syster - SysML v2 Parser and Tooling

## Project Overview

Syster is a comprehensive Rust-based parser and tooling suite for SysML v2 (Systems Modeling Language) and KerML (Kernel Modeling Language). The project provides:

- Core parsing library with grammar-based parsing using Pest
- Command-line analysis tool (syster-cli)
- Language Server Protocol implementation (syster-lsp) for IDE integration
- VS Code extension for SysML v2 development

**Status:** Alpha - Active development, APIs may change

## Technology Stack

- **Primary Language:** Rust (edition 2024, version 1.70+)
- **Parser:** Pest parser generator
- **LSP:** async-lsp library for Language Server Protocol
- **Formatter:** Rowan-based Concrete Syntax Tree (CST)
- **VS Code Extension:** TypeScript, Node.js
- **Build System:** Independent submodules, each with own Cargo.toml/package.json

## Repository Structure

```
syster/
├── base/                 # syster-base: parser, AST, semantic analysis
├── cli/                  # syster-cli: command-line tool
├── language-server/      # syster-lsp: Language Server Protocol implementation
├── language-client/      # VS Code LSP extension
├── modeller/             # VS Code modeller extension
├── viewer/               # VS Code viewer extension
├── diagram-core/         # TypeScript diagram utilities
├── diagram-ui/           # React diagram components
├── pipelines/            # CI/CD pipeline definitions
└── .github/              # GitHub configuration and instructions
```

## Core Architecture - THREE CRITICAL RULES

### 1. Three-Phase Pipeline (NEVER MIX PHASES)
- **Parse Layer** (Pest) → Grammar-based parsing only
- **Syntax Layer** (AST) → AST construction only, no cross-file knowledge
- **Semantic Layer** (Symbols + Graphs) → All cross-file resolution and validation

**⚠️ Critical:** Each phase has clear boundaries. Never add semantic logic to the parser, and never add parsing logic to semantic analysis.

### 2. Symbol Table is Global
- AST nodes are immutable and don't contain resolved references
- All symbols stored in centralized `SymbolTable`
- Use `QualifiedName` (e.g., `"Package::Element"`) for cross-references
- **Don't add back-references to AST nodes**

### 3. Relationship Graphs
- Never store relationships (specialization, typing, subsetting, etc.) in Symbol enum
- Use separate `RelationshipGraph` for all relationships
- Enables efficient queries and cycle detection

## Development Workflow

### Test-Driven Development (MANDATORY)
1. Write a failing test first
2. Run test to confirm it fails
3. Implement minimal code to pass
4. Run tests again to verify
5. Refactor while keeping tests green

### Incremental Development
- **One function at a time** - Complete full TDD cycle before moving on
- **Small changes** - Keep modifications minimal and focused (< 15 lines when possible)
- **Stop signals:** If modifying multiple files simultaneously or scope is growing, break down the task

### Build & Test Commands

```bash
# Build all crates
cargo build

# Run all tests
cargo test

# Run specific crate tests
cargo test -p syster-base
cargo test -p syster-cli
cargo test -p syster-lsp

# Complete validation pipeline
make run-guidelines  # Runs format + lint + test

# Format code
cargo fmt

# Run linter
cargo clippy --all-targets -- -D warnings
```

## Code Quality Standards

### Error Handling
- Use `Result<T, E>` for fallible operations
- Use `Option<T>` for optional values
- Avoid `.unwrap()` and `.expect()` in production code (tests are okay)
- Propagate errors with `?` operator

### Naming Conventions
- Types: `PascalCase` (e.g., `SymbolTable`, `SemanticError`)
- Functions/variables: `snake_case` (e.g., `resolve_qualified`, `symbol_name`)
- Constants: `SCREAMING_SNAKE_CASE` (e.g., `MAX_RECURSION_DEPTH`)

### Documentation
- Document all public APIs with `///` doc comments
- Include examples for complex functions
- Document error conditions and invariants
- Keep comments focused and valuable

## Domain-Specific Knowledge

### SysML v2 / KerML Terminology
- **Qualified Name:** Full path like `Package::Class::Feature`
- **Classifier:** KerML type that can have features (class, struct, etc.)
- **Definition:** SysML type (part def, port def, action def, etc.)
- **Usage:** SysML instance (part, port, action, etc.)
- **Feature:** Property or operation of a classifier
- **Specialization:** IS-A relationship (inheritance)
- **Typing:** INSTANCE-OF relationship
- **Subsetting:** REFINES relationship
- **Redefinition:** OVERRIDES relationship

### Import Resolution Algorithm
Three-pass resolution (order matters):
1. **Namespace imports** (`Package::*`) - order-independent
2. **Member imports** (`Package::Member`) - may depend on pass 1
3. **Recursive imports** (`Package::*::**`) - requires fully populated namespaces

## Essential Documentation

Before making changes, review:
- **[ARCHITECTURE.md](../ARCHITECTURE.md)** - System design, patterns, common operations
- **[docs/CONTRIBUTING.md](../docs/CONTRIBUTING.md)** - Development workflow, conventions
- **[docs/SYSML_PRIMER.md](../docs/SYSML_PRIMER.md)** - SysML v2 concepts for humans

## Common Pitfalls to Avoid

❌ **Don't:** Add semantic logic to AST nodes  
✅ **Do:** Keep AST immutable, use SymbolTable for semantic info

❌ **Don't:** Resolve imports while building symbol table  
✅ **Do:** Build symbol table first, then run three-pass import resolution

❌ **Don't:** Store relationships in Symbol enum  
✅ **Do:** Use RelationshipGraph for all relationships

❌ **Don't:** Create circular dependencies between modules  
✅ **Do:** Follow dependency flow: parser → language → semantic

❌ **Don't:** Implement features before writing tests  
✅ **Do:** Follow strict TDD - test first, then implement

## Security & Best Practices

- No hardcoded secrets or credentials
- Validate all external inputs
- Use type system to enforce invariants
- Follow Rust's ownership and borrowing rules
- Leverage the borrow checker for memory safety
- Keep modules focused and cohesive
- Make items private by default

## VS Code Extension

Located in `language-client/`:
- TypeScript-based extension
- Integrates with syster-lsp server
- Provides syntax highlighting, IntelliSense, formatting, and more
- Build with: `npm install && npm run compile`

## Standard Library

The `base/sysml.library/` directory contains the SysML v2 standard library files. These are loaded automatically by the workspace when needed.

## Getting Help

- Check [ARCHITECTURE.md](../ARCHITECTURE.md) for design patterns
- See [docs/CONTRIBUTING.md](../docs/CONTRIBUTING.md) for detailed guides
- Review [docs/SYSML_PRIMER.md](../docs/SYSML_PRIMER.md) for SysML concepts
