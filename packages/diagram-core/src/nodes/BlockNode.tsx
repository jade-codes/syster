import React from 'react';
import { NodeProps } from 'reactflow';
import { BlockNodeData } from '../types.js';

/**
 * BlockNode component for rendering SysML block elements in diagrams.
 * Supports various block types like parts, actions, and requirements.
 */
export const BlockNode: React.FC<NodeProps<BlockNodeData>> = ({ data }) => {
  return (
    <div className="block-node">
      <div className="block-node-label">{data.label}</div>
      {data.description && (
        <div className="block-node-description">{data.description}</div>
      )}
    </div>
  );
};
