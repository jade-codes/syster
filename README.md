# Syster

**Status: Alpha** - Active development, APIs may change

A Rust-based parser and tooling for SysML v2 (Systems Modeling Language) and KerML (Kernel Modeling Language).

## Repository Structure

Feature-based organization with independent submodules for versioning flexibility.

```
syster/
├── core/              # Parser, AST, semantic analysis
├── cli/               # Command-line tool
├── lsp/
│   ├── server/        # Language Server Protocol implementation
│   └── vscode/        # VS Code LSP extension
├── modeller/
│   ├── core/          # Diagram types and layout (TypeScript)
│   ├── ui/            # React Flow components (TypeScript)
│   └── vscode/        # VS Code modeller extension
├── viewer/
│   └── vscode/        # VS Code viewer extension
└── pipelines/         # CI/CD pipeline templates
```

### Components

| Feature | Path | Repository | Description |
|---------|------|------------|-------------|
| **Core** | `core/` | [syster-base](https://github.com/jade-codes/syster-base) | Parser, AST, semantic analysis |
| **CLI** | `cli/` | [syster-cli](https://github.com/jade-codes/syster-cli) | Command-line tool |
| **LSP Server** | `lsp/server/` | [syster-lsp](https://github.com/jade-codes/syster-lsp) | Language Server Protocol |
| **LSP Extension** | `lsp/vscode/` | [syster-vscode-lsp](https://github.com/jade-codes/syster-vscode-lsp) | VS Code language support |
| **Diagram Core** | `modeller/core/` | [syster-diagram-core](https://github.com/jade-codes/syster-diagram-core) | Diagram types (TS) |
| **Diagram UI** | `modeller/ui/` | [syster-diagram-ui](https://github.com/jade-codes/syster-diagram-ui) | React Flow components |
| **Modeller** | `modeller/vscode/` | [syster-vscode-modeller](https://github.com/jade-codes/syster-vscode-modeller) | VS Code modeller |
| **Viewer** | `viewer/vscode/` | [syster-vscode-viewer](https://github.com/jade-codes/syster-vscode-viewer) | VS Code viewer |
| **Pipelines** | `pipelines/` | [syster-pipelines](https://github.com/jade-codes/syster-pipelines) | CI/CD templates |

## Getting Started

### Dev Container (Recommended)

This repository includes a VS Code dev container with all development tools pre-installed:

1. Open the repository in VS Code
2. When prompted, click "Reopen in Container" (or run `Dev Containers: Reopen in Container` from the command palette)
3. The container will automatically:
   - Initialize all git submodules
   - Install Rust, Node.js, and Bun
   - Set up dependencies

### Clone with Submodules

```bash
# Clone with all submodules
git clone --recurse-submodules https://github.com/jade-codes/syster.git

# Or if already cloned, initialize submodules
git submodule update --init --recursive
```

### Build

```bash
# Build all Rust crates from root
cargo build
cargo test

# Install TypeScript dependencies
bun install

# Setup VS Code extensions
npm run setup:lsp
npm run setup:modeller
npm run setup:viewer
```

### Running the VS Code Extension Locally

1. Build the LSP binary:
   ```bash
   cd crates/syster-lsp && cargo build --release
   ```

2. Build the extension:
   ```bash
   cd editors/vscode-lsp && npm install && npm run esbuild
   ```

3. Press `F5` in VS Code to launch the extension in a new window

## Documentation

Documentation lives in each component's repository:

- **[syster-base](https://github.com/jade-codes/syster-base)** - Core architecture, SysML primer, contributing guide
- **[syster-lsp](https://github.com/jade-codes/syster-lsp)** - LSP features and VS Code extension usage

## License

MIT License - see [LICENSE.md](LICENSE.md)
