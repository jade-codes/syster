# Syster Architecture

## Project Overview

Syster is a **SysML v2 / KerML tooling suite** built in Rust with TypeScript extensions. It provides parsing, semantic analysis, and IDE features for systems modeling.

```
┌─────────────────────────────────────────────────────────────────────┐
│                         VS Code Extensions                          │
│  ┌─────────────┐  ┌─────────────────┐  ┌─────────────────────────┐  │
│  │ vscode-lsp  │  │ vscode-viewer   │  │    vscode-modeller      │  │
│  │ (Language)  │  │ (Diagrams)      │  │    (Editing)            │  │
│  └──────┬──────┘  └────────┬────────┘  └────────────┬────────────┘  │
└─────────┼──────────────────┼────────────────────────┼───────────────┘
          │ LSP              │ Commands               │ Commands
          ▼                  ▼                        ▼
┌─────────────────────────────────────────────────────────────────────┐
│                       syster-lsp (Binary)                           │
│                   Language Server Protocol                          │
└─────────────────────────────┬───────────────────────────────────────┘
                              │
┌─────────────────────────────┴───────────────────────────────────────┐
│                       syster-base (Library)                         │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌───────────────────┐   │
│  │  Parser  │─▶│  Syntax  │─▶│ Semantic │  │      Project      │   │
│  │  (Pest)  │  │  (AST)   │  │ (Analysis)│  │ (Workspace/Stdlib)│   │
│  └──────────┘  └──────────┘  └──────────┘  └───────────────────┘   │
└─────────────────────────────────────────────────────────────────────┘
```

## Repository Structure

```
syster/
├── crates/
│   ├── syster-base/           # Core library (THE IMPORTANT ONE)
│   │   ├── src/
│   │   │   ├── parser/        # Pest grammars (.pest files)
│   │   │   ├── syntax/        # AST types (KerML + SysML)
│   │   │   ├── semantic/      # Symbol table, resolver, workspace
│   │   │   ├── core/          # Shared utilities (spans, errors)
│   │   │   └── project/       # File loading, stdlib
│   │   └── sysml.library/     # SysML v2 standard library
│   │
│   ├── syster-cli/            # Command-line tool
│   │   └── src/               # Analysis commands
│   │
│   └── syster-lsp/            # Language server
│       └── crates/syster-lsp/src/
│           ├── server/        # LSP request handlers
│           └── main.rs        # Server entry point
│
├── editors/
│   ├── vscode-lsp/            # Main VS Code extension (TypeScript)
│   ├── vscode-viewer/         # Diagram viewer extension
│   └── vscode-modeller/       # Diagram editor extension
│
└── packages/
    ├── diagram-core/          # Diagram types & layout (TypeScript)
    └── diagram-ui/            # React diagram components
```

## The Three-Phase Pipeline

This is the **most important concept**. Data flows through three distinct phases:

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────────┐
│   1. PARSER     │───▶│   2. SYNTAX     │───▶│    3. SEMANTIC      │
│   (Pest)        │    │   (AST)         │    │    (Analysis)       │
└─────────────────┘    └─────────────────┘    └─────────────────────┘
     Text → Pairs       Pairs → AST Nodes      AST → SymbolTable
     
     ONLY grammar       ONLY structure         ALL cross-file logic
     NO semantics       NO cross-file refs     ALL relationships
```

### Phase 1: Parser (`src/parser/`)

**Input:** Source code text  
**Output:** Pest parse tree (Pairs with spans)

| File | Purpose |
|------|---------|
| `kerml.pest` | KerML grammar rules |
| `sysml.pest` | SysML grammar (extends KerML) |
| `kerml_expressions.pest` | Expression grammar |
| `kerml.rs`, `sysml.rs` | Parser entry points |

**Rules:**
- Pure grammar - NO semantic logic
- Produces parse tree with source positions
- Errors = syntax errors only

### Phase 2: Syntax (`src/syntax/`)

**Input:** Pest parse tree  
**Output:** Typed AST nodes

```
syntax/
├── kerml/              # KerML language
│   ├── ast/            # AST node types
│   └── populator.rs    # Pairs → AST
├── sysml/              # SysML language  
│   ├── ast/            # AST node types
│   └── populator.rs    # Pairs → AST
├── file.rs             # SyntaxFile (SysML | KerML)
└── formatter/          # Code formatting (CST-based)
```

**Rules:**
- AST nodes are **immutable**
- No cross-file knowledge
- No resolved references (just string names)
- Each node has source span

### Phase 3: Semantic (`src/semantic/`)

**Input:** AST + all project files  
**Output:** Queryable model (SymbolTable + Graphs)

```
semantic/
├── symbol_table/       # Global symbol registry
│   ├── table.rs        # SymbolTable struct
│   ├── symbol.rs       # Symbol enum
│   ├── scope.rs        # Scope + Import tracking
│   └── lookup.rs       # Name resolution helpers
│
├── resolver/           # Import resolution
│   └── mod.rs          # Three-pass algorithm
│
├── graphs/             # Relationship storage
│   └── reference_index.rs  # Cross-references
│
├── workspace/          # Multi-file coordination
│   ├── core.rs         # Workspace struct
│   ├── file_manager.rs # File tracking
│   ├── populator.rs    # AST → SymbolTable
│   └── events.rs       # Change notifications
│
├── adapters/           # AST → Semantic bridges
├── processors/         # Analysis passes
└── types/              # Errors, diagnostics
```

**Rules:**
- All cross-file logic lives here
- SymbolTable is the source of truth
- Relationships stored in graphs (NOT in Symbol enum)
- Three-pass import resolution

## Key Data Structures

### SymbolTable

Central registry of all named elements:

```rust
pub struct SymbolTable {
    symbols: HashMap<SymbolId, Symbol>,
    scopes: Vec<Scope>,
    // name → symbol mapping
}

// Lookup by qualified name: "Package::Subpackage::Element"
let symbol = symbol_table.lookup("Vehicle::Engine");
```

**Key principle:** AST nodes don't contain resolved references. Always look up in SymbolTable.

### Workspace

Manages multi-file projects:

```rust
pub struct Workspace<F: ParsedFile> {
    files: HashMap<PathBuf, WorkspaceFile<F>>,
    symbol_table: SymbolTable,
    // ... dependency tracking
}

// Usage
let mut workspace = Workspace::new();
workspace.add_file(path, syntax_file);
workspace.populate_all()?;  // Build symbol table
```

### SyntaxFile

Union type for parsed files:

```rust
pub enum SyntaxFile {
    SysML(SysMLFile),
    KerML(KerMLFile),
}
```

## LSP Architecture

The language server (`syster-lsp`) provides IDE features:

```
syster-lsp/src/server/
├── core.rs              # LspServer struct, lifecycle
├── document.rs          # Text document sync
├── completion.rs        # Autocomplete
├── definition.rs        # Go to definition
├── references.rs        # Find all references
├── hover.rs             # Hover information
├── diagnostics.rs       # Error reporting
├── semantic_tokens.rs   # Syntax highlighting
├── formatting.rs        # Code formatting
├── rename.rs            # Symbol rename
├── code_lens.rs         # Inline actions
└── document_symbols.rs  # Outline view
```

Each handler:
1. Receives LSP request
2. Queries Workspace/SymbolTable
3. Transforms to LSP response

## VS Code Extensions

### vscode-lsp (Main Extension)
- Language registration (`.sysml`, `.kerml`)
- LSP client connecting to syster-lsp
- Syntax highlighting (TextMate grammars)

### vscode-viewer
- Read-only diagram visualization
- Uses `diagram-core` for layout

### vscode-modeller  
- Interactive diagram editing
- Uses `diagram-ui` React components

## Data Flow Example

User opens `Vehicle.sysml`:

```
1. VS Code Extension
   └─▶ Sends textDocument/didOpen to LSP

2. syster-lsp
   └─▶ Reads file content
   └─▶ Calls syster-base parser

3. Parser (syster-base)
   └─▶ sysml.pest grammar
   └─▶ Returns Pest Pairs

4. Syntax (syster-base)
   └─▶ populator.rs
   └─▶ Returns SysMLFile AST

5. Semantic (syster-base)
   └─▶ workspace.add_file()
   └─▶ workspace.populate_all()
   └─▶ Updates SymbolTable

6. syster-lsp
   └─▶ Runs diagnostics
   └─▶ Sends publishDiagnostics to VS Code
```

## Import Resolution Algorithm

Three passes (order matters):

| Pass | Pattern | Description |
|------|---------|-------------|
| 1 | `Package::*` | Namespace imports - all direct children |
| 2 | `Package::Member` | Member imports - specific elements |
| 3 | `Package::*::**` | Recursive imports - all descendants |

Pass 2 may depend on Pass 1 results. Pass 3 requires fully populated namespaces.

## Crate Dependencies

```
syster-base (lib)
    │
    ├──▶ syster-cli (bin)
    │
    └──▶ syster-lsp (bin)
              │
              └──▶ editors/vscode-lsp (TypeScript, npm)

packages/diagram-core (TypeScript)
    │
    └──▶ packages/diagram-ui
              │
              └──▶ editors/vscode-viewer
              └──▶ editors/vscode-modeller
```

## Type Aliases

Defined in `semantic/mod.rs`:

```rust
pub type QualifiedName = String;    // "A::B::C"
pub type SimpleName = String;       // "C"  
pub type ScopeId = usize;           // Scope index
pub type SourceFilePath = String;   // File path
```

## Critical Invariants

1. **Phase separation** - Parser has no semantic logic, syntax has no cross-file knowledge
2. **Immutable AST** - Syntax nodes never mutated after creation
3. **Centralized symbols** - All lookups go through SymbolTable
4. **Graph-based relationships** - Specialization/typing NOT stored in Symbol enum
5. **Three-pass imports** - Cannot resolve all imports in single pass

## Error Handling

- `Result<T, E>` for fallible operations
- `?` operator for propagation
- Semantic errors collected (not thrown)
- Parse errors include spans for diagnostics

## Thread Safety

- Parser is single-threaded per file
- Workspace can parse files in parallel
- LSP uses async runtime (async-lsp)
- Symbol table updates synchronized

## Thread Safety

- Parser is single-threaded per file
- Workspace can parse multiple files in parallel
- Symbol table updates are synchronized
- LSP uses async runtime for concurrent requests
