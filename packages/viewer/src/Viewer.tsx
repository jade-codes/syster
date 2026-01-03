import React, { useMemo } from 'react';
import ReactFlow, { Background, Controls, MarkerType } from 'reactflow';
import 'reactflow/dist/style.css';
import type { Diagram } from '@syster/diagram-core';
import { NODE_TYPES, EDGE_TYPES } from '@syster/diagram-core';
import { PartDefNode } from './nodes/PartDefNode';
import { PortDefNode } from './nodes/PortDefNode';

interface ViewerProps {
  diagram?: Diagram;
}

// Register custom node types with React Flow
const nodeTypes = {
  [NODE_TYPES.PART_DEF]: PartDefNode,
  [NODE_TYPES.PORT_DEF]: PortDefNode,
};

/**
 * Map SysML edge types to appropriate React Flow marker styles.
 * Different SysML relationships have different standard notations:
 * - Specialization: hollow triangle
 * - Typing: open arrow
 * - Composition: filled diamond
 * - Others: standard arrow
 */
const getMarkerEnd = (edgeType?: string) => {
  switch (edgeType) {
    case EDGE_TYPES.SPECIALIZATION:
      // Hollow triangle for inheritance
      return { type: MarkerType.ArrowClosed, color: '#64748b' };
    case EDGE_TYPES.COMPOSITION:
      // Filled diamond for composition
      return { type: MarkerType.ArrowClosed, color: '#64748b' };
    case EDGE_TYPES.TYPING:
    case EDGE_TYPES.SUBSETTING:
    case EDGE_TYPES.REDEFINITION:
      // Open arrow for typing and refinement relationships
      return { type: MarkerType.Arrow, color: '#64748b' };
    default:
      // Standard closed arrow for other relationships
      return { type: MarkerType.ArrowClosed, color: '#64748b' };
  }
};

/**
 * Read-only viewer component for SysML v2 diagrams.
 * Renders diagrams using React Flow with an empty canvas by default.
 */
export const Viewer: React.FC<ViewerProps> = ({ diagram }) => {
  // Convert diagram to React Flow nodes and edges format
  const nodes = useMemo(() => diagram?.nodes.map(node => ({
    id: node.id,
    type: node.type,
    data: node.data,
    position: node.position,
  })) || [], [diagram]);

  const edges = useMemo(() => diagram?.edges.map(edge => ({
    id: edge.id,
    source: edge.source,
    target: edge.target,
    type: edge.type ?? 'smoothstep',
    animated: true,
    style: { stroke: '#64748b', strokeWidth: 2 },
    markerEnd: getMarkerEnd(edge.type),
  })) || [], [diagram]);

  return (
    <div style={{ width: '100%', height: '100vh' }}>
      <ReactFlow
        nodes={nodes}
        edges={edges}
        nodeTypes={nodeTypes}
        fitView
        nodesDraggable={false}
        nodesConnectable={false}
        elementsSelectable={true}
      >
        <Background />
        <Controls />
      </ReactFlow>
    </div>
  );
};

export default Viewer;
