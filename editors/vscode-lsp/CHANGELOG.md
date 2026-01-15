# Changelog

All notable changes to the "SysML v2 Language Support" extension will be documented in this file.

## [0.1.8-alpha] - 2026-01-15

### Added
- New diagram viewer extension (`syster-viewer`) for visualizing SysML models
- New diagram modeller extension (`syster-modeller`) for interactive model editing
- Custom LSP endpoint `syster/getDiagram` for diagram data retrieval
- React Flow based webview for rendering diagrams

### Fixed
- Replaced deprecated `all_symbols()` with `iter_symbols()` in diagram endpoint
- Fixed relationship extraction to use symbol data directly instead of hardcoded typing
- Fixed webview bundling to use esbuild with proper `import.meta` handling for VS Code compatibility
- Added `isDisposed` guard in diagram panel to prevent "Webview is disposed" errors
- Restored `sysml-language-support` package name for marketplace compatibility
- Updated CI workflow paths for refactored extension structure

### Improved
- Simplified kind mapping in diagram endpoint (pass through directly)
- Better error handling for disposed webviews during async operations

## [0.1.7-alpha] - 2026-01-11

### Fixed
- Semantic token highlighting for feature chains like `pwrCmd.pwrLevel` now highlights each identifier separately
- Semantic token highlighting for qualified names like `SysML::Usage` works correctly with stdlib
- Token span calculation no longer includes trailing whitespace
- Path normalization ensures consistent file matching between symbol table and reference index

### Added
- Path utilities module (`path_utils.rs`) for consistent file path handling across the codebase
- Separate reference extraction for feature chain parts (each identifier gets its own span)
- TokenType enum for semantic token classification (Type, Property, etc.)

### Improved
- Grammar rules `owned_feature_chain` and `qualified_name` are now atomic for precise span calculation
- Quote stripping for qualified names with quoted parts like `'Foo'::'Bar'`
- Enhanced reference indexing with proper token type classification

## [0.1.6-alpha] - 2026-01-09

### Improved
- Consolidated KerML grammar with unified rules for better maintainability
- Added unified patterns: `any_relationship`, `feature_or_chain`, `classifier_relationships`, `ordering_modifiers`, `feature_prefix_modifiers`, `connector_feature_modifiers`, `type_body`, `feature_declaration`
- Synchronized all keywords across parser, code completion, and syntax highlighting
- Added missing keywords: `meta`, `multiplicity`, `new`, `var`, `variation`, `references`

### Fixed
- Fixed `readonly` and `derived` feature modifier detection in AST parsing
- Corrected multiple test cases with invalid KerML syntax

## [0.1.5-alpha] - 2026-01-08

### Fixed
- Code formatting and import ordering consistency across codebase

## [0.1.4-alpha] - 2026-01-08

### Added
- Grammar support for `satisfy X by Y` requirement satisfaction syntax
- Grammar support for measurement references `value@[unit]` (e.g., `5@[SI::kg]`)
- Grammar support for interface end crosses operator `=>` (e.g., `port => Target::input`)
- Grammar support for function/calculation definitions with parameters and return types
- Grammar support for action usages with parameter lists

### Fixed
- Workspace loader now continues processing files when one fails to parse
- Hover information now works correctly on quoted names (e.g., `'Data Packet'`)

### Improved
- Better error resilience when loading workspaces with syntax errors

## [0.1.3-alpha] - 2026-01-08

### Added
- Extension icon for marketplace and sidebar display

### Fixed
- Corrected icon path in package.json

## [0.1.2-alpha] - 2026-01-08

### Fixed
- Critical bug fix for extension activation
- Corrected Language Server initialization

## [0.1.1-alpha] - 2026-01-07

### Added
- Initial public release
- SysML v2 syntax highlighting
- Code completion for keywords
- Go to definition
- Find references
- Hover documentation
- Document outline
- Folding ranges
- Semantic token highlighting

## [0.1.0-alpha] - 2026-01-06

### Added
- Project scaffolding
- Basic grammar parsing
