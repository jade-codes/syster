# Future Work

## LSP Feature Implementation (Priority Order)

### Completed ✓
- [x] Task 8: AST span tracking for all elements
- [x] Task 9: Rich hover with relationships
- [x] Task 10: Go-to-definition
- [x] Task 11: Find References (text-based search)

### In Progress / Next Tasks
  - [ ] Add `get_document_symbols()` method to Backend
  - [ ] Extract all symbols from a file with hierarchy
  - [ ] Map Symbol types to LSP SymbolKind (Package → Namespace, Definition → Class, etc.)
  - [ ] Include symbol ranges for navigation
  - [ ] Wire up `textDocument/documentSymbol` LSP handler
  - [ ] Tests: flat file, nested packages, definitions with members

- [ ] **Task 13: Semantic Tokens** (Syntax highlighting)
  - [ ] Add `get_semantic_tokens()` method to Backend
  - [ ] Walk AST and classify each token (keyword, type, variable, etc.)
  - [ ] Use `extract_word_at_cursor()` pattern for precise token ranges
  - [ ] Map to LSP semantic token types and modifiers
  - [ ] Wire up `textDocument/semanticTokens/full` LSP handler
  - [ ] Tests: keywords, types, variables, relationships

- [ ] **Task 14: Code Completion**
  - [ ] Add `get_completions()` method to Backend
  - [ ] Detect completion context (after `def`, after `:`, etc.)
  - [ ] Query symbol table for available symbols in scope
  - [ ] Include relationship keywords (specializes, subsets, etc.)
  - [ ] Wire up `textDocument/completion` LSP handler
  - [ ] Tests: keyword completion, type completion, member completion

- [ ] **Task 15: Rename Symbol**
  - [ ] Add `get_rename_edits()` method to Backend
  - [ ] Find all references to symbol (reuse Task 11 logic)
  - [ ] Generate WorkspaceEdit with all locations
  - [ ] Update qualified names in symbol table
  - [ ] Wire up `textDocument/rename` LSP handler
  - [ ] Tests: rename definition, rename usage, cross-file rename

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

## Architecture
- [ ] Split `semantic/events.rs` into folder if >300 lines
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

## Code Cleanup
- [ ] Replace hardcoded strings in `language/sysml/populator.rs` with SYSML_KIND_* constants
- [ ] Create relationship type constants (RELATIONSHIP_SATISFY, RELATIONSHIP_PERFORM, etc.)
- [ ] Extract `is_abstract` and `is_variation` from definition_prefix in AST
- [ ] Add KerML parser support in file_loader
- [ ] Add annotation properties to KerML types
