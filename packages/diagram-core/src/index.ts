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
}

export interface Diagram {
  nodes: DiagramNode[];
  edges: DiagramEdge[];
}

export const createEmptyDiagram = (): Diagram => ({
  nodes: [],
  edges: [],
});
