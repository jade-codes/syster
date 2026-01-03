import React, { useMemo } from 'react';
import ReactFlow, { Background, Controls, MarkerType } from 'reactflow';
import 'reactflow/dist/style.css';
import type { Diagram } from '@syster/diagram-core';
import { NODE_TYPES, EDGE_TYPES } from '@syster/diagram-core';
import { PartDefNode, PortDefNode } from '@syster/diagram-ui';

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
 * Different SysML relationships have different standard notations.
 * Note: React Flow has limited built-in marker types. For full SysML compliance,
 * custom markers would be needed (e.g., hollow triangles, filled diamonds).
 * Currently using available marker types as semantic indicators:
 * - Specialization: ArrowClosed (represents inheritance)
 * - Composition: ArrowClosed (represents containment)
 * - Typing/Subsetting/Redefinition: Arrow (represents refinement relationships)
 * - Others: ArrowClosed (default for relationships)
 */
const getMarkerEnd = (edgeType?: string) => {
  switch (edgeType) {
    case EDGE_TYPES.SPECIALIZATION:
    case EDGE_TYPES.COMPOSITION:
      // Closed arrow for inheritance and composition relationships
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
