---
applyTo: '**/{Cargo.toml,package.json,tsconfig.json,.gitignore,Makefile}'
---

# Configuration Files Guidelines

## Overview

This file provides guidance for working with configuration files in the Syster repository. These files define build settings, dependencies, linting rules, and project structure.

## Cargo.toml Files

### Workspace Root (Cargo.toml)

The root `Cargo.toml` defines:
- Workspace members (all crates)
- Shared workspace package metadata
- Workspace-wide linter rules (clippy)
- Build optimization profiles

**Key Principles:**
- Keep workspace metadata centralized
- Use consistent versioning across crates
- Define strict clippy rules for code quality
- Optimize profiles for development vs. production

### Workspace Lints

Current clippy rules enforce:
```toml
# Enforce explicit error handling
unwrap_used = "warn"           # Avoid .unwrap() in production code
expect_used = "warn"           # Avoid .expect() in production code  
panic = "warn"                 # Avoid panic!() in production code
todo = "warn"                  # Mark incomplete code
unimplemented = "warn"         # Mark unimplemented code

# Code quality
dbg_macro = "warn"             # Remove debug macros
print_stdout = "warn"          # Avoid print to stdout
print_stderr = "warn"          # Avoid print to stderr
missing_errors_doc = "warn"    # Document error conditions
missing_panics_doc = "warn"    # Document panic conditions

# Performance
inefficient_to_string = "warn"
unnecessary_wraps = "warn"

# Style consistency
enum_glob_use = "warn"
wildcard_imports = "warn"

# Test quality
collapsible_if = "warn"
collapsible_else_if = "warn"
cognitive_complexity = "deny"  # Keep functions simple

# Deny by category (set explicit priority to avoid ambiguity)
correctness = { level = "deny", priority = -1 }  # Critical errors
suspicious = { level = "deny", priority = -1 }
complexity = { level = "deny", priority = -1 }
perf = { level = "deny", priority = -1 }
```

**When adding new lints:**
- Justify why it improves code quality
- Test across the entire workspace
- Document in comments if non-obvious
- Use `warn` for style, `deny` for correctness

### Build Profiles

```toml
[profile.dev]           # Fast compilation for development
[profile.test]          # Test optimization
[profile.release]       # Balanced for containers
[profile.production]    # Maximum optimization (may fail in low memory)
```

**Don't modify profiles unless:**
- Compilation is too slow
- Runtime performance is problematic
- Memory constraints require changes

### Per-Crate Cargo.toml

Each crate (`syster-base`, `syster-cli`, `syster-lsp`) has its own `Cargo.toml`:

```toml
[package]
name = "syster-base"
version.workspace = true        # Use workspace version
edition.workspace = true        # Use workspace edition

[dependencies]
# Core dependencies
pest = "2.7"                   # Parser generator
# ... other deps

[dev-dependencies]
# Test-only dependencies
rstest = "0.18"               # Test fixtures
```

**Guidelines:**
- Use `workspace = true` for version, edition, authors, license
- Group dependencies logically (parser, semantic, LSP, etc.)
- Specify minimum required version
- Keep dev-dependencies separate
- Document non-obvious dependency choices

## package.json (VS Code Extension)

Located in `editors/vscode/package.json`, this defines:
- Extension metadata (name, version, description)
- VS Code engine compatibility
- Extension activation events
- Language definitions
- Configuration settings
- Commands and contributions

**Key Sections:**

### Extension Metadata
```json
{
  "name": "sysml-language-support",
  "displayName": "SysML v2 Language Support",
  "version": "0.1.0",
  "engines": {
    "vscode": "^1.85.0"  // Minimum VS Code version
  }
}
```

### Language Contributions
```json
{
  "contributes": {
    "languages": [{
      "id": "sysml",
      "extensions": [".sysml"],
      "aliases": ["SysML", "sysml"]
    }]
  }
}
```

### Configuration Settings
```json
{
  "contributes": {
    "configuration": {
      "properties": {
        "syster.stdlib.enabled": {
          "type": "boolean",
          "default": true,
          "description": "Load SysML standard library on document open"
        },
        "syster.stdlib.path": {
          "type": "string",
          "default": "",
          "description": "Custom path to SysML standard library directory"
        },
        "syster.lsp.path": {
          "type": "string",
          "default": "/workspaces/syster/target/release/syster-lsp",
          "description": "Path to syster-lsp binary"
        }
      }
    }
  }
}
```

**When modifying package.json:**
- Validate JSON syntax
- Test activation events carefully (affects startup time)
- Keep settings prefixed with `sysml.`
- Provide clear descriptions for user-facing settings
- Update README.md if adding new features

## tsconfig.json (TypeScript Configuration)

Controls TypeScript compilation for the VS Code extension:

```json
{
  "compilerOptions": {
    "module": "commonjs",
    "target": "ES2020",
    "strict": true,
    "outDir": "out"
  }
}
```

**Guidelines:**
- Keep `strict: true` for type safety
- Don't lower target below ES2020
- Use `outDir` for compiled output
- Enable source maps for debugging

## Makefile

Provides convenient commands for common tasks:

```makefile
build:           # Build the project
test:            # Run tests
fmt:             # Format code
lint:            # Run clippy
run-guidelines:  # Full validation pipeline
```

**Guidelines:**
- Keep targets simple and focused
- Use `.PHONY` for non-file targets
- Add helpful comments for each target
- Show progress for multi-step targets
- Don't duplicate functionality that's better in Cargo

**When adding new targets:**
1. Choose a clear, descriptive name
2. Add to help text
3. Test that it works
4. Document any dependencies

## .gitignore

Controls which files Git tracks:

**Current patterns:**
- `/target/` - Rust build artifacts
- `*.swp`, `*.swo` - Editor temporary files
- `Cargo.lock` in libraries (keep in binaries)
- `node_modules/` - npm dependencies
- `.DS_Store` - macOS files

**When modifying:**
- Add patterns for new build tools
- Don't ignore files that should be committed
- Use comments to explain non-obvious patterns
- Test with `git status` to verify

**Common additions:**
```gitignore
# IDE files
.vscode/settings.json  # But keep .vscode/extensions.json
.idea/

# Build artifacts  
/out/
/dist/
*.vsix

# OS files
Thumbs.db
```

## Configuration Best Practices

### Version Management
- Use workspace versioning for consistent releases
- Keep versions in sync across related crates
- Bump versions following semantic versioning:
  - MAJOR: Breaking changes
  - MINOR: New features, backward compatible
  - PATCH: Bug fixes

### Dependency Management
- Specify minimum required versions
- Avoid overly restrictive version constraints
- Keep dependencies up-to-date for security
- Review new dependencies carefully
- Document why non-obvious dependencies are needed

### Testing Configuration Changes
Before committing configuration changes:
```bash
# Test Cargo changes
cargo check --workspace
cargo test --workspace
cargo clippy --workspace

# Test package.json changes
cd editors/vscode
npm install
npm run compile

# Test Makefile changes
make build
make test
make run-guidelines
```

### Common Pitfalls

❌ **Don't:**
- Change profile settings without understanding impact
- Add dependencies without considering alternatives
- Modify linter rules to silence warnings (fix the code instead)
- Use wildcard version constraints (`*` or `>=x.0`)
- Add configuration options without documentation

✅ **Do:**
- Test configuration changes thoroughly
- Document non-obvious settings
- Keep configurations minimal and focused
- Use workspace features to avoid duplication
- Follow established patterns in the codebase

## File-Specific Guidelines

### When to Edit Cargo.toml
- Adding or removing dependencies
- Updating dependency versions
- Changing build profile settings
- Adding new crate features
- Modifying metadata or lints

### When to Edit package.json
- Adding new VS Code commands
- Adding configuration settings
- Updating extension metadata
- Changing activation events
- Adding language definitions

### When to Edit Makefile
- Adding convenience commands
- Simplifying complex workflows
- Creating shortcuts for testing
- Adding validation pipelines
- Improving developer experience

### When to Edit .gitignore
- New build tools generate artifacts
- IDE creates temporary files
- OS-specific files appear
- New dependency managers used
- Build output changes location

## Validation

After modifying configuration files:

```bash
# Validate Cargo.toml syntax
cargo check

# Validate package.json syntax
npm install --dry-run

# Validate tsconfig.json
npx tsc --noEmit

# Validate Makefile
make --dry-run <target>

# Verify .gitignore works
git status  # Should show only tracked files
```

## Security Considerations

- Never commit secrets or credentials
- Don't expose internal paths in public configs
- Be careful with git-ignored files (ensure secrets are ignored)
- Review dependencies for known vulnerabilities
- Keep dependencies up-to-date

## Documentation

When changing configuration files:
- Update README.md if user-facing features change
- Update CONTRIBUTING.md if developer workflow changes
- Document new configuration options
- Explain why changes were made in commit messages
