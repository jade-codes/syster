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
| **syster-viewer** | [jade-codes/syster-viewer](https://github.com/jade-codes/syster-viewer) | Diagram viewer extension |
| **syster-modeller** | [jade-codes/syster-modeller](https://github.com/jade-codes/syster-modeller) | Diagram modeller extension |

### Infrastructure

| Component | Repository | Description |
|-----------|------------|-------------|
| **syster-pipelines** | [jade-codes/syster-pipelines](https://github.com/jade-codes/syster-pipelines) | CI/CD pipeline templates |

## Getting Started

### Clone with Submodules

\`\`\`bash
# Clone with all submodules
git clone --recurse-submodules https://github.com/jade-codes/syster.git

# Or if already cloned, initialize submodules
git submodule update --init --recursive
\`\`\`

### Build Rust Crates

Each crate is independent - build separately:

\`\`\`bash
cd crates/syster-base && cargo build && cargo test
cd crates/syster-cli && cargo build
cd crates/syster-lsp && cargo build
\`\`\`

### Build TypeScript Packages

\`\`\`bash
npm install
npm run build:packages
\`\`\`

## Documentation

Documentation lives in each component's repository:

- **[syster-base](https://github.com/jade-codes/syster-base)** - Core architecture, SysML primer, contributing guide
- **[syster-lsp](https://github.com/jade-codes/syster-lsp)** - LSP features and VS Code extension usage

## License

MIT License - see [LICENSE.md](LICENSE.md)
