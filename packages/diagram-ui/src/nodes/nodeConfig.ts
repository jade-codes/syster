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

// ========== Color Palette ==========
// Consistent with Tailwind CSS color scheme

const COLORS = {
  /** Structural definitions (part, item, attribute) */
  structural: '#2563eb',    // blue-600
  /** Ports, interfaces, connections, flows */
  interface: '#7c3aed',     // violet-600  
  /** Actions, states, calculations */
  behavioral: '#059669',    // emerald-600
  /** Requirements, concerns, constraints */
  requirement: '#d97706',   // amber-600
  /** Cases (use case, analysis, verification) */
  case: '#4f46e5',          // indigo-600
  /** Views, viewpoints, rendering */
  view: '#0d9488',          // teal-600
  /** Other/miscellaneous */
  other: '#475569',         // slate-600
} as const;

// ========== Config Builders ==========

/** Create a definition config with features */
const def = (color: string, name: string): NodeConfig => ({
  borderColor: color,
  stereotype: `${name} def`,
  showFeatures: true,
});

/** Create a definition config with direction (for ports) */
const defDir = (color: string, name: string): NodeConfig => ({
  borderColor: color,
  stereotype: `${name} def`,
  showDirection: true,
});

/** Create a usage config with features */
const usage = (color: string, name: string): NodeConfig => ({
  borderColor: color,
  stereotype: name,
  showFeatures: true,
});

/** Create a usage config with direction (for ports, refs) */
const usageDir = (color: string, name: string): NodeConfig => ({
  borderColor: color,
  stereotype: name,
  showDirection: true,
});

// ========== Node Configurations ==========

/**
 * SysML v2 node type configurations.
 * 
 * Organized by category, with definitions and usages grouped together.
 * Each category uses a consistent color for visual grouping.
 */
export const NODE_CONFIGS: Record<string, NodeConfig> = {
  // ==================== Structural (Blue) ====================
  // Parts, items, attributes, occurrences, individuals
  
  [NODE_TYPES.PART_DEF]:           def(COLORS.structural, 'part'),
  [NODE_TYPES.PART_USAGE]:         usage(COLORS.structural, 'part'),
  
  [NODE_TYPES.ITEM_DEF]:           def(COLORS.structural, 'item'),
  [NODE_TYPES.ITEM_USAGE]:         usage(COLORS.structural, 'item'),
  
  [NODE_TYPES.ATTRIBUTE_DEF]:      def(COLORS.structural, 'attribute'),
  [NODE_TYPES.ATTRIBUTE_USAGE]:    usage(COLORS.structural, 'attribute'),
  
  [NODE_TYPES.OCCURRENCE_DEF]:     def(COLORS.structural, 'occurrence'),
  [NODE_TYPES.OCCURRENCE_USAGE]:   usage(COLORS.structural, 'occurrence'),
  
  [NODE_TYPES.INDIVIDUAL_DEF]:     def(COLORS.structural, 'individual'),
  [NODE_TYPES.INDIVIDUAL_USAGE]:   usage(COLORS.structural, 'individual'),
  
  [NODE_TYPES.SNAPSHOT_USAGE]:     usage(COLORS.structural, 'snapshot'),
  [NODE_TYPES.TIMESLICE_USAGE]:    usage(COLORS.structural, 'timeslice'),
  [NODE_TYPES.REFERENCE_USAGE]:    usageDir(COLORS.structural, 'ref'),

  // ==================== Interfaces (Purple) ====================
  // Ports, interfaces, connections, flows
  
  [NODE_TYPES.PORT_DEF]:           defDir(COLORS.interface, 'port'),
  [NODE_TYPES.PORT_USAGE]:         usageDir(COLORS.interface, 'port'),
  
  [NODE_TYPES.INTERFACE_DEF]:      def(COLORS.interface, 'interface'),
  [NODE_TYPES.INTERFACE_USAGE]:    usage(COLORS.interface, 'interface'),
  
  [NODE_TYPES.CONNECTION_DEF]:     def(COLORS.interface, 'connection'),
  [NODE_TYPES.CONNECTION_USAGE]:   usage(COLORS.interface, 'connection'),
  
  [NODE_TYPES.FLOW_DEF]:           defDir(COLORS.interface, 'flow'),
  [NODE_TYPES.FLOW_USAGE]:         usageDir(COLORS.interface, 'flow'),

  // ==================== Behavioral (Green) ====================
  // Actions, states, calculations
  
  [NODE_TYPES.ACTION_DEF]:         def(COLORS.behavioral, 'action'),
  [NODE_TYPES.ACTION_USAGE]:       usage(COLORS.behavioral, 'action'),
  [NODE_TYPES.PERFORM_ACTION_USAGE]: usage(COLORS.behavioral, 'perform'),
  
  [NODE_TYPES.STATE_DEF]:          def(COLORS.behavioral, 'state'),
  [NODE_TYPES.STATE_USAGE]:        usage(COLORS.behavioral, 'state'),
  [NODE_TYPES.EXHIBIT_STATE_USAGE]: usage(COLORS.behavioral, 'exhibit'),
  
  [NODE_TYPES.CALCULATION_DEF]:    def(COLORS.behavioral, 'calculation'),
  [NODE_TYPES.CALCULATION_USAGE]:  usage(COLORS.behavioral, 'calc'),

  // ==================== Requirements (Orange) ====================
  // Requirements, concerns, constraints
  
  [NODE_TYPES.REQUIREMENT_DEF]:    def(COLORS.requirement, 'requirement'),
  [NODE_TYPES.REQUIREMENT_USAGE]:  usage(COLORS.requirement, 'requirement'),
  [NODE_TYPES.SATISFY_REQUIREMENT_USAGE]: usage(COLORS.requirement, 'satisfy'),
  
  [NODE_TYPES.CONCERN_DEF]:        def(COLORS.requirement, 'concern'),
  [NODE_TYPES.CONCERN_USAGE]:      usage(COLORS.requirement, 'concern'),
  
  [NODE_TYPES.CONSTRAINT_DEF]:     def(COLORS.requirement, 'constraint'),
  [NODE_TYPES.CONSTRAINT_USAGE]:   usage(COLORS.requirement, 'constraint'),

  // ==================== Cases (Indigo) ====================
  // Use cases, analysis, verification
  
  [NODE_TYPES.CASE_DEF]:           def(COLORS.case, 'case'),
  [NODE_TYPES.CASE_USAGE]:         usage(COLORS.case, 'case'),
  
  [NODE_TYPES.USE_CASE_DEF]:       def(COLORS.case, 'use case'),
  [NODE_TYPES.INCLUDE_USE_CASE_USAGE]: usage(COLORS.case, 'include'),
  
  [NODE_TYPES.ANALYSIS_CASE_DEF]:  def(COLORS.case, 'analysis case'),
  [NODE_TYPES.VERIFICATION_CASE_DEF]: def(COLORS.case, 'verification case'),

  // ==================== Views (Teal) ====================
  // Views, viewpoints, rendering
  
  [NODE_TYPES.VIEW_DEF]:           def(COLORS.view, 'view'),
  [NODE_TYPES.VIEW_USAGE]:         usage(COLORS.view, 'view'),
  
  [NODE_TYPES.VIEWPOINT_DEF]:      def(COLORS.view, 'viewpoint'),
  [NODE_TYPES.RENDERING_DEF]:      def(COLORS.view, 'rendering'),

  // ==================== Other (Slate) ====================
  // Allocations, enumerations, metadata
  
  [NODE_TYPES.ALLOCATION_DEF]:     def(COLORS.other, 'allocation'),
  [NODE_TYPES.ALLOCATION_USAGE]:   usage(COLORS.other, 'allocate'),
  
  [NODE_TYPES.ENUMERATION_DEF]:    def(COLORS.other, 'enumeration'),
  [NODE_TYPES.ENUMERATION_USAGE]:  usage(COLORS.other, 'enum'),
  
  [NODE_TYPES.METADATA_DEF]:       def(COLORS.other, 'metadata'),
};
