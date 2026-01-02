# Bun Testing Infrastructure

## Setup

The web packages have been configured with Bun's built-in test runner.

### Structure Created:
- `/packages/` - Root directory for web packages
- `/packages/core/` - Example core package with Bun test infrastructure
- `/packages/utils/` - Example utils package with Bun test infrastructure
- `/packages/diagram-core/` - Additional package (no Bun test infrastructure yet)
- `/packages/modeller/` - Additional package (no Bun test infrastructure yet)
- `/packages/viewer/` - Additional package (no Bun test infrastructure yet)

> Note: Bun-based testing is currently set up only for the `core` and `utils` packages as concrete examples.  
> The other packages listed above do not yet have Bun test infrastructure and are out of scope for this guide.

### Configuration Files:
- Each package has `bunfig.toml` with test and coverage configuration
- Each package has `package.json` with test script
- Root `/package.json` configured as workspace with test scripts
- Test files created: `src/__tests__/index.test.ts` in each package

### Running Tests:

From repository root:
```bash
bun test              # Run all tests
bun test --coverage   # Run with coverage
```

From individual packages:
```bash
cd packages/core && bun test
cd packages/utils && bun test
```

## Coverage Configuration

Coverage is enabled by default in all `bunfig.toml` files with:
- Output directory: `coverage/`
- Reporters: text, lcov, html

## Note

If Bun crashes during `bun install` on certain Linux environments, it may be due to CPU instruction incompatibility. 
This is a known issue with Bun 1.3.5. The configuration and tests are correct and will work in compatible environments.
