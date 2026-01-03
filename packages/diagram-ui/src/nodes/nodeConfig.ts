import { NODE_TYPES } from '@syster/diagram-core';

/**
 * Configuration for a SysML node's visual appearance.
 */
export interface NodeConfig {
  /** Border color for the node */
  borderColor: string;
  /** Stereotype label (e.g., "part def", "port def") */
  stereotype: string;
  /** Whether to show features list */
  showFeatures?: boolean;
  /** Whether to show direction indicator */
  showDirection?: boolean;
}

/**
 * SysML v2 node type configurations.
 * 
 * Color scheme:
 * - Blue (#2563eb): Structural definitions (part, item)
 * - Purple (#7c3aed): Ports and interfaces
 * - Green (#059669): Behavioral (actions, states)
 * - Orange (#d97706): Requirements and constraints
 * - Indigo (#4f46e5): Cases (use case, analysis, verification)
 * - Teal (#0d9488): Views and viewpoints
 * - Slate (#475569): Other/miscellaneous
 * 
 * Usages use the same color as their corresponding definitions.
 */
export const NODE_CONFIGS: Record<string, NodeConfig> = {
  // ========== Definitions ==========
  
  // Structural
  [NODE_TYPES.PART_DEF]: {
    borderColor: '#2563eb',
    stereotype: 'part def',
    showFeatures: true,
  },
  [NODE_TYPES.ITEM_DEF]: {
    borderColor: '#2563eb',
    stereotype: 'item def',
    showFeatures: true,
  },
  [NODE_TYPES.ATTRIBUTE_DEF]: {
    borderColor: '#2563eb',
    stereotype: 'attribute def',
    showFeatures: true,
  },
  [NODE_TYPES.INDIVIDUAL_DEF]: {
    borderColor: '#2563eb',
    stereotype: 'individual def',
    showFeatures: true,
  },
  [NODE_TYPES.OCCURRENCE_DEF]: {
    borderColor: '#2563eb',
    stereotype: 'occurrence def',
    showFeatures: true,
  },
  
  // Ports and Interfaces
  [NODE_TYPES.PORT_DEF]: {
    borderColor: '#7c3aed',
    stereotype: 'port def',
    showDirection: true,
  },
  [NODE_TYPES.INTERFACE_DEF]: {
    borderColor: '#7c3aed',
    stereotype: 'interface def',
    showFeatures: true,
  },
  [NODE_TYPES.CONNECTION_DEF]: {
    borderColor: '#7c3aed',
    stereotype: 'connection def',
    showFeatures: true,
  },
  [NODE_TYPES.FLOW_DEF]: {
    borderColor: '#7c3aed',
    stereotype: 'flow def',
    showDirection: true,
  },
  
  // Behavioral
  [NODE_TYPES.ACTION_DEF]: {
    borderColor: '#059669',
    stereotype: 'action def',
    showFeatures: true,
  },
  [NODE_TYPES.STATE_DEF]: {
    borderColor: '#059669',
    stereotype: 'state def',
    showFeatures: true,
  },
  [NODE_TYPES.CALCULATION_DEF]: {
    borderColor: '#059669',
    stereotype: 'calculation def',
    showFeatures: true,
  },
  
  // Requirements and Constraints
  [NODE_TYPES.REQUIREMENT_DEF]: {
    borderColor: '#d97706',
    stereotype: 'requirement def',
    showFeatures: true,
  },
  [NODE_TYPES.CONCERN_DEF]: {
    borderColor: '#d97706',
    stereotype: 'concern def',
    showFeatures: true,
  },
  [NODE_TYPES.CONSTRAINT_DEF]: {
    borderColor: '#d97706',
    stereotype: 'constraint def',
    showFeatures: true,
  },
  
  // Cases
  [NODE_TYPES.CASE_DEF]: {
    borderColor: '#4f46e5',
    stereotype: 'case def',
    showFeatures: true,
  },
  [NODE_TYPES.USE_CASE_DEF]: {
    borderColor: '#4f46e5',
    stereotype: 'use case def',
    showFeatures: true,
  },
  [NODE_TYPES.ANALYSIS_CASE_DEF]: {
    borderColor: '#4f46e5',
    stereotype: 'analysis case def',
    showFeatures: true,
  },
  [NODE_TYPES.VERIFICATION_CASE_DEF]: {
    borderColor: '#4f46e5',
    stereotype: 'verification case def',
    showFeatures: true,
  },
  
  // Views
  [NODE_TYPES.VIEW_DEF]: {
    borderColor: '#0d9488',
    stereotype: 'view def',
    showFeatures: true,
  },
  [NODE_TYPES.VIEWPOINT_DEF]: {
    borderColor: '#0d9488',
    stereotype: 'viewpoint def',
    showFeatures: true,
  },
  [NODE_TYPES.RENDERING_DEF]: {
    borderColor: '#0d9488',
    stereotype: 'rendering def',
    showFeatures: true,
  },
  
  // Other
  [NODE_TYPES.ALLOCATION_DEF]: {
    borderColor: '#475569',
    stereotype: 'allocation def',
    showFeatures: true,
  },
  [NODE_TYPES.ENUMERATION_DEF]: {
    borderColor: '#475569',
    stereotype: 'enumeration def',
    showFeatures: true,
  },
  [NODE_TYPES.METADATA_DEF]: {
    borderColor: '#475569',
    stereotype: 'metadata def',
    showFeatures: true,
  },
  
  // ========== Usages ==========
  
  // Structural
  [NODE_TYPES.PART_USAGE]: {
    borderColor: '#2563eb',
    stereotype: 'part',
    showFeatures: true,
  },
  [NODE_TYPES.ITEM_USAGE]: {
    borderColor: '#2563eb',
    stereotype: 'item',
    showFeatures: true,
  },
  [NODE_TYPES.ATTRIBUTE_USAGE]: {
    borderColor: '#2563eb',
    stereotype: 'attribute',
    showFeatures: true,
  },
  
  // Ports
  [NODE_TYPES.PORT_USAGE]: {
    borderColor: '#7c3aed',
    stereotype: 'port',
    showDirection: true,
  },
  
  // Behavioral
  [NODE_TYPES.ACTION_USAGE]: {
    borderColor: '#059669',
    stereotype: 'action',
    showFeatures: true,
  },
  [NODE_TYPES.PERFORM_ACTION_USAGE]: {
    borderColor: '#059669',
    stereotype: 'perform',
    showFeatures: true,
  },
  [NODE_TYPES.EXHIBIT_STATE_USAGE]: {
    borderColor: '#059669',
    stereotype: 'exhibit',
    showFeatures: true,
  },
  
  // Requirements
  [NODE_TYPES.REQUIREMENT_USAGE]: {
    borderColor: '#d97706',
    stereotype: 'requirement',
    showFeatures: true,
  },
  [NODE_TYPES.CONCERN_USAGE]: {
    borderColor: '#d97706',
    stereotype: 'concern',
    showFeatures: true,
  },
  [NODE_TYPES.SATISFY_REQUIREMENT_USAGE]: {
    borderColor: '#d97706',
    stereotype: 'satisfy',
    showFeatures: true,
  },
  
  // Cases
  [NODE_TYPES.CASE_USAGE]: {
    borderColor: '#4f46e5',
    stereotype: 'case',
    showFeatures: true,
  },
  [NODE_TYPES.INCLUDE_USE_CASE_USAGE]: {
    borderColor: '#4f46e5',
    stereotype: 'include',
    showFeatures: true,
  },
  
  // Views
  [NODE_TYPES.VIEW_USAGE]: {
    borderColor: '#0d9488',
    stereotype: 'view',
    showFeatures: true,
  },
  
  // Other
  [NODE_TYPES.ENUMERATION_USAGE]: {
    borderColor: '#475569',
    stereotype: 'enum',
    showFeatures: true,
  },
};
