# Syster Architecture

An overview of the Syster toolchain for SysML v2 and KerML.

## System Overview

```
                          ┌──────────────────────────────────────────────┐
                          │              VS Code Extensions              │
                          │  ┌────────────┐ ┌──────────┐ ┌───────────┐  │
                          │  │  language-  │ │ modeller │ │  viewer   │  │
                          │  │  client     │ │          │ │           │  │
                          │  └─────┬──────┘ └────┬─────┘ └─────┬─────┘  │
                          │        │ LSP         │ React       │ React  │
                          └────────┼─────────────┼─────────────┼────────┘
                                   │             │             │
                          ┌────────┼─────────────┴─────────────┘
                          │        │        ┌──────────────────────────┐
                          │        │        │ Diagram Library (TS)     │
                          │        │        │ ┌────────────┐           │
                          │        │        │ │diagram-core│ types,    │
                          │        │        │ │            │ layout    │
                          │        │        │ └─────┬──────┘           │
                          │        │        │       ▼                  │
                          │        │        │ ┌────────────┐           │
                          │        │        │ │ diagram-ui │ React     │
                          │        │        │ │            │ Flow      │
                          │        │        │ └────────────┘           │
                          │        │        └──────────────────────────┘
                          │        │
  ┌───────────────────────┼────────┼───────────────────────────────────┐
  │ Rust Core             │        │                                   │
  │                       ▼        │                                   │
  │  ┌─────────────────────────┐   │   ┌─────────────┐                │
  │  │    language-server      │   │   │     cli      │                │
  │  │    (LSP binary)         │   │   │   (syster)   │                │
  │  └────────────┬────────────┘   │   └──────┬───────┘                │
  │               │                │          │                        │
  │               ▼                │          ▼                        │
  │  ┌─────────────────────────────────────────────────────────────┐   │
  │  │                        base (syster-base)                   │   │
  │  │                                                             │   │
  │  │  ┌──────┐ ┌────────┐ ┌────────┐ ┌─────────┐ ┌────┐ ┌────┐ │   │
  │  │  │ base │→│ parser │→│ syntax │→│ project │→│ hir│→│ ide│ │   │
  │  │  └──────┘ └────────┘ └────────┘ └─────────┘ └────┘ └────┘ │   │
  │  │                                                             │   │
  │  │  ┌─────────────────────────────────────────────────────┐    │   │
  │  │  │            interchange (feature-gated)              │    │   │
  │  │  │  model · views · host · editing · render · metadata │    │   │
  │  │  │  xmi · yaml · jsonld · kpar · decompile · recompile │    │   │
  │  │  └─────────────────────────────────────────────────────┘    │   │
  │  └─────────────────────────────────────────────────────────────┘   │
  │                                                                    │
  │  ┌─────────────────┐                                               │
  │  │  tree (systree)  │  Python bindings (PyO3)                      │
  │  └─────────────────┘                                               │
  └────────────────────────────────────────────────────────────────────┘
```

## Components

### base (`syster-base`)

The core library. Everything else depends on this crate. Organized as a layered stack where each layer depends only on the layers below it.

#### Layer 1: `base`

Foundational primitives shared across all modules.

| Type | Purpose |
|------|---------|
| `FileId` | Opaque handle identifying a source file |
| `Name` / `Interner` | String interning for efficient symbol comparison |
| `Span`, `Position`, `LineCol` | Source location tracking |
| `TextRange`, `TextSize` | Byte-level ranges (via rowan) |

#### Layer 2: `parser`

Lossless, incremental parser using the rust-analyzer architecture.

```
Source Text → Lexer (logos) → Tokens → Parser → GreenNode (rowan CST)
                                                       ↓
                                              SyntaxNode (typed AST)
```

- **Lexer** — logos-based tokenizer producing `SyntaxKind` tokens
- **Parser** — recursive-descent parser emitting a lossless CST via rowan `GreenNodeBuilder`
- **Grammar** — grammar rule traits organized by KerML/SysML sublanguage
- **AST** — typed wrappers over `SyntaxNode` for safe navigation

The CST preserves all whitespace and comments, enabling formatting and incremental reparsing.

#### Layer 3: `syntax`

AST types, formatting, and parse result wrappers. Includes the SysML formatter.

#### Layer 4: `project`

Workspace and file management.

| Module | Purpose |
|--------|---------|
| `StdLibLoader` | Locates and loads the SysML standard library |
| `WorkspaceLoader` | Scans directories for `.sysml`/`.kerml` files |
| `FileLoader` | Reads files into the database |
| `CachedStdLib` | Caches parsed stdlib for fast subsequent loads |

#### Layer 5: `hir` (High-level IR)

The semantic model, built on [Salsa](https://github.com/salsa-rs/salsa) for incremental computation.

```
file_text(file)           ← INPUT: raw source text
    │
    ▼
parse(file)               ← Parse into AST (per-file, memoized)
    │
    ▼
file_symbols(file)        ← Extract HirSymbols (per-file)
    │
    ▼
symbol_index              ← Workspace-wide name index
    │
    ▼
resolve_name(scope, name) ← Name resolution with imports
    │
    ▼
file_diagnostics(file)    ← Semantic error checking
```

Key types:

| Type | Purpose |
|------|---------|
| `RootDatabase` | Salsa database owning all inputs and query results |
| `HirSymbol` | A symbol extracted from the AST (name, kind, span, relationships) |
| `SymbolIndex` | Workspace-wide index mapping names → symbols |
| `Resolver` | Name resolution with import and alias handling |
| `DefId` | Unique identifier for a definition |
| `Diagnostic` | Semantic error/warning with source location |

#### Layer 6: `ide`

IDE features built on top of HIR queries. Each function maps to an LSP request.

| Module | LSP Feature |
|--------|-------------|
| `analysis.rs` | `AnalysisHost` / `Analysis` — entry point for all queries |
| `completion.rs` | `textDocument/completion` |
| `goto.rs` | `textDocument/definition`, `textDocument/typeDefinition` |
| `hover.rs` | `textDocument/hover` |
| `references.rs` | `textDocument/references` |
| `symbols.rs` | `textDocument/documentSymbol`, `workspace/symbol` |
| `semantic_tokens.rs` | `textDocument/semanticTokens` |
| `folding.rs` | `textDocument/foldingRange` |
| `inlay_hints.rs` | `textDocument/inlayHint` |
| `selection.rs` | `textDocument/selectionRange` |
| `document_links.rs` | `textDocument/documentLink` |

`AnalysisHost` is the mutable owner of the database. Call `host.analysis()` for a read-only snapshot (`Analysis`) that can be used concurrently.

#### Interchange Module (feature-gated: `interchange`)

A standalone model layer decoupled from the Salsa database, enabling format conversion, programmatic editing, and round-trip fidelity.

```
┌──────────────┐     ┌──────────────┐     ┌──────────────┐     ┌──────────────┐
│   XMI File   │     │  KPAR File   │     │  JSON-LD     │     │  YAML File   │
└──────┬───────┘     └──────┬───────┘     └──────┬───────┘     └──────┬───────┘
       │                    │                    │                    │
       ▼                    ▼                    ▼                    ▼
┌──────────────────────────────────────────────────────────────────────────────┐
│                         ModelFormat trait                                    │
│                read(&[u8]) → Model    write(&Model) → Vec<u8>               │
└──────────────────────────────────────────────────────────────────────────────┘
                                    │
                                    ▼
┌──────────────────────────────────────────────────────────────────────────────┐
│                          Model (standalone)                                  │
│  elements: IndexMap<ElementId, Element>           (preserves insertion order) │
│  relationships: Vec<Relationship>                                            │
│  roots: Vec<ElementId>                                                       │
│  metadata: ModelMetadata                                                     │
└──────────────────────────────────────────────────────────────────────────────┘
```

Sub-modules:

| Module | Purpose |
|--------|---------|
| `model.rs` | `Model`, `Element`, `ElementId`, `ElementKind`, `Relationship` — the standalone model graph |
| `views.rs` | Zero-copy typed views (`ElementView`, `PackageView`, `DefinitionView`, …) over the model |
| `host.rs` | `ModelHost` — ergonomic entry-point: parse text or load XMI → typed queries via views |
| `editing.rs` | `ChangeTracker` — mutation API (rename, add, remove, reparent) with dirty tracking |
| `render.rs` | `SourceMap` + `render_dirty()` — incremental re-rendering, splicing only changed regions |
| `decompile.rs` | Model → SysML text + metadata (from interchange format to editable source) |
| `recompile.rs` | Restores original element IDs from metadata when re-exporting |
| `metadata.rs` | `ImportMetadata`, `ProjectMetadata` — companion JSON for element ID round-trip |
| `integrate.rs` | Bridge between `Model` ↔ `SymbolIndex`: `model_from_symbols()`, `symbols_from_model()`, plus `AnalysisHost` edit/export integration |
| `format.rs` | `ModelFormat` trait and `FormatCapability` |
| `xmi.rs` | XMI (OMG XML Metadata Interchange) reader/writer |
| `yaml.rs` | YAML format reader/writer |
| `jsonld.rs` | JSON-LD format reader/writer |
| `kpar.rs` | KPAR (Kernel Package Archive — ZIP with XMI + metadata) reader/writer |

**Edit → Render pipeline:**

```
                   ModelHost::from_text("package P { part def A; }")
                                    │
                                    ▼
                   SourceMap::build(model) → (original_text, source_map)
                                    │
                     ┌──────────────┤
                     ▼              ▼
              ChangeTracker    original_text
              .rename(A → B)        │
              .add_element(C)       │
                     │              │
                     ▼              ▼
              render_dirty(text, source_map, model, tracker)
                                    │
                                    ▼
                          patched SysML text
                    (only dirty regions re-rendered)
```

**ID preservation round-trip:**

```
SysML → export → XMI (elements get UUIDs)
                  │
                  ▼
         decompile → SysML text + .metadata.json (records UUIDs)
                  │
                  ▼
         edit (rename/add/remove) → reads metadata, carries IDs forward
                  │
                  ▼
         re-export → XMI (original UUIDs preserved via recompile)
```

**Unified Model projection (`AnalysisHost::model()`):**

`AnalysisHost` now serves as the **single entry-point** for both IDE queries
and interchange `Model` access. The model is lazily cached from the existing
`SymbolIndex` via `model_from_symbols()`, eliminating the need for a separate
`ModelHost::from_text()` (which is now deprecated).

```
  SysML text ─── parse ──→ SymbolIndex ─── model_from_symbols() ──→ Model (cached)
       │                        │                                       │
       ▼                        ▼                                       ▼
  IDE queries              AnalysisHost.analysis()              AnalysisHost.model()
  (hover, goto, …)        (symbol_index snapshot)              (navigation, export)
```

**Semantic edits via `AnalysisHost::apply_model_edit()`:**

When edits made via `ChangeTracker` need to be reflected back in the host's
`SymbolIndex`, `apply_model_edit()` internalizes the text round-trip:

```
  edit closure ──→ ChangeTracker edits ──→ render_dirty() ──→ patched SysML text
                                                                    │
                                                                    ▼
                                                    set_file_content() → re-parse
                                                                    │
                                                                    ▼
                                                    Element IDs restored from Model
```

This keeps the parser as the single source of truth while allowing
ChangeTracker edits to propagate into IDE features (hover, goto, completions).

> **Note:** The standalone `apply_edits_to_host()` function in `integrate.rs`
> is deprecated in favor of `AnalysisHost::apply_model_edit()`.
> `ModelHost::from_text()` is also deprecated — use `AnalysisHost::model()`.

---

### cli (`syster-cli`)

Command-line binary exposing analysis, interchange, and semantic editing.

| Command | Purpose |
|---------|---------|
| `syster model.sysml` | Parse + semantic analysis with diagnostics |
| `--export xmi\|yaml\|jsonld\|kpar` | Export to interchange format |
| `--import` | Import and validate interchange file |
| `--import-workspace` | Import into analysis workspace (preserves IDs) |
| `--decompile` | Convert interchange → SysML text + metadata |
| `--list` / `--query` / `--kind` | Browse model elements |
| `--inspect` | Detailed element view (children, relationships) |
| `--rename OLD=NEW` | Rename element + update metadata |
| `--add-member PARENT:KIND:NAME[:TYPE]` | Add child element |
| `--remove NAME` | Remove element |
| `--export-ast` | Export AST as JSON |
| `--json` | JSON output mode for any command |

Exit codes: `0` success, `1` error, `2` success with warnings.

All edit commands (`--rename`, `--add-member`, `--remove`) read companion `.metadata.json` files and write updated metadata alongside the output, preserving element IDs across edits.

---

### language-server (`syster-lsp`)

LSP server binary built on `tower-lsp`. Wraps `AnalysisHost` from the base crate.

```
VS Code (language-client) ←── stdio ──→ language-server binary
                                              │
                                         AnalysisHost
                                              │
                                         base (ide module)
```

The server maintains a live `AnalysisHost`, updating file contents on `didOpen`/`didChange` and serving requests by calling the corresponding `ide` module function.

---

### language-client (`syster-vscode-lsp`)

VS Code extension providing:

- Syntax highlighting (TextMate grammar in `syntaxes/`)
- Language configuration (brackets, comments, auto-closing)
- LSP client spawning the `syster-lsp` binary
- Language-specific settings

---

### modeller (`syster-vscode-modeller`)

VS Code extension with a webview-based diagram editor. Uses `diagram-ui` React components rendered inside a VS Code webview panel.

---

### viewer (`syster-vscode-viewer`)

Read-only diagram viewer extension. Same architecture as modeller but without editing capabilities.

---

### diagram-core (`syster-diagram-core`)

TypeScript library defining:

- SysML node types (`sysml-nodes.ts`) — part, package, port, requirement, etc.
- SysML edge types (`sysml-edges.ts`) — composition, specialization, dependency, etc.
- Layout algorithms

---

### diagram-ui (`syster-diagram-ui`)

React + React Flow component library:

- Node renderers for each SysML element kind
- Edge renderers for each relationship kind
- Theme system
- Layout integration

---

### tree (`systree`)

Python wrapper using PyO3/maturin:

```python
from systree import analyze, get_symbols, export_xmi

result = analyze("model.sysml")
symbols = get_symbols("model.sysml")
xmi = export_xmi("model.sysml")
```

Exposes `base` crate functionality (analysis, symbol extraction, interchange export) to Python.

---

## Data Flow

### Parsing → Analysis → IDE

```
.sysml file
    │
    ▼
Lexer (logos) → Token stream
    │
    ▼
Parser → GreenNode (lossless CST via rowan)
    │
    ▼
AST layer → typed SyntaxNode wrappers
    │
    ▼
HIR extraction → HirSymbol[] per file
    │
    ▼
SymbolIndex → workspace-wide name → symbol mapping
    │
    ▼
Resolver → qualified names, import resolution
    │
    ▼
Diagnostics → errors + warnings
    │
    ▼
IDE features → completions, hover, goto-def, references, …
```

### Interchange Round-Trip

```
SysML text ──parse──→ HIR symbols ──model_from_symbols──→ Model ──Xmi.write──→ XMI bytes
                                                                                    │
XMI bytes ──Xmi.read──→ Model ──decompile──→ SysML text + ImportMetadata            │
                                    │                                                │
                              (edit cycle)                                           │
                                    │                                                │
                              SysML text ──parse──→ HIR ──model_from_symbols──→ Model│
                                    +                                          │     │
                              ImportMetadata ──restore_element_ids────────────→ │     │
                                                                               ▼     │
                                                              Model (original IDs) ──┘
                                                                    │
                                                              Xmi.write → XMI (IDs preserved)
```

### CLI Edit Pipeline

```
input.sysml + input.metadata.json
          │
          ▼
    ModelHost::from_text(source)
          │
          ▼
    restore_element_ids(model, metadata)   ← IDs remapped
          │
          ▼
    SourceMap::build(model)                ← span tracking
          │
          ▼
    ChangeTracker.rename / add / remove    ← mutations
          │
          ▼
    render_dirty(text, map, model, tracker) ← incremental patch
          │
          ▼
    build_updated_metadata(model)          ← updated ID map
          │
          ▼
    output.sysml + output.metadata.json
```

## Key Design Decisions

### Lossless CST (rowan)

The parser builds a lossless Concrete Syntax Tree preserving all whitespace and comments. This enables:

- Exact formatting preservation
- Incremental reparsing (only changed subtrees are re-parsed)
- Source-faithful error recovery

### Salsa Incremental Computation

HIR queries use Salsa for automatic memoization and invalidation. When a file changes, only affected queries are recomputed. This makes the LSP responsive even on large workspaces.

### Standalone Model

The interchange `Model` type is deliberately decoupled from the Salsa database:

- Can be constructed from XMI/JSON-LD without text parsing
- Can be serialized without database overhead
- Enables the edit→render pipeline without Salsa
- Lazily cached inside `AnalysisHost` via `model()` — single owner for both IDE and interchange
- `ModelHost` remains useful for format I/O (XMI, KPAR, etc.) but `from_text()` is deprecated

### Companion Metadata

Element identity (UUIDs) doesn't survive a text round-trip (parsing always generates fresh IDs). Companion `.metadata.json` files record the original IDs keyed by qualified name, enabling lossless round-trip through the edit pipeline.

### Feature-Gated Interchange

The `interchange` module is behind a Cargo feature flag. The core parser and analysis work without it, keeping the dependency footprint small for consumers that don't need format conversion.

## Repository Layout

Each component lives in its own directory and has its own repository for independent versioning:

```
syster/
├── base/                  Rust   syster-base       Core library (parser, HIR, IDE, interchange)
├── cli/                   Rust   syster-cli        CLI binary
├── language-server/       Rust   syster-lsp        LSP server binary
├── language-client/       TS     syster-vscode-lsp VS Code language extension
├── modeller/              TS     syster-vscode-modeller  Diagram editor extension
├── viewer/                TS     syster-vscode-viewer    Diagram viewer extension
├── diagram-core/          TS     syster-diagram-core     Diagram types + layout
├── diagram-ui/            TS     syster-diagram-ui       React Flow components
├── tree/                  Python systree                 Python bindings
├── pipelines/             YAML   syster-pipelines        CI/CD templates
└── scripts/               Shell                          Release automation
```

## Build & Test

```bash
# Base library (core)
cd base && cargo test

# Base with interchange
cd base && cargo test --features interchange

# CLI
cd cli && cargo build --features interchange
cd cli && cargo test --features interchange

# LSP server
cd language-server && cargo build --release

# Python bindings
cd tree && pip install -e ".[dev]" && pytest

# TypeScript (diagram libraries)
cd diagram-core && bun install && bun test
cd diagram-ui && bun install && bun test

# VS Code extensions
cd language-client && npm install && npm run compile
cd modeller && npm install && npm run compile
cd viewer && npm install && npm run compile
```
