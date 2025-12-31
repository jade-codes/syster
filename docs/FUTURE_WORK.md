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

**File Size Analysis** (lines):
```
2778  tests.rs         ← CRITICAL: 82 tests in one file
 433  main.rs          ← Protocol handlers mixed with state
 213  formatting.rs
 176  helpers.rs       ← Has span_to_lsp_range() but not used everywhere!
 170  completion.rs
 146  core.rs
 135  rename.rs        ← Duplicates reference-finding from references.rs
 114  document.rs
 105  selection_range.rs
 101  semantic_tokens.rs
  89  references.rs    ← Core pattern that rename.rs duplicates
```

#### Cross-Cutting Concerns Identified

1. **URI → PathBuf Conversion** (11 occurrences, 3 different patterns)
   - `uri.to_file_path().ok()?` - hover, formatting, references, definition, rename
   - `uri.to_file_path().map_err(...)` - document.rs
   - `match uri.to_file_path() { Ok/Err }` - diagnostics, inlay_hints
   → **Extract:** `fn require_path(uri: &Url) -> Result<PathBuf, LspError>`

2. **Span → LSP Range Conversion** ✅ COMPLETE
   - `span_to_lsp_range()` in helpers.rs is now used everywhere
   - `position_to_lsp_position()` also available and used
   - `span_to_folding_range()` used for folding ranges

3. **Symbol Lookup Pattern** (repeated 6 times)
   ```rust
   .lookup_qualified(&name)
   .or_else(|| .lookup(&name))?
   ```
   - Found in: rename.rs (3x), definition.rs, and implicit in others
   → **Extract:** `fn resolve_symbol(&self, name: &str) -> Option<&Symbol>`

4. **Reference Collection + Location Building** (massive duplication)
   - `references.rs` lines 23-80: get refs → lookup source → build Location
   - `rename.rs` lines 72-107: EXACT SAME PATTERN → build TextEdit instead
   → **Extract:** `fn collect_reference_locations(&self, qname: &str) -> Vec<(PathBuf, Span)>`

5. **Position Finding Pattern**
   - `find_symbol_at_position()` used in: hover, references, definition, rename (2x)
   → Already extracted to position.rs ✓

#### Refactoring Plan

**Phase 1: Use Existing Helpers** ✅ COMPLETE
- [x] Replace all manual Span→Range with `span_to_lsp_range()` (all files now use helper)
- [x] Add `span_to_lsp_position()` helper for single positions (exists and used in diagnostics)

**Phase 2: Extract Cross-Cutting Utilities**
- [ ] Create `server/uri.rs`: `require_path()`, `path_to_uri()`
- [ ] Create `server/symbol_resolver.rs`: `resolve_symbol()`, `resolve_at_position()`
- [ ] Create `server/reference_collector.rs`: shared logic for refs + rename

**Phase 3: Split Large Files** ✅ COMPLETE
- [x] `tests.rs` → `tests/` directory with feature modules
- [ ] `main.rs` → extract `initialization.rs` for workspace/stdlib setup

**Phase 4: Consolidate Handler Logic**
- [ ] `references.rs` + `rename.rs` → share `ReferenceCollector`
- [ ] Review if `definition.rs` (38 lines) should merge with position.rs
- [ ] Metrics/observability layer for EventEmitter

### Code Cleanup
- [ ] Replace hardcoded strings in `language/sysml/populator.rs` with SYSML_KIND_* constants
- [ ] Create relationship type constants (RELATIONSHIP_SATISFY, RELATIONSHIP_PERFORM, etc.)
- [ ] Extract `is_abstract` and `is_variation` from definition_prefix in AST
- [ ] Add annotation properties to KerML types