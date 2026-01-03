import React, { useMemo } from 'react';
import ReactFlow, { Background, Controls } from 'reactflow';
import 'reactflow/dist/style.css';
import type { Diagram } from '@syster/diagram-core';
import { NODE_TYPES } from '@syster/diagram-core';
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
    type: 'smoothstep',
    animated: true,
    style: { stroke: '#64748b', strokeWidth: 2 },
    markerEnd: 'arrowclosed' as const,
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
