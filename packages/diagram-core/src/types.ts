import { Node, Edge } from 'reactflow';

/**
 * Base diagram node type extending React Flow's Node
 */
export interface DiagramNode extends Node {
  /** Node type identifier */
  type: string;
  /** Node data payload */
  data: DiagramNodeData;
}

/**
 * Data payload for diagram nodes
 */
export interface DiagramNodeData {
  /** Display label for the node */
  label: string;
  /** Optional description */
  description?: string;
  /** Node-specific custom properties */
  [key: string]: any;
}

/**
 * Base diagram edge type
 */
export type DiagramEdge = Edge & {
  /** Edge label */
  label?: string;
  /** Edge-specific custom properties */
  data?: {
    [key: string]: any;
  };
};

/**
 * Block node data specific properties
 */
export interface BlockNodeData extends DiagramNodeData {
  /** Block type (e.g., 'part', 'action', 'requirement') */
  blockType?: string;
}

/**
 * Port node data specific properties
 */
export interface PortNodeData extends DiagramNodeData {
  /** Port direction (in, out, inout) */
  direction?: 'in' | 'out' | 'inout';
  /** Port type */
  portType?: string;
}
