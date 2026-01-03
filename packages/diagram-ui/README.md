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

### nodeTypes

Pre-configured React Flow node types for all 39 SysML element types:

- **Definitions**: part def, port def, action def, state def, requirement def, etc.
- **Usages**: part, port, action, requirement, etc.

### DefinitionNode

Base component used by all node types. Customizable via props.

## Usage

```tsx
import { nodeTypes } from '@syster/diagram-ui';

// Use directly with React Flow - all 39 node types are registered
<ReactFlow nodes={nodes} nodeTypes={nodeTypes} />
```

### Custom Node Types

```tsx
import { createDefinitionNode, NODE_CONFIGS } from '@syster/diagram-ui';

// Create a custom node type
const MyCustomNode = createDefinitionNode({
  borderColor: '#ff0000',
  stereotype: 'custom def',
  showFeatures: true,
});
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
- `@xyflow/react` - React Flow library (v12+)
