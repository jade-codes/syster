import React from 'react';
import { NodeProps } from 'reactflow';
import { PortNodeData } from '../types.js';

/**
 * PortNode component for rendering SysML port elements in diagrams.
 * Displays port direction and type information.
 */
export const PortNode: React.FC<NodeProps<PortNodeData>> = ({ data }) => {
  return (
    <div className="port-node">
      <div className="port-node-direction">{data.direction || 'inout'}</div>
      <div className="port-node-label">{data.label}</div>
      {data.portType && (
        <div className="port-node-type">{data.portType}</div>
      )}
    </div>
  );
};
