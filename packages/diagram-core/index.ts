// Placeholder for diagram core types and utilities
// This will contain shared types and logic for SysML v2 diagrams

export interface DiagramNode {
  id: string;
  type: string;
  label?: string;
}

export interface DiagramEdge {
  id: string;
  source: string;
  target: string;
  type?: string;
}

export interface Diagram {
  nodes: DiagramNode[];
  edges: DiagramEdge[];
}