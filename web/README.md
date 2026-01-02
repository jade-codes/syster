# Syster Web Workspace

This workspace contains TypeScript packages for SysML v2 visualization and editing.

## Packages

- **@syster/diagram-core** - Shared rendering, node types, and layout for SysML diagrams
- **@syster/viewer** - Read-only, embeddable viewer for SysML diagrams
- **@syster/modeller** - Full editing experience with LSP integration

## Getting Started

```bash
# Install dependencies
pnpm install

# Build all packages
pnpm run build

# Run development mode (watch)
pnpm run dev

# Run tests
pnpm run test

# Lint code
pnpm run lint

# Format code
pnpm run format
```

## Development

This workspace uses pnpm workspaces for package management. Each package can be developed independently or as part of the monorepo.

### Package Structure

Each package follows the same structure:
```
packages/<package-name>/
├── src/
│   └── index.ts
├── dist/          # Built output
├── package.json
└── tsconfig.json
```

### Shared Configuration

- `tsconfig.base.json` - Base TypeScript configuration
- `.eslintrc.js` - ESLint configuration
- `.prettierrc.json` - Prettier configuration

## License

MIT
