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
### Next Module Refactoring Tasks
- [ ] **lsp/ folder** (lsp-server crate) - Review files for refactoring opportunities
- Check file sizes and identify files >100 lines
- Apply same modularization pattern as semantic/
- [ ] Metrics/observability layer for EventEmitter

### Code Cleanup
- [ ] Replace hardcoded strings in `language/sysml/populator.rs` with SYSML_KIND_* constants
- [ ] Create relationship type constants (RELATIONSHIP_SATISFY, RELATIONSHIP_PERFORM, etc.)
- [ ] Extract `is_abstract` and `is_variation` from definition_prefix in AST
- [ ] Add annotation properties to KerML types