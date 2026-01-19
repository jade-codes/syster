# Syster

**Status: Alpha** - Active development, APIs may change

A Rust-based parser and tooling for SysML v2 (Systems Modeling Language) and KerML (Kernel Modeling Language).

## Meta-Repository Structure

This is a **meta-repository** that aggregates all Syster components via Git submodules. Each component lives in its own repository for independent development and versioning.

### Rust Crates

| Component | Repository | Description |
|-----------|------------|-------------|
| **syster-base** | [jade-codes/syster-base](https://github.com/jade-codes/syster-base) | Core library with parser, AST, and semantic analysis |
| **syster-cli** | [jade-codes/syster-cli](https://github.com/jade-codes/syster-cli) | Command-line tool for analyzing SysML/KerML files |
| **syster-lsp** | [jade-codes/syster-lsp](https://github.com/jade-codes/syster-lsp) | Language Server Protocol implementation with VS Code extension |

### TypeScript Packages

| Component | Repository | Description |
|-----------|------------|-------------|
| **@syster/diagram-core** | [jade-codes/syster-diagram-core](https://github.com/jade-codes/syster-diagram-core) | Core diagram types and layout algorithms |
| **@syster/diagram-ui** | [jade-codes/syster-diagram-ui](https://github.com/jade-codes/syster-diagram-ui) | React Flow UI components for diagrams |

### VS Code Extensions

| Extension | Repository | Description |
|-----------|------------|-------------|
| **sysml-language-support** | [jade-codes/syster-vscode-lsp](https://github.com/jade-codes/syster-vscode-lsp) | Main language support extension (LSP client) |
| **syster-viewer** | [jade-codes/syster-vscode-viewer](https://github.com/jade-codes/syster-vscode-viewer) | Diagram viewer extension |
| **syster-modeller** | [jade-codes/syster-vscode-modeller](https://github.com/jade-codes/syster-vscode-modeller) | Diagram modeller extension |

### Infrastructure

| Component | Repository | Description |
|-----------|------------|-------------|
| **syster-pipelines** | [jade-codes/syster-pipelines](https://github.com/jade-codes/syster-pipelines) | CI/CD pipeline templates |

## Getting Started

### Dev Container (Recommended)

This repository includes a VS Code dev container with all development tools pre-installed:

1. Open the repository in VS Code
2. When prompted, click "Reopen in Container" (or run `Dev Containers: Reopen in Container` from the command palette)
3. The container will automatically:
   - Initialize all git submodules
   - Install Rust, Node.js, and Bun
   - Set up the VS Code LSP extension dependencies

### Clone with Submodules

```bash
# Clone with all submodules
git clone --recurse-submodules https://github.com/jade-codes/syster.git

# Or if already cloned, initialize submodules
git submodule update --init --recursive
```

### Build Rust Crates

Each crate is independent - build separately:

```bash
(cd crates/syster-base && cargo build && cargo test)
(cd crates/syster-cli && cargo build)
(cd crates/syster-lsp && cargo build)
```

### Build TypeScript Packages

```bash
(cd editors/vscode-lsp && npm install && npm run esbuild)
(cd packages/diagram-core && bun install)
(cd packages/diagram-ui && bun install)
```

### Running the VS Code Extension Locally

1. Build the LSP binary:
   ```bash
   (cd crates/syster-lsp && cargo build --release)
   ```

2. Build the extension:
   ```bash
   (cd editors/vscode-lsp && npm install && npm run esbuild)
   ```

3. Press `F5` in VS Code to launch the extension in a new window

## Documentation

Documentation lives in each component's repository:

- **[syster-base](https://github.com/jade-codes/syster-base)** - Core architecture, SysML primer, contributing guide
- **[syster-lsp](https://github.com/jade-codes/syster-lsp)** - LSP features and VS Code extension usage

## License

MIT License - see [LICENSE.md](LICENSE.md)
