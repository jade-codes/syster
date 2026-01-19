# Syster AI Context

> **TL;DR:** Rust SysML v2 parser. Three crates. Test-first development. Don't mix parsing with semantic analysis.

## What Is This?

Syster is a **Rust parser and tooling suite for SysML v2** (Systems Modeling Language) with:
- `syster-base` - Core parser/AST/semantic analysis library
- `syster-cli` - Command-line tool
- `syster-lsp` - Language Server Protocol for IDE integration
- `editors/vscode-lsp` - VS Code extension (TypeScript)

## Quick Commands

```bash
cargo build                    # Build all
cargo test                     # Test all
cargo test -p syster-base      # Test core library only
cargo clippy --all-targets -- -D warnings  # Lint
cargo fmt                      # Format
```

## Directory Layout

```
crates/
├── syster-base/src/
│   ├── parser/        # Pest grammar files (.pest)
│   ├── syntax/        # AST types (kerml/, sysml/)
│   ├── semantic/      # Symbol table, resolver, workspace
│   ├── core/          # Shared utilities, errors, spans
│   └── project/       # Workspace/project loading
├── syster-cli/        # CLI tool
└── syster-lsp/        # LSP server

editors/vscode-lsp/    # VS Code extension (TypeScript)
packages/              # TypeScript diagram packages
```

## THE THREE RULES (Don't Violate These)

### 1. Three-Phase Pipeline - NEVER MIX
```
Parse (Pest grammar) → Syntax (AST) → Semantic (SymbolTable + Graphs)
```
- **Parser:** Only grammar → parse tree
- **Syntax:** Only AST construction, no cross-file knowledge
- **Semantic:** All cross-file resolution, validation, relationships

### 2. Symbol Table is Global
- AST nodes are **immutable** - no back-references
- All symbols go in centralized `SymbolTable`
- Use `QualifiedName` (like `"Package::Element"`) for references

### 3. Relationships Use Graphs
- **Never** store specialization/typing/subsetting in Symbol enum
- Use `RelationshipGraph` for all relationships
- Located in `semantic/graphs/`

## Type Aliases (Use These)

```rust
pub type QualifiedName = String;   // "Package::Class::Feature"
pub type SimpleName = String;      // "Feature"
pub type ScopeId = usize;
pub type SourceFilePath = String;
```

## Import Resolution (Three Passes)

1. **Namespace imports** (`Package::*`) - order-independent
2. **Member imports** (`Package::Member`) - depends on pass 1
3. **Recursive imports** (`Package::*::**`) - needs full namespaces

## SysML/KerML Terminology

| Term | Meaning |
|------|---------|
| **Qualified Name** | Full path: `Package::Class::Feature` |
| **Classifier** | KerML type with features (class, struct) |
| **Definition** | SysML type: part def, port def, action def |
| **Usage** | SysML instance: part, port, action |
| **Feature** | Property/operation of a classifier |
| **Specialization** | IS-A (inheritance) |
| **Typing** | INSTANCE-OF |
| **Subsetting** | REFINES |
| **Redefinition** | OVERRIDES |

## Common Operations

### Adding a New SysML Element
1. Grammar: `src/parser/sysml.pest`
2. AST struct: `src/syntax/sysml/`
3. Populator: `src/semantic/adapters/`
4. Tests

### Adding a Semantic Check
1. Error kind: `src/semantic/types/`
2. Implement in analyzer
3. Tests

## Test-Driven Development (REQUIRED)

1. Write failing test first
2. Run test, confirm it fails
3. Implement minimal code
4. Run test, confirm it passes
5. Refactor

**One function at a time. Small changes. < 15 lines when possible.**

## Don't Do These Things

❌ Add semantic logic to AST nodes  
❌ Resolve imports while building symbol table  
❌ Store relationships in Symbol enum  
❌ Create circular module dependencies  
❌ Skip writing tests first  

## Do These Things

✅ Keep AST immutable  
✅ Build symbol table first, then resolve imports  
✅ Use RelationshipGraph for relationships  
✅ Follow: parser → syntax → semantic  
✅ Write test first, then implement  

## Release Checklist

Before releasing ANY package/crate/extension:

- [ ] **Bump version** in `package.json` or `Cargo.toml`
- [ ] **Update CHANGELOG.md** with changes
- [ ] **Swap local packages** - Replace any `path = "../"` or `file:../` references with published versions
- [ ] **Run `make run-guidelines`** - Must pass format + lint + test

## Key Files

- `crates/syster-base/src/semantic/symbol_table.rs` - Global symbol registry
- `crates/syster-base/src/semantic/resolver.rs` - Name resolution
- `crates/syster-base/src/semantic/workspace.rs` - Multi-file coordination
- `crates/syster-base/src/semantic/graphs.rs` - Relationship graphs
- `crates/syster-base/src/syntax/kerml/` - KerML AST
- `crates/syster-base/src/syntax/sysml/` - SysML AST
- `crates/syster-base/src/parser/kerml.pest` - KerML grammar
- `crates/syster-base/src/parser/sysml.pest` - SysML grammar
