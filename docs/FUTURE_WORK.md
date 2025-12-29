# Future Work
---

## LSP Feature Implementation (Priority Order)

### Architecture Notes
- **Reusable patterns:**
  - `extract_word_at_cursor()` - Used in: go-to-def, find-refs, semantic-tokens
  - `find_symbol_at_position()` - Used in: hover, go-to-def, find-refs
  - Symbol lookup fallback (qualified → simple → all_symbols) - Used in: go-to-def, find-refs
  - Main files ~10-20 lines with focused submodules handling specific concerns.

## Event System
- [ ] Event batching for bulk operations
- [ ] Event replay/history for debugging
- [ ] Async event handlers (tokio/async-std)
- [ ] Priority-based listener ordering

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
  
## Architecture & Code Cleanup

### 🚨 IMMEDIATE: LSP Layer Refactoring (High Priority)
The LSP crate needs significant cleanup to improve maintainability:

- [ ] **Split `tests.rs` (~2700 lines)** - Currently a monolithic test file
  - Extract to feature-based test modules: `tests/references_tests.rs`, `tests/hover_tests.rs`, etc.
  - Each feature module should have its own focused test file
  
- [ ] **Improve main.rs organization** - `ServerState` struct is growing
  - Consider splitting protocol handlers into separate files
  - Extract initialization logic to dedicated module
  
- [ ] **Review handler modules** - Some may benefit from extraction
  - `references.rs` + `rename.rs` share code → extract common reference-finding logic
  - `document.rs` handles multiple concerns → consider splitting
  
- [ ] **Standardize patterns across handlers**
  - Common error handling patterns
  - Consistent URI → Path conversion with proper error messages
  - Shared position/range utilities

### Next Module Refactoring Tasks
- [ ] **lsp/ folder** (lsp-server crate) - Apply same modularization pattern as semantic/
  - Check file sizes and identify files >100 lines
  - Create focused submodules with clear single responsibilities
- [ ] Metrics/observability layer for EventEmitter

### Code Cleanup
- [ ] Replace hardcoded strings in `language/sysml/populator.rs` with SYSML_KIND_* constants
- [ ] Create relationship type constants (RELATIONSHIP_SATISFY, RELATIONSHIP_PERFORM, etc.)
- [ ] Extract `is_abstract` and `is_variation` from definition_prefix in AST
- [ ] Add annotation properties to KerML types