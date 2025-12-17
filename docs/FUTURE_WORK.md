# Future Work

## LSP Feature Implementation (Priority Order)

### Completed
- [x] **Task 12: Document Symbols** (Outline view) ✅
  - [x] Add `get_document_symbols()` method to Backend
  - [x] Extract all symbols from a file with hierarchy
  - [x] Map Symbol types to LSP SymbolKind (Package → Namespace, Definition → Class, etc.)
  - [x] Include symbol ranges for navigation
  - [x] Wire up `textDocument/documentSymbol` LSP handler
  - [x] Tests: flat file, nested packages, definitions with members

- [x] **Task 13: Semantic Tokens** (Syntax highlighting) ✅
  - [x] Add SemanticTokenCollector processor in semantic layer
  - [x] Walk AST and classify tokens (Package → NAMESPACE, Definition → TYPE, Usage → PROPERTY, Alias → VARIABLE)
  - [x] Return tokens in document order (AST traversal order)
  - [x] Add LSP adapter for protocol conversion with delta encoding
  - [x] Wire up `textDocument/semanticTokens/full` LSP handler
  - [x] Tests: 6 unit tests (processor), 1 integration test (LSP)
  - [x] Architecture: Processor pattern in semantic/processors/, thin LSP adapter

- [x] **Task 14: Code Completion** ✅
  - [x] Add keywords module in parser layer (KERML_KEYWORDS, SYSML_KEYWORDS)
  - [x] Implement get_keywords_for_file() for file-type detection
  - [x] Detect completion context (AfterColon, AfterRelationshipKeyword, AfterDef, General)
  - [x] Query symbol table for type and symbol completions
  - [x] Wire up `textDocument/completion` LSP handler
  - [x] Tests: keyword completion, file type detection
  - [x] Architecture: Keywords from parser layer, symbols from semantic layer, context in LSP

- [x] **Task 15: Rename Symbol** ✅
  - [x] Add `get_rename_edits()` method to LspServer
  - [x] Find all references to symbol (reuse find-references logic)
  - [x] Generate WorkspaceEdit with all locations (definition + references)
  - [x] Wire up `textDocument/rename` LSP handler
  - [x] Tests: rename from definition, rename from usage

### In Progress / Next Tasks

### Architecture Notes
- **Reusable patterns:**
  - `extract_word_at_cursor()` - Used in: go-to-def, find-refs, semantic-tokens
  - `find_symbol_at_position()` - Used in: hover, go-to-def, find-refs
  - Symbol lookup fallback (qualified → simple → all_symbols) - Used in: go-to-def, find-refs

## Event System
- [ ] Event batching for bulk operations
- [ ] Event replay/history for debugging
- [ ] Async event handlers (tokio/async-std)
- [ ] Priority-based listener ordering

## Architecture & Code Organization

**Pattern established:** Main files ~10-20 lines with focused submodules handling specific concerns.

### Next Module Refactoring Tasks
- [ ] **project/ folder** - Review files for refactoring opportunities
  - Check file sizes and identify files >100 lines
  - Apply same modularization pattern as semantic/
  
- [ ] **language/ folder** - Review files for refactoring opportunities
  - Check file sizes and identify files >100 lines
  - Apply same modularization pattern as semantic/
  
- [ ] **lsp/ folder** (lsp-server crate) - Review files for refactoring opportunities
  - Check file sizes and identify files >100 lines
  - Apply same modularization pattern as semantic/

- [ ] Metrics/observability layer for EventEmitter

## LSP Features
- [ ] Incremental symbol resolution (fine-grained updates)
- [ ] Workspace-wide event aggregation
- [ ] Snapshot/restore state for crash recovery

## Performance
- [ ] Parallel file population with Rayon
- [ ] Specialized symbol index (trie/inverted index)

## Testing & Quality
- [ ] Property-based testing with proptest
- [ ] Benchmark suite with criterion
- [ ] 100% public API documentation coverage
- [ ] **Test Organization & Separation of Concerns**
  - Review test files for proper organization (unit vs integration vs end-to-end)
  - Separate test helpers from test code (extract common test utilities)
  - Move integration tests to tests/ directory where appropriate
  - Ensure tests follow same modularization pattern as main code
  - Create test fixtures/builders for complex test data setup
  - Review workspace/tests.rs (934 lines) - consider splitting by feature area
  - Extract common test patterns (e.g., unwrap_sysml helper, parse_sysml helper)

## Code Cleanup
- [ ] Replace hardcoded strings in `language/sysml/populator.rs` with SYSML_KIND_* constants
- [ ] Create relationship type constants (RELATIONSHIP_SATISFY, RELATIONSHIP_PERFORM, etc.)
- [ ] Extract `is_abstract` and `is_variation` from definition_prefix in AST
- [ ] Add KerML parser support in file_loader
- [ ] Add annotation properties to KerML types
