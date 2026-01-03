# @syster/diagram-ui

Shared React components for SysML v2 diagram visualization.

## Overview

This package provides reusable React components for rendering SysML v2 diagram elements. It's used by both:
- `@syster/viewer` - Read-only diagram viewer
- `@syster/modeller` - Interactive diagram editor

## Installation

```bash
bun add @syster/diagram-ui
```

## Components

### Node Components

- **DefinitionNode** - Base node component for SysML definitions
- **PartDefNode** - Part definition node (`«part def»`)
- **PortDefNode** - Port definition node (`«port def»`)

## Usage

```tsx
import { PartDefNode, PortDefNode } from '@syster/diagram-ui';
import { NODE_TYPES } from '@syster/diagram-core';

// Register with React Flow
const nodeTypes = {
  [NODE_TYPES.PART_DEF]: PartDefNode,
  [NODE_TYPES.PORT_DEF]: PortDefNode,
};

// Use in ReactFlow
<ReactFlow nodes={nodes} nodeTypes={nodeTypes} />
```

## Development

```bash
# Run tests
bun test

# Type check
bun run typecheck
```

## Dependencies

- `@syster/diagram-core` - Types and converters
- `react` - React library
- `reactflow` - React Flow library
