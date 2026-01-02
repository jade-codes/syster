# @syster/viewer

Read-only viewer for embedding SysML v2 diagrams.

## Features

- 📊 Empty React Flow canvas for diagram rendering
- 🔒 Read-only interaction (no editing)
- ⚛️ React-based component architecture
- 📦 Embeddable library output

## Development

Start the development server:

```bash
bun run dev
# or
./run-dev.sh
```

The dev server will start on http://localhost:3000

## Building

Build the library for distribution:

```bash
bun run build
# or
./run-build.sh
```

Output will be in the `dist/` directory:
- `Viewer.js` - Main library bundle
- `Viewer.css` - Styles
- `Viewer.js.map` - Source map

## Usage

```typescript
import { Viewer } from '@syster/viewer';
import type { Diagram } from '@syster/diagram-core';

// Empty canvas (default)
<Viewer />

// With diagram data
const diagram: Diagram = {
  nodes: [/* ... */],
  edges: [/* ... */]
};

<Viewer diagram={diagram} />
```

## Dependencies

- React 18.2+
- React Flow 11.10+
- @syster/diagram-core

**No LSP dependencies** - This is a pure UI library.

