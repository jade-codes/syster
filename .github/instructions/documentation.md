---
applyTo: '**/*.md'
---

# Documentation Guidelines

## Documentation Philosophy

Documentation in Syster should be:
- **Clear and concise** - Get to the point quickly
- **Up-to-date** - Keep in sync with code changes
- **Actionable** - Provide concrete examples and commands
- **Well-organized** - Use consistent structure and hierarchy

## Key Documentation Files

### Repository Root
- **README.md** - Project overview, features, quick start, usage examples
- **ARCHITECTURE.md** - System design, core patterns, critical rules, common operations
- **LICENSE.md** - MIT license information

### docs/ Directory
- **CONTRIBUTING.md** - Development workflow, TDD guidelines, code conventions
- **SYSML_PRIMER.md** - SysML v2 and KerML concepts for developers new to the domain
- **FUTURE_WORK.md** - Planned features and improvements

### .github/
- **copilot-instructions.md** - Repository-wide Copilot guidance
- **instructions/*.md** - Path-specific Copilot instructions

### Per-Crate Documentation
- **crates/syster-base/README.md** - Core library documentation
- **crates/syster-cli/README.md** - CLI tool documentation  
- **crates/syster-lsp/README.md** - LSP server documentation
- **editors/vscode/README.md** - VS Code extension documentation

## Documentation Structure

### For Feature Documentation

Use this structure:
1. **Overview** - What is this feature?
2. **Purpose** - Why does it exist?
3. **Usage** - How to use it (with examples)
4. **Configuration** - Available options and settings
5. **Examples** - Real-world usage patterns
6. **Troubleshooting** - Common issues and solutions

### For Architecture Documentation

Use this structure:
1. **High-level overview** - System architecture diagram or description
2. **Key components** - Major modules and their responsibilities
3. **Data flow** - How information moves through the system
4. **Design decisions** - Why things are structured this way
5. **Constraints** - Important rules and invariants
6. **Common operations** - How to accomplish typical tasks

### For Contributing Documentation

Use this structure:
1. **Getting started** - Setup and prerequisites
2. **Development workflow** - How to develop features
3. **Testing guidelines** - How to write and run tests
4. **Code conventions** - Style and patterns to follow
5. **Pull request process** - How to submit changes
6. **Review checklist** - What reviewers look for

## Markdown Style Guide

### Headers
```markdown
# Top-level heading (only one per document)

## Section heading

### Subsection heading

#### Minor section (use sparingly)
```

### Code Blocks
Always specify the language for syntax highlighting:

````markdown
```rust
pub fn example() -> Result<(), Error> {
    // Code here
}
```

```bash
cargo build
cargo test
```
````

### Links
- Use descriptive link text: `[ARCHITECTURE.md](ARCHITECTURE.md)`
- Not: `[click here](ARCHITECTURE.md)`
- Prefer relative links within the repository
- Use absolute links for external resources

### Lists
```markdown
- Unordered lists use dashes
- Keep items parallel in structure
- Use bullet points for related items

1. Ordered lists use numbers
2. Use for sequential steps
3. Keep numbering consistent
```

### Emphasis
- Use **bold** for important terms and commands
- Use *italics* for emphasis
- Use `code formatting` for:
  - File names and paths
  - Commands and CLI arguments
  - Function and variable names
  - Code snippets inline

### Tables
```markdown
| Feature | Status | Notes |
|---------|--------|-------|
| Parsing | ✅ Complete | Full grammar support |
| LSP | ✅ Complete | All major features |
```

### Admonitions
Use consistent symbols for different message types:

```markdown
✅ **Good:** This is a good practice

❌ **Don't:** This is a bad practice

⚠️ **Warning:** This is important to know

💡 **Tip:** This is a helpful suggestion

📝 **Note:** This is additional information
```

## Documentation Updates

### When to Update Documentation

**Always update docs when:**
- Adding a new feature or API
- Changing existing behavior
- Adding new configuration options
- Modifying build or test commands
- Changing architecture or design patterns
- Fixing bugs that users might encounter

**Where to update:**
- **README.md** - For user-facing changes
- **ARCHITECTURE.md** - For design pattern changes
- **CONTRIBUTING.md** - For workflow changes
- **Doc comments** - For API changes
- **Changelog** - For all notable changes

### Documentation Review Checklist

Before committing documentation changes:
- [ ] Spell check completed
- [ ] Grammar checked
- [ ] Links tested (especially relative links)
- [ ] Code examples tested and working
- [ ] Consistent with existing style
- [ ] No broken markdown formatting
- [ ] Clear and concise
- [ ] Actionable with concrete examples

## Rust Doc Comments

### Module Documentation
```rust
//! # Module Name
//!
//! Brief description of the module's purpose.
//!
//! ## Overview
//! More detailed explanation...
//!
//! ## Examples
//! ```
//! use syster::module::Function;
//! 
//! let result = Function::new();
//! ```
```

### Function Documentation
```rust
/// Resolves a qualified name to its symbol definition.
///
/// # Arguments
///
/// * `qualified_name` - Fully qualified name like "Package::Element"
///
/// # Returns
///
/// The symbol if found, or `None` if not found.
///
/// # Examples
///
/// ```
/// let symbol = resolver.resolve_qualified("MyPackage::MyClass");
/// ```
///
/// # Errors
///
/// Returns `SemanticError::UndefinedSymbol` if the name cannot be resolved.
pub fn resolve_qualified(&self, name: &str) -> Result<&Symbol, SemanticError> {
    // Implementation
}
```

### What to Document
✅ **Do document:**
- Public APIs (functions, structs, enums)
- Complex algorithms or non-obvious logic
- Error conditions and edge cases
- Invariants and preconditions
- Examples for non-trivial usage

❌ **Don't document:**
- Self-explanatory code
- Trivial getters/setters
- Private implementation details (unless complex)
- Test functions (unless testing pattern is complex)

## Examples and Code Snippets

### In README.md
- Show the most common use cases first
- Include complete, runnable examples
- Show both simple and advanced usage
- Include output or expected results

### In ARCHITECTURE.md
- Use pseudocode for abstract concepts
- Use actual code for concrete implementations
- Include before/after examples for design patterns
- Show both good and bad examples

### In CONTRIBUTING.md
- Show complete workflows, not just fragments
- Include all necessary commands
- Explain why, not just how
- Provide troubleshooting steps

## Keeping Documentation Current

### During Development
- Update docs as you write code, not after
- Make doc updates part of your PR
- Update examples when APIs change
- Run `cargo doc` to check doc comments

### During Code Review
- Review doc changes as carefully as code changes
- Ensure examples compile and work
- Check that changes are reflected in all relevant docs
- Verify links and references are correct

### Periodic Audits
- Review documentation quarterly
- Remove outdated information
- Update examples with current best practices
- Consolidate or reorganize as needed

## Special Documentation Types

### API Documentation
- Generated from Rust doc comments
- Run `cargo doc --open` to view
- Should include examples and error conditions
- Links to related functions and types

### Tutorial Documentation
- Step-by-step guides
- Start simple, gradually increase complexity
- Include complete working examples
- Explain concepts before showing code

### Reference Documentation
- Comprehensive coverage of all features
- Organized by category or module
- Include all options and parameters
- Link to examples and tutorials

## Documentation Tools

### Checking Documentation
```bash
# Build and check Rust docs
cargo doc --no-deps --document-private-items

# Check for broken links in Rust docs
cargo doc --no-deps

# Spell check (if installed)
typos

# Markdown linting (if installed)
markdownlint **/*.md
```

### Generating Documentation
```bash
# Generate and open Rust API docs
cargo doc --open

# Generate docs for all crates
cargo doc --workspace
```

## Writing Style

### Voice and Tone
- Use active voice: "The parser generates..." not "The AST is generated by..."
- Be direct and concise
- Use present tense: "The function returns..." not "The function will return..."
- Be friendly but professional

### Technical Accuracy
- Be precise with terminology
- Use consistent names for concepts
- Define domain-specific terms
- Link to definitions when introducing terms

### Audience Awareness
- **README.md** - Assumes basic knowledge, shows what's possible
- **CONTRIBUTING.md** - Assumes developer familiarity with Rust
- **ARCHITECTURE.md** - Assumes developer working on the codebase
- **SYSML_PRIMER.md** - Assumes no SysML knowledge

## Common Documentation Pitfalls

❌ **Don't:**
- Let documentation get out of sync with code
- Use vague language like "usually" or "might"
- Assume readers know internal terminology
- Skip examples for complex features
- Leave broken links or outdated screenshots
- Write documentation after the code is done

✅ **Do:**
- Update docs with every code change
- Be specific and concrete
- Define terms when first used
- Provide multiple examples for different scenarios
- Test all links and examples
- Write docs alongside code

## Templates

### New Feature Documentation Template
```markdown
## Feature Name

### Overview
Brief description of what the feature does.

### Usage
Basic usage example with code.

### Configuration
Available settings and their defaults.

### Examples
#### Example 1: Basic Usage
Code and explanation.

#### Example 2: Advanced Usage
Code and explanation.

### Troubleshooting
Common issues and solutions.
```

### API Reference Template
```rust
/// Brief one-line description.
///
/// More detailed explanation of what this does,
/// including any important behavior or edge cases.
///
/// # Arguments
///
/// * `param1` - Description of first parameter
/// * `param2` - Description of second parameter
///
/// # Returns
///
/// Description of return value.
///
/// # Examples
///
/// ```
/// // Example usage
/// let result = function(arg1, arg2);
/// ```
///
/// # Errors
///
/// Description of error conditions.
///
/// # Panics
///
/// Description of panic conditions (if any).
pub fn function(param1: Type1, param2: Type2) -> ReturnType {
    // Implementation
}
```
