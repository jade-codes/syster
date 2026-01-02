# Syster Web Packages

This document describes the web-related packages for the Syster project, located under the `packages/` directory and configured with Bun's built-in test runner.

## Prerequisites

Install Bun:
```bash
curl -fsSL https://bun.sh/install | bash
```

## Running Tests

From the repository root:

```bash
# Run all tests
bun test

# Run tests with coverage
bun test --coverage
```

From individual package directories:

```bash
# In packages/core/
cd packages/core && bun test

# In packages/utils/
cd packages/utils && bun test
```

## Package Structure

Each package includes:
- `package.json` - Package configuration with test script
- `bunfig.toml` - Bun test configuration with coverage settings
- `src/__tests__/index.test.ts` - Initial test file

## Test Configuration

Tests are configured via `bunfig.toml` in each package with:
- Coverage enabled by default
- Coverage output to `coverage/` directory
- Multiple coverage reporters: text, lcov, and html

## Workspace Structure

```
/ (repository root)
├── bunfig.toml           # Root test configuration
├── package.json          # Root package with workspace config
└── packages/
    ├── core/             # Core functionality package
    │   ├── bunfig.toml
    │   ├── package.json
    │   └── src/__tests__/index.test.ts
    └── utils/            # Utilities package
        ├── bunfig.toml
        ├── package.json
        └── src/__tests__/index.test.ts
```
