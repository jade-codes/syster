import { describe, test, expect, mock, afterEach } from 'bun:test';
import { render, screen, cleanup } from '@testing-library/react';
import { NODE_TYPES } from '@syster/diagram-core';

// Mock @xyflow/react before importing components
mock.module('@xyflow/react', () => ({
  Handle: () => null,
  Position: { Top: 'top', Bottom: 'bottom', Left: 'left', Right: 'right' },
  ReactFlowProvider: ({ children }: { children: React.ReactNode }) => <>{children}</>,
}));

import { createSysMLNode, nodeTypes, getNodeConfig } from '../nodes/nodeFactory';
import { NODE_CONFIGS } from '../nodes/nodeConfig';

// Clean up after each test to prevent DOM pollution
afterEach(() => {
  cleanup();
});

// Minimal test data - only includes fields the components actually use
interface TestNodeData {
  name: string;
  features?: string[];
  direction?: string;
}

describe('createSysMLNode', () => {
  test('creates a component with the specified stereotype', () => {
    const TestNode = createSysMLNode({
      borderColor: '#ff0000',
      stereotype: 'test def',
      showFeatures: true,
    });

    const data: TestNodeData = {
      name: 'TestElement',
    };

    render(<TestNode id="test-1" data={data as any} />);

    expect(screen.getByText('«test def»')).toBeDefined();
    expect(screen.getByText('TestElement')).toBeDefined();
  });

  test('shows features when showFeatures is true', () => {
    const TestNode = createSysMLNode({
      borderColor: '#ff0000',
      stereotype: 'test def',
      showFeatures: true,
    });

    const data: TestNodeData = {
      name: 'TestElement',
      features: ['myFeature: Integer'],
    };

    render(<TestNode id="test-1" data={data as any} />);

    expect(screen.getByText('myFeature: Integer')).toBeDefined();
  });

  test('shows multiple features correctly', () => {
    const TestNode = createSysMLNode({
      borderColor: '#ff0000',
      stereotype: 'test def',
      showFeatures: true,
    });

    const data: TestNodeData = {
      name: 'TestElement',
      features: ['feature1: String', 'feature2: Integer', 'feature3: Boolean'],
    };

    render(<TestNode id="test-1" data={data as any} />);

    expect(screen.getByText('feature1: String')).toBeDefined();
    expect(screen.getByText('feature2: Integer')).toBeDefined();
    expect(screen.getByText('feature3: Boolean')).toBeDefined();
  });

  test('hides features when showFeatures is false', () => {
    const TestNode = createSysMLNode({
      borderColor: '#ff0000',
      stereotype: 'test def',
      showFeatures: false,
    });

    const data: TestNodeData = {
      name: 'TestElement',
      features: ['hiddenFeature: Integer'],
    };

    render(<TestNode id="test-1" data={data as any} />);

    expect(screen.queryByText('hiddenFeature: Integer')).toBeNull();
  });

  test('hides direction when showDirection is false', () => {
    const TestNode = createSysMLNode({
      borderColor: '#ff0000',
      stereotype: 'port def',
      showDirection: false,
    });

    const data: TestNodeData = {
      name: 'DataPort',
      direction: 'in',
    };

    render(<TestNode id="test-1" data={data as any} />);

    expect(screen.queryByText('in')).toBeNull();
  });

  test('shows direction when showDirection is true', () => {
    const TestNode = createSysMLNode({
      borderColor: '#ff0000',
      stereotype: 'port def',
      showDirection: true,
    });

    const data: TestNodeData = {
      name: 'DataPort',
      direction: 'in',
    };

    render(<TestNode id="test-1" data={data as any} />);

    expect(screen.getByText('in')).toBeDefined();
  });

  test('sets displayName based on stereotype', () => {
    const TestNode = createSysMLNode({
      borderColor: '#ff0000',
      stereotype: 'part def',
    });

    expect(TestNode.displayName).toBe('partdefNode');
  });
});

describe('nodeTypes', () => {
  test('contains all node types from NODE_CONFIGS', () => {
    const configKeys = Object.keys(NODE_CONFIGS);
    const nodeTypeKeys = Object.keys(nodeTypes);

    expect(nodeTypeKeys.length).toBe(configKeys.length);
    
    for (const key of configKeys) {
      expect(nodeTypes[key]).toBeDefined();
      expect(typeof nodeTypes[key]).toBe('function');
    }
  });

  test('includes all definition types', () => {
    const definitionTypes = [
      NODE_TYPES.PART_DEF,
      NODE_TYPES.PORT_DEF,
      NODE_TYPES.ACTION_DEF,
      NODE_TYPES.STATE_DEF,
      NODE_TYPES.REQUIREMENT_DEF,
      NODE_TYPES.ITEM_DEF,
      NODE_TYPES.ATTRIBUTE_DEF,
      NODE_TYPES.VIEW_DEF,
    ];

    for (const type of definitionTypes) {
      expect(nodeTypes[type]).toBeDefined();
    }
  });

  test('includes all usage types', () => {
    const usageTypes = [
      NODE_TYPES.PART_USAGE,
      NODE_TYPES.PORT_USAGE,
      NODE_TYPES.ACTION_USAGE,
      NODE_TYPES.REQUIREMENT_USAGE,
    ];

    for (const type of usageTypes) {
      expect(nodeTypes[type]).toBeDefined();
    }
  });

  test('renders a part def node correctly', () => {
    const PartDefNode = nodeTypes[NODE_TYPES.PART_DEF];
    
    const data: TestNodeData = {
      name: 'Vehicle',
      features: ['engine: Engine'],
    };

    render(<PartDefNode id="vehicle-1" data={data as any} />);

    expect(screen.getByText('«part def»')).toBeDefined();
    expect(screen.getByText('Vehicle')).toBeDefined();
    expect(screen.getByText('engine: Engine')).toBeDefined();
  });

  test('renders a port def node with direction', () => {
    const PortDefNode = nodeTypes[NODE_TYPES.PORT_DEF];
    
    const data: TestNodeData = {
      name: 'DataPort',
      direction: 'out',
    };

    render(<PortDefNode id="port-1" data={data as any} />);

    expect(screen.getByText('«port def»')).toBeDefined();
    expect(screen.getByText('DataPort')).toBeDefined();
    expect(screen.getByText('out')).toBeDefined();
  });
});

describe('getNodeConfig', () => {
  test('returns config for valid node type', () => {
    const config = getNodeConfig(NODE_TYPES.PART_DEF);

    expect(config).toBeDefined();
    expect(config?.stereotype).toBe('part def');
    expect(config?.borderColor).toBe('#2563eb');
    expect(config?.showFeatures).toBe(true);
  });

  test('returns config for port def with showDirection', () => {
    const config = getNodeConfig(NODE_TYPES.PORT_DEF);

    expect(config).toBeDefined();
    expect(config?.stereotype).toBe('port def');
    expect(config?.showDirection).toBe(true);
  });

  test('returns undefined for unknown node type', () => {
    const config = getNodeConfig('unknown-type');

    expect(config).toBeUndefined();
  });
});

describe('NODE_CONFIGS', () => {
  test('all configs have required properties', () => {
    const missingProps: string[] = [];

    for (const [type, config] of Object.entries(NODE_CONFIGS)) {
      if (!config.borderColor) {
        missingProps.push(`${type}: missing borderColor`);
      }
      if (!config.stereotype) {
        missingProps.push(`${type}: missing stereotype`);
      }
    }

    expect(missingProps).toEqual([]);
  });

  test('all border colors are valid hex colors', () => {
    const invalidColors: string[] = [];
    const hexColorRegex = /^#[0-9a-fA-F]{6}$/;

    for (const [type, config] of Object.entries(NODE_CONFIGS)) {
      if (!hexColorRegex.test(config.borderColor)) {
        invalidColors.push(`${type}: ${config.borderColor}`);
      }
    }

    expect(invalidColors).toEqual([]);
  });

  test('definition types have "def" in stereotype', () => {
    const defTypes = Object.entries(NODE_CONFIGS).filter(([type]) => type.endsWith('-def'));
    const invalidStereotypes: string[] = [];

    for (const [type, config] of defTypes) {
      if (!config.stereotype.includes('def')) {
        invalidStereotypes.push(`${type}: ${config.stereotype}`);
      }
    }

    expect(invalidStereotypes).toEqual([]);
  });

  test('usage types do not have "def" in stereotype', () => {
    const usageTypes = Object.entries(NODE_CONFIGS).filter(([type]) => type.endsWith('-usage'));
    const invalidStereotypes: string[] = [];

    for (const [type, config] of usageTypes) {
      if (config.stereotype.includes('def')) {
        invalidStereotypes.push(`${type}: ${config.stereotype}`);
      }
    }

    expect(invalidStereotypes).toEqual([]);
  });
});
