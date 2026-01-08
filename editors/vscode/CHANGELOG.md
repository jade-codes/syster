# Changelog

All notable changes to the "SysML v2 Language Support" extension will be documented in this file.

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
- Fixed previous release date in changelog

## [0.1.1-alpha] - 2026-01-08

### Changed
- LSP server binary is now bundled with the extension (no separate installation required)
- SysML v2 standard library is now bundled with the extension
- Updated README to reflect bundled installation

### Improved
- LSP performance optimizations

### Fixed
- Simplified setup process - extension works out of the box

## [0.1.0-alpha] - 2026-01-06

### Added
- Initial release
- Basic syntax highlighting for comments and strings
- LSP-based language features:
  - Diagnostics (errors and warnings)
  - Hover information
  - Go to definition
  - Find references
  - Code completion
  - Document symbols (outline)
  - Rename symbol
  - Semantic tokens (rich syntax coloring)
- Support for both `.sysml` and `.kerml` files
- Auto-detection of `syster-lsp` binary
- "Restart Language Server" command
- Status bar indicator
- Configuration options for LSP path, tracing, and stdlib
