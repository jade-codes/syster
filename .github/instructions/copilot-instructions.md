---
applyTo: '**/*.rs'
---

# Rust Development Instructions

## Project-Specific Guidance

### Architecture Understanding - READ FIRST

Before making changes, familiarize yourself with the project architecture:

- **[ARCHITECTURE.md](../../ARCHITECTURE.md)** - System design, three-phase pipeline, design decisions
- **[docs/CONTRIBUTING.md](../../docs/CONTRIBUTING.md)** - Development workflow, code conventions
- **[docs/SYSML_PRIMER.md](../../docs/SYSML_PRIMER.md)** - SysML v2 and KerML concepts

### Key Project Patterns

1. **Three-Phase Pipeline**: Parse (Pest) → Syntax (AST) → Semantic (Symbols + Graphs)
   - **Never mix phases**: Don't add semantic logic to parser, don't parse in semantic analyzer
   - Each phase has clear inputs and outputs (see ARCHITECTURE.md)

2. **Symbol Table is Global**: AST nodes don't contain resolved references
   - Symbols are stored in centralized `SymbolTable`
   - Use qualified names (`QualifiedName`) for cross-file references
   - Don't add back-references to AST nodes

3. **Relationship Graphs**: Use separate graphs for relationships
   - Don't store specialization/typing in Symbol enum
   - Use `RelationshipGraph` methods for queries
   - Graph operations are in `src/semantic/graph.rs`

4. **Import Resolution**: Three-pass algorithm (see SEMANTIC_ANALYSIS.md)
   - Pass 1: Namespace imports (`Package::*`)
   - Pass 2: Member imports (`Package::Member`)
   - Pass 3: Recursive imports (`Package::*::**`)
   - Don't try to resolve imports in a single pass

5. **Type Aliases**: Use documented type aliases for clarity
   - `QualifiedName` for fully qualified names
   - `SimpleName` for unqualified names
   - `ScopeId` for scope identifiers
   - `SourceFilePath` for file paths

### Adding New Features

**Adding a new SysML element type** (e.g., `concern def`):
1. Update grammar: `src/parser/sysml.pest`
2. Define AST struct: `src/language/sysml/syntax/types.rs`
3. Add to parent enum: `src/language/sysml/syntax/enums.rs`
4. Update populator: `src/language/sysml/populator.rs`
5. Add tests: `tests/semantic/sysml_parsing_tests.rs`

**Adding a new semantic check**:
1. Add error kind: `src/semantic/error.rs`
2. Implement check: `src/semantic/analyzer.rs`
3. Call from `analyze()` method
4. Add tests: `tests/semantic/mod.rs`

See ARCHITECTURE.md "Common Operations Guide" for detailed examples.

### Module Organization

```
src/
├── parser/          # Pest grammar files (kerml.pest, sysml.pest)
├── language/        # Language-specific AST and populators
│   ├── kerml/       # KerML foundation language
│   └── sysml/       # SysML v2 systems language
├── semantic/        # Semantic analysis (YOU ARE HERE most of the time)
│   ├── symbol_table.rs  # Global symbol registry
│   ├── graph.rs         # Relationship graphs
│   ├── resolver.rs      # Name resolution
│   ├── analyzer.rs      # Validation passes
│   └── workspace.rs     # Multi-file coordination
└── project/         # Workspace loading (stdlib, user projects)
```

### Common Pitfalls

❌ **Don't**: Add semantic logic to AST nodes
✅ **Do**: Keep AST immutable, use SymbolTable for semantic info

❌ **Don't**: Resolve imports while building symbol table
✅ **Do**: Build symbol table first, then run three-pass import resolution

❌ **Don't**: Store relationships in Symbol enum
✅ **Do**: Use RelationshipGraph for all relationships

❌ **Don't**: Create circular dependencies between modules
✅ **Do**: Follow dependency flow: parser → language → semantic

### Terminology (SysML/KerML Specific)

- **Qualified Name**: Full path like `Package::Class::Feature`
- **Classifier**: KerML type that can have features (class, struct, etc.)
- **Definition**: SysML type (part def, port def, action def, etc.)
- **Usage**: SysML instance (part, port, action, etc.)
- **Feature**: Property or operation of a classifier
- **Specialization**: IS-A relationship (inheritance)
- **Typing**: INSTANCE-OF relationship
- **Subsetting**: REFINES relationship
- **Redefinition**: OVERRIDES relationship

See SYSML_PRIMER.md for full glossary.

## Test-Driven Development (TDD) - MANDATORY

1. **Always write tests first** before implementing functionality
   - Write a failing test that describes the desired behavior
   - Run the test to confirm it fails for the right reason
   - Implement the minimal code to make the test pass
   - Refactor if needed while keeping tests green

2. **Test execution is required**
   - After writing a test, you MUST run it using `runTests` tool or `cargo test`
   - Never proceed to implementation without seeing the test fail first
   - Never claim completion without running tests to verify success

## Incremental Development - STRICT LIMITS

1. **One function at a time**
   - Edit only ONE function per change cycle
   - Complete the full TDD cycle (test → implement → verify) for that function
   - Do not move to the next function until current one is complete and tested

2. **Small, focused changes**
   - Each change should be minimal and focused
   - If a change requires modifying multiple functions, STOP and break it down
   - Prefer multiple small commits over one large change

3. **Restart trigger**
   - If you find yourself making changes across multiple files simultaneously, STOP
   - If you're modifying more than ~10-15 lines in a single function, STOP and reconsider
   - If the scope is growing beyond the original small task, STOP and restart with a smaller goal
   - Ask the user to break down the task into smaller pieces if needed

## Rust Best Practices

1. **Error handling**
   - Use `Result<T, E>` for fallible operations
   - Use `Option<T>` for values that may or may not exist
   - Avoid `.unwrap()` and `.expect()` in production code (tests are okay)
   - Propagate errors with `?` operator when appropriate

2. **Ownership and borrowing**
   - Prefer borrowing (`&T`, `&mut T`) over transferring ownership
   - Use `.clone()` judiciously - only when necessary
   - Leverage the borrow checker to ensure memory safety

3. **Idiomatic Rust**
   - Use iterator methods (`.map()`, `.filter()`, `.collect()`) over explicit loops when clearer
   - Prefer pattern matching over if-let chains
   - Use `derive` macros for common traits (Debug, Clone, PartialEq, etc.)
   - Follow Rust naming conventions (snake_case for functions/variables, CamelCase for types)

4. **Code organization**
   - Keep modules focused and cohesive
   - Use `mod.rs` or module files appropriately
   - Make items private by default, expose only what's needed
   - Document public APIs with doc comments (`///`)

## Workflow Checklist

Before each change:
- [ ] Have I identified ONE specific function to modify?
- [ ] Have I written a test for this change?
- [ ] Is this change small enough (< 15 lines in the function)?

After each change:
- [ ] Did I run the tests?
- [ ] Did the tests pass?
- [ ] Is the code formatted with `cargo fmt`?
- [ ] Does `cargo clippy` pass without warnings?

If you answer "no" to any of these, STOP and address it before proceeding.

## Development Commands

- `cargo test` - Run all tests
- `cargo test <name>` - Run specific test
- `cargo fmt` - Format code
- `cargo clippy` - Run linter
- `cargo build` - Build project
- `cargo run` - Run the application

## Red Flags - When to STOP

⛔ You're editing multiple functions simultaneously
⛔ You're making changes across multiple modules at once  
⛔ The diff is growing beyond 20-30 lines
⛔ You haven't run tests in the last change cycle
⛔ You're implementing before writing tests
⛔ You're unsure how to test the change

**When you see these flags, pause and restart with a smaller scope.**
