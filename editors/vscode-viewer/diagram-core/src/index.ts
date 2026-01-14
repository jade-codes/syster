/**
 * @syster/diagram-core
 * 
 * Core diagram types and utilities for Syster SysML v2 modeller.
 * This is a stub package that will be implemented later.
 */

export interface DiagramNode {
  id: string;
  type: string;
  position: { x: number; y: number };
  data: Record<string, unknown>;
}

export interface DiagramEdge {
  id: string;
  source: string;
  target: string;
  type?: string;
  /** Optional label to display on edge */
  label?: string;
  /** Multiplicity for composition edges (e.g., "4", "*") */
  multiplicity?: string;
}

export interface Diagram {
  nodes: DiagramNode[];
  edges: DiagramEdge[];
}

export const createEmptyDiagram = (): Diagram => ({
  nodes: [],
  edges: [],
});

// Re-export SysML types for convenience
export { NODE_TYPES, FEATURE_DIRECTIONS } from "./sysml-nodes";
export { EDGE_TYPES } from "./sysml-edges";
export type { SysMLNode } from "./sysml-nodes";
export type { SysMLEdge } from "./sysml-edges";

// Re-export converter functions
export { 
  convertSymbolToNode, 
  convertRelationshipToEdge, 
  createDiagramFromWorkspace 
} from "./converter";
export type { 
  SymbolData, 
  RelationshipData, 
  WorkspaceData 
} from "./converter";

// Re-export layout types
export type { 
  LayoutConfig, 
  Position, 
  Dimensions, 
  PositionedNode, 
  PositionedEdge, 
  LayoutResult 
} from "./layout";
export { LAYOUT_DIRECTIONS, LAYOUT_ALGORITHMS, createDefaultLayoutConfig } from "./layout";
