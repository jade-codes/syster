import { FC } from 'react';
import { SysMLNode } from './SysMLNode';
import { NodeConfig, NODE_CONFIGS } from './nodeConfig';
import type { SymbolData } from '@syster/diagram-core';

/**
 * Props for SysML node components.
 * Compatible with React Flow's node component requirements.
 */
interface SysMLNodeProps {
  id: string;
  data: SymbolData;
}

/**
 * Creates a definition node component from configuration.
 * 
 * This factory function generates React Flow node components for each
 * SysML node type, using the provided configuration to customize
 * appearance (border color, stereotype label, etc.).
 * 
 * @param config - Visual configuration for the node type
 * @returns A React Flow node component
 * 
 * @example
 * ```tsx
 * const MyPartDefNode = createSysMLNode({
 *   borderColor: '#2563eb',
 *   stereotype: 'part def',
 *   showFeatures: true,
 * });
 * ```
 */
export function createSysMLNode(config: NodeConfig): FC<SysMLNodeProps> {
  const NodeComponent: FC<SysMLNodeProps> = ({ id, data }) => (
    <SysMLNode
      id={id}
      data={data}
      borderColor={config.borderColor}
      stereotype={config.stereotype}
      showFeatures={config.showFeatures}
      showDirection={config.showDirection}
    />
  );
  NodeComponent.displayName = `${config.stereotype.replace(/\s+/g, '')}Node`;
  return NodeComponent;
}

/**
 * Generated React Flow node types map.
 * 
 * Maps each SysML node type string to its corresponding React component.
 * Use this object directly with React Flow's `nodeTypes` prop.
 * 
 * @example
 * ```tsx
 * import { nodeTypes } from '@syster/diagram-ui';
 * 
 * <ReactFlow
 *   nodes={nodes}
 *   edges={edges}
 *   nodeTypes={nodeTypes}
 * />
 * ```
 */
export const nodeTypes: Record<string, FC<SysMLNodeProps>> = Object.fromEntries(
  Object.entries(NODE_CONFIGS).map(([type, config]) => [type, createSysMLNode(config)])
);

/**
 * Gets the configuration for a specific node type.
 * 
 * @param nodeType - The SysML node type string
 * @returns The node configuration, or undefined if not found
 */
export function getNodeConfig(nodeType: string): NodeConfig | undefined {
  return NODE_CONFIGS[nodeType];
}
