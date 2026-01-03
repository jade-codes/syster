/**
 * @syster/diagram-ui
 * 
 * Shared React components for SysML v2 diagram visualization.
 * Used by both the viewer (read-only) and modeller (editable) packages.
 */

// Base component
export { DefinitionNode } from './nodes';
export type { DefinitionNodeProps } from './nodes';

// Node factory and configuration
export { NODE_CONFIGS } from './nodes/nodeConfig';
export type { NodeConfig } from './nodes/nodeConfig';

export {
  createDefinitionNode,
  nodeTypes,
  getNodeConfig,
} from './nodes/nodeFactory';
