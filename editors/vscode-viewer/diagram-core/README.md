# @syster/diagram-core

Core diagram types and utilities for Syster SysML v2 modeller.

## Overview

This package provides the core types and utilities for working with SysML v2 diagrams.

## Status

This is currently a stub package with minimal types defined. It will be expanded as the modeller implementation progresses.

## Types

- `DiagramNode` - Represents a node in a diagram
- `DiagramEdge` - Represents an edge between nodes
- `Diagram` - Represents a complete diagram with nodes and edges

## Usage

```typescript
import { createEmptyDiagram, DiagramNode, DiagramEdge } from '@syster/diagram-core';

const diagram = createEmptyDiagram();
```
