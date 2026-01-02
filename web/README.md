# Web Packages

This directory contains web-based packages and tools for Syster.

## Status

🚧 **Under Construction** - This directory is currently a placeholder for future web packages.

## CI/CD

The web packages have their own CI workflow (`.github/workflows/web.yml`) that runs:
- Bun installation and dependency caching
- Linting
- Type checking
- Testing
- Building

The workflow is triggered on changes to files in the `web/` directory.

## Getting Started

Currently, this is a minimal scaffold. To run the placeholder scripts:

```bash
cd web
bun install
bun run lint
bun run type-check
bun run test
bun run build
```

## Future Plans

This directory will contain:
- Web-based SysML v2 editors and viewers
- Browser-based parsers and analysis tools
- Interactive documentation and examples
