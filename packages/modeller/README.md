# @syster/modeller

SysML v2 visual modelling tool with editing capabilities.

## Features

- React-based visual modelling canvas using React Flow
- Empty sidebar panels for element palette and properties
- LSP connection hook (stubbed for now)
- Standalone app with hot-reload development server

## Development

```bash
# Install dependencies
npm install

# Start development server
bun run dev

# Build for production
bun run build
```

The dev server will start at http://localhost:3000

## Structure

- `src/index.tsx` - Entry point
- `src/Modeller.tsx` - Main modeller component with React Flow canvas
- `src/components/Palette.tsx` - Empty palette sidebar (to be implemented)
- `src/components/PropertiesPanel.tsx` - Empty properties panel (to be implemented)
- `src/hooks/useLspConnection.ts` - LSP connection hook (stub with mock data)

## Status

This is a scaffolded package with basic structure in place. The following features are stubbed:

- Element palette (empty)
- Properties panel (empty)
- LSP connection (returns mock data)

## Next Steps

- Implement drag-and-drop element palette
- Implement properties editing interface
- Connect to real syster-lsp server
- Add SysML v2 element types to the canvas
