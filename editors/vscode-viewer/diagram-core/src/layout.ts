/**
 * Layout Configuration Types for Diagram Visualization
 * 
 * This module defines pure data types for configuring automatic layout
 * of SysML diagrams. These types are independent of any layout library
 * (dagre, elk, etc.) and define the contract for layout algorithms.
 * 
 * Key Concepts:
 * - LayoutConfig: Configuration for layout algorithm
 * - Position/Dimensions: Coordinate and size information
 * - PositionedNode/Edge: Elements with computed positions
 * - LayoutResult: Output from layout computation
 */

// ========== Constants ==========

/** Layout direction options for hierarchical layouts */
export const LAYOUT_DIRECTIONS = {
  TOP_BOTTOM: "TB",
  BOTTOM_TOP: "BT",
  LEFT_RIGHT: "LR",
  RIGHT_LEFT: "RL",
} as const;

/** Available layout algorithms */
export const LAYOUT_ALGORITHMS = {
  DAGRE: "dagre",
  ELK: "elk",
} as const;

// ========== Types ==========

/** Layout direction type */
export type LayoutDirection = typeof LAYOUT_DIRECTIONS[keyof typeof LAYOUT_DIRECTIONS];

/** Layout algorithm type */
export type LayoutAlgorithm = typeof LAYOUT_ALGORITHMS[keyof typeof LAYOUT_ALGORITHMS];

/** Configuration for automatic layout */
export interface LayoutConfig {
  /** Which layout algorithm to use */
  algorithm: LayoutAlgorithm;
  
  /** Direction of hierarchical layout */
  direction: LayoutDirection;
  
  /** Horizontal/vertical spacing between nodes (in pixels) */
  nodeSpacing?: number;
  
  /** Spacing between ranks/levels in hierarchy (in pixels) */
  rankSpacing?: number;
  
  /** Spacing for edge routing (in pixels) */
  edgeSpacing?: number;
}

/** 2D position coordinates */
export interface Position {
  /** X coordinate in pixels */
  x: number;
  
  /** Y coordinate in pixels */
  y: number;
}

/** 2D dimensions */
export interface Dimensions {
  /** Width in pixels */
  width: number;
  
  /** Height in pixels */
  height: number;
}

/** Node with computed position from layout algorithm */
export interface PositionedNode {
  /** Unique node identifier */
  id: string;
  
  /** Computed position */
  position: Position;
  
  /** Optional computed dimensions (may be calculated by layout) */
  dimensions?: Dimensions;
  
  /** Arbitrary data attached to the node */
  data: Record<string, unknown>;
}

/** Edge with source/target references */
export interface PositionedEdge {
  /** Unique edge identifier */
  id: string;
  
  /** Source node ID */
  source: string;
  
  /** Target node ID */
  target: string;
  
  /** Edge type (for styling) */
  type?: string;
  
  /** Optional label */
  label?: string;
}

/** Result from layout computation */
export interface LayoutResult {
  /** Positioned nodes */
  nodes: PositionedNode[];
  
  /** Edges with source/target references */
  edges: PositionedEdge[];
  
  /** Optional overall graph dimensions */
  dimensions?: Dimensions;
}

// ========== Helper Functions ==========

/** Create a default layout configuration with sensible defaults */
export function createDefaultLayoutConfig(overrides?: Partial<LayoutConfig>): LayoutConfig {
  return {
    algorithm: LAYOUT_ALGORITHMS.DAGRE,
    direction: LAYOUT_DIRECTIONS.TOP_BOTTOM,
    nodeSpacing: 50,
    rankSpacing: 100,
    ...overrides,
  };
}
