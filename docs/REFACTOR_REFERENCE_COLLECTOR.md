# Refactoring Plan: Remove ReferenceCollector Redundancy

**Created:** 2026-01-07  
**Status:** In Progress

## Problem

We have duplicated reference storage:

| Component | Storage | When Populated | Lookup |
|-----------|---------|----------------|--------|
| `RelationshipGraph.get_references_to()` | Reverse index | On AST population | O(1) ✅ |
| `Symbol.references` field | Vec per symbol | Via ReferenceCollector | O(1) |

**Both store the same data!** The `ReferenceCollector`:
1. Iterates ALL symbols (O(n)) 
2. Extracts relationships from `RelationshipGraph`
3. Copies them into `Symbol.references`

LSP already uses `RelationshipGraph.get_references_to()` directly. The `Symbol.references` field is only used in tests.

## Key Insight

`ReferenceCollector` does **two separate things**:
1. **Relationship references** → Copies from RelationshipGraph into Symbol.references ❌ REDUNDANT

**Solution:** Keep import reference collection, move it out of ReferenceCollector.

---

## Phase 1: Extract Import Reference Collection

| Step | File | Change |
|------|------|--------|
| 1.1 | `populator.rs` | Create new `collect_import_references()` method that only populates `import_references` HashMap |
| 1.2 | `populator.rs` | Call it from `populate_all()` instead of `ReferenceCollector` |

## Phase 2: Remove Symbol.references Field

| Step | File | Change |
|------|------|--------|
| 2.1 | `symbol.rs` | Remove `references` field from all Symbol variants |
| 2.2 | `symbol.rs` | Remove `references()` and `add_references()` methods |
| 2.3 | `types.rs` | Remove `SymbolReference` struct |
| 2.4 | `table.rs` | Remove `add_references_to_symbol()` method |

## Phase 3: Delete ReferenceCollector

| Step | File | Change |
|------|------|--------|
| 3.1 | `reference_collector.rs` | Delete entire file |
| 3.2 | `processors/mod.rs` | Remove `ReferenceCollector` export |
| 3.3 | `mod.rs` (semantic) | Update re-exports if needed |

## Phase 4: Update Tests

| Step | File | Change |
|------|------|--------|
| 4.1 | `tests_processors.rs` | Delete ReferenceCollector tests (~10 tests) |
| 4.2 | `tests_lookup_global_mut.rs` | Remove `references()` assertions |
| 4.3 | `tests_server.rs` | Remove debug loop over `symbol.references()` |

---

## Risk Assessment

| Phase | Risk | Mitigation |
|-------|------|------------|
| Phase 1 | Import refs stop working | Run tests after each step |
| Phase 2 | Compilation errors | Follow dependency order |
| Phase 3 | Missing exports | Grep for all usages first |
| Phase 4 | Test failures | Tests should pass by Phase 3 end |

## Execution Order

```
1. Phase 1.1-1.2 (extract import refs) → cargo test
2. Phase 2.1-2.4 (remove field) → cargo test  
3. Phase 3.1-3.3 (delete module) → cargo test
4. Phase 4.1-4.3 (cleanup tests) → cargo test
```

## Files Affected

- `crates/syster-base/src/semantic/workspace/populator.rs`
- `crates/syster-base/src/semantic/symbol_table/symbol.rs`
- `crates/syster-base/src/semantic/symbol_table/types.rs`
- `crates/syster-base/src/semantic/symbol_table/table.rs`
- `crates/syster-base/src/semantic/processors/reference_collector.rs` (DELETE)
- `crates/syster-base/src/semantic/processors/mod.rs`
- `crates/syster-base/src/semantic/processors/tests/tests_processors.rs`
- `crates/syster-base/src/semantic/symbol_table/tests/tests_lookup_global_mut.rs`
- `crates/syster-lsp/src/server/tests/tests_server.rs`
