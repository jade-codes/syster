import React, { useMemo } from 'react';
import { ReactFlow, Background, Controls } from '@xyflow/react';
import '@xyflow/react/dist/style.css';
import type { Diagram } from '@syster/diagram-core';
import { nodeTypes, edgeTypes } from '@syster/diagram-ui';

interface ViewerProps {
  diagram?: Diagram;
}

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
    type: edge.type,
    data: {
      label: edge.label,
      multiplicity: 'multiplicity' in edge ? edge.multiplicity : undefined,
    },
  })) || [], [diagram]);

  return (
    <div style={{ width: '100%', height: '100vh' }}>
      <ReactFlow
        nodes={nodes}
        edges={edges}
        nodeTypes={nodeTypes}
        edgeTypes={edgeTypes}
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
