# @syster/diagram-core

Shared diagram components with React Flow foundation for SysML v2 visualization.

## Installation

```bash
npm install @syster/diagram-core
```

## Features

- **React Flow Integration**: Built on React Flow for powerful diagram rendering
- **TypeScript Support**: Full type definitions included
- **SysML v2 Components**: Pre-built components for blocks and ports
- **Extensible**: Easy to create custom node types

## Usage

```tsx
import { ReactFlow, BlockNode, PortNode, DiagramNode } from '@syster/diagram-core';

const nodes: DiagramNode[] = [
  {
    id: '1',
    type: 'block',
    position: { x: 0, y: 0 },
    data: { label: 'Vehicle', blockType: 'part' }
  }
];

function MyDiagram() {
  return (
    <ReactFlow
      nodes={nodes}
      nodeTypes={{ block: BlockNode, port: PortNode }}
    />
  );
}
```

## Components

### BlockNode

Renders SysML block elements (parts, actions, requirements).

```tsx
<BlockNode data={{ label: 'MyBlock', blockType: 'part' }} />
```

### PortNode

Renders SysML port elements with direction indicators.

```tsx
<PortNode data={{ label: 'MyPort', direction: 'in' }} />
```

## Types

The package exports the following TypeScript types:

- `DiagramNode` - Base node type
- `DiagramNodeData` - Node data interface
- `DiagramEdge` - Base edge type
- `BlockNodeData` - Block-specific data
- `PortNodeData` - Port-specific data

## Building

```bash
bun run build
```

## License

MIT
