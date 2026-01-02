import React from 'react';
import ReactFlow, { Background, Controls } from 'reactflow';
import 'reactflow/dist/style.css';
import type { Diagram } from '@syster/diagram-core';

interface ViewerProps {
  diagram?: Diagram;
}

/**
 * Read-only viewer component for SysML v2 diagrams.
 * Renders diagrams using React Flow with an empty canvas by default.
 */
export const Viewer: React.FC<ViewerProps> = ({ diagram }) => {
  // Convert diagram to React Flow nodes and edges format
  const nodes = diagram?.nodes.map(node => ({
    id: node.id,
    type: node.type,
    data: { label: node.label || node.id },
    position: { x: 0, y: 0 }, // TODO: Add positioning logic
  })) || [];

  const edges = diagram?.edges.map(edge => ({
    id: edge.id,
    source: edge.source,
    target: edge.target,
    type: edge.type,
  })) || [];

  return (
    <div style={{ width: '100%', height: '100vh' }}>
      <ReactFlow
        nodes={nodes}
        edges={edges}
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
