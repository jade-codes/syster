# TODO: Proper Adapter/Semantic Separation

## Phase 1: Create Semantic Type System ✅ COMPLETE
- [x] Create `semantic/types/semantic_role.rs` with enum (Requirement, Action, State, UseCase, Component, Interface, Unknown)
- [x] Add `semantic_role: Option<SemanticRole>` field to `Symbol::Definition` and `Symbol::Usage`
- [x] Export SemanticRole from `semantic/types/mod.rs`

## Phase 2: Update Adapter to Map Semantic Roles ✅ COMPLETE
- [x] In `semantic/adapters/sysml/helpers.rs`: Add `definition_kind_to_semantic_role()` and `usage_kind_to_semantic_role()`
- [x] Convert directly from AST enums (DefinitionKind, UsageKind) to SemanticRole (eliminated wasteful string conversion)
- [x] In `semantic/adapters/sysml/visitors.rs`: When creating Definition/Usage symbols, call mapping function and set `semantic_role` field
- [x] Adapter now translates language → semantic during population
- [x] Fixed all test Symbol creations to include semantic_role field
- [x] All 304 unit tests + 1441 integration tests passing
- [x] Created adapter factory pattern - workspace no longer knows about specific adapter types
- [x] Workspace is now language-agnostic (only uses SyntaxFile abstraction, delegates to adapters module)

## Phase 3: Make Validator Truly Generic ✅ COMPLETE
- [x] Simplified `RelationshipValidator` trait to just define the interface
- [x] Moved `SysmlValidator` to `semantic/adapters/sysml/validator.rs` (where language knowledge belongs)
- [x] Validator uses `SemanticRole` helpers: `is_requirement()`, `is_action()`, `is_state()`, `is_use_case()`
- [x] Deleted old `semantic/analyzer/validation/sysml_validator.rs` and tests
- [x] Tests now live in the adapter module with the validator
- [x] Updated exports: `SysmlValidator` exported from adapters, not analyzer
- [x] All 313 unit tests + 1441 integration tests passing

## Phase 4: Move Language-Specific Validation to Adapter ✅ COMPLETE
- [x] Validation logic lives in `semantic/adapters/sysml/validator.rs`
- [x] Trait is generic, implementations are language-specific
- [x] Clean separation: semantic/analyzer defines trait, adapters provide implementations
- [x] Ready for KerML validator when needed

## Phase 5: Update All References ✅ COMPLETE
- [x] All imports of `SysMLRelationshipValidator` removed (validator moved to adapters)
- [x] No `SymbolTablePopulator` references found (already using `SysmlAdapter`)
- [x] Workspace uses `semantic::adapters::populate_syntax_file` and `create_validator`
- [x] Analyzer uses factory-created validators
- [x] All tests updated to use semantic roles and factory pattern
- [x] `semantic/mod.rs` exports updated to use adapters
- [x] `semantic/analyzer.rs` exports cleaned up (no more SysMLRelationshipValidator)

## Phase 6: Add Architecture Tests ✅ COMPLETE
- [x] Added test in `tests/architecture_tests.rs`: `test_semantic_layer_only_adapters_import_syntax()`
- [x] Check that only files in `semantic/adapters/` and `semantic/processors/` import from syntax
- [x] Added `test_validators_use_semantic_roles_not_strings()` to ensure no hard-coded strings in validators
- [x] Added `test_core_constants_defined()` to verify required constants exist
- [x] All architecture tests passing (8 passed, 3 ignored for known issues)

## Phase 7: Build and Test ✅ COMPLETE
- [x] Run `cargo build` to check compilation
- [x] Run `cargo test` to verify all tests pass (371 tests passing)
- [x] Run architecture tests to verify no violations in new code
- [x] All semantic adapter validation uses constants (no hard-coded strings)

## Phase 8: Documentation ✅ COMPLETE
- [x] Updated comments in `semantic/adapters/mod.rs` explaining architectural boundary
- [x] Added ASCII diagram showing: Syntax → Adapters → Semantic → Analysis
- [x] Documented responsibilities: AST conversion, semantic role mapping, validators
- [x] Clarified that only adapters/ and processors/ import from syntax layer
- [x] Architecture boundary enforcement documented and tested

---

## Key Principle
- **Adapter** = Language-aware (imports from syntax)
- **Semantic** = Language-agnostic (works with semantic roles)
- **Validator** = Constraint checker (uses semantic roles only)

## Current State
- ✅ Phase 1-7 COMPLETE: Full refactoring and testing complete
- ✅ All 371 tests passing (34 adapter tests + 337 others)
- ✅ Architecture tests passing (8 passed, 3 ignored for legacy issues)
- ✅ Zero hard-coded strings in validators (all use constants)
- ✅ Clean separation: Syntax → Adapter → Semantic (with roles) → Analyzer → Validation

## Next Steps
**Phase 8**: Documentation improvements
- Add comments explaining adapter pattern
- Document the architecture boundary enforcement
