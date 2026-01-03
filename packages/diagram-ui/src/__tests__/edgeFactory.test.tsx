import { describe, test, expect, mock, afterEach } from 'bun:test';
import { render, cleanup } from '@testing-library/react';
import { EDGE_TYPES } from '@syster/diagram-core';
import { MarkerType, Position } from '@xyflow/react';

// Mock @xyflow/react components
mock.module('@xyflow/react', () => ({
  BaseEdge: ({ id, path, style }: { id: string; path: string; style?: object }) => (
    <path data-testid={`edge-${id}`} d={path} style={style} />
  ),
  EdgeLabelRenderer: ({ children }: { children: React.ReactNode }) => (
    <div data-testid="edge-label-renderer">{children}</div>
  ),
  getSmoothStepPath: () => ['M0,0 L100,100', 50, 50],
  MarkerType: { Arrow: 'arrow', ArrowClosed: 'arrowclosed' },
  Position: { Top: 'top', Bottom: 'bottom', Left: 'left', Right: 'right' },
}));

import { EDGE_CONFIGS, getEdgeConfig, createSysMLEdge, edgeTypes } from '../edges';
import type { EdgeConfig } from '../edges';

// Cleanup after each test to prevent DOM pollution
afterEach(() => {
  cleanup();
});

describe('EDGE_CONFIGS', () => {
  test('has configuration for all EDGE_TYPES', () => {
    const edgeTypeValues = Object.values(EDGE_TYPES);

    for (const edgeType of edgeTypeValues) {
      expect(EDGE_CONFIGS[edgeType]).toBeDefined();
    }
  });

  test('specialization edge has correct config', () => {
    const config = EDGE_CONFIGS[EDGE_TYPES.SPECIALIZATION];

    expect(config.strokeColor).toBe('#475569');
    expect(config.strokeWidth).toBe(2);
    expect(config.markerEnd).toBe(MarkerType.ArrowClosed);
    expect(config.label).toBe('specializes');
  });

  test('composition edge has correct config', () => {
    const config = EDGE_CONFIGS[EDGE_TYPES.COMPOSITION];

    expect(config.strokeColor).toBe('#2563eb');
    expect(config.strokeWidth).toBe(2);
    expect(config.markerEnd).toBe(MarkerType.ArrowClosed);
  });

  test('typing edge is dashed', () => {
    const config = EDGE_CONFIGS[EDGE_TYPES.TYPING];

    expect(config.strokeDasharray).toBe('5 5');
    expect(config.markerEnd).toBe(MarkerType.Arrow);
  });

  test('satisfy edge has stereotype label', () => {
    const config = EDGE_CONFIGS[EDGE_TYPES.SATISFY];

    expect(config.label).toBe('«satisfy»');
    expect(config.strokeColor).toBe('#d97706');
  });

  test('perform edge has behavioral color', () => {
    const config = EDGE_CONFIGS[EDGE_TYPES.PERFORM];

    expect(config.strokeColor).toBe('#059669');
    expect(config.label).toBe('«perform»');
  });

  test('subsetting and cross-subsetting have different labels', () => {
    const subsettingConfig = EDGE_CONFIGS[EDGE_TYPES.SUBSETTING];
    const crossSubsettingConfig = EDGE_CONFIGS[EDGE_TYPES.CROSS_SUBSETTING];

    expect(subsettingConfig.label).toBe('subsets');
    expect(crossSubsettingConfig.label).toBe('cross-subsets');
    expect(subsettingConfig.label).not.toBe(crossSubsettingConfig.label);
  });
});

describe('getEdgeConfig', () => {
  test('returns config for known edge type', () => {
    const config = getEdgeConfig(EDGE_TYPES.SPECIALIZATION);

    expect(config.strokeColor).toBe('#475569');
    expect(config.label).toBe('specializes');
  });

  test('returns default config for unknown edge type', () => {
    const config = getEdgeConfig('unknown_edge_type');

    expect(config.strokeColor).toBe('#475569');
    expect(config.strokeWidth).toBe(2);
    expect(config.markerEnd).toBe(MarkerType.ArrowClosed);
  });
});

describe('createSysMLEdge', () => {
  test('creates edge component from config', () => {
    const config: EdgeConfig = {
      strokeColor: '#ff0000',
      strokeWidth: 3,
      markerEnd: MarkerType.Arrow,
      label: 'test',
    };

    const EdgeComponent = createSysMLEdge(config);
    expect(EdgeComponent).toBeDefined();
    expect(typeof EdgeComponent).toBe('function');
  });

  test('created edge has correct displayName without edgeType', () => {
    const config: EdgeConfig = {
      strokeColor: '#ff0000',
      strokeWidth: 3,
      markerEnd: MarkerType.Arrow,
      label: 'test',
    };

    const EdgeComponent = createSysMLEdge(config);
    expect(EdgeComponent.displayName).toBe('SysMLEdge');
  });

  test('created edge has unique displayName with edgeType', () => {
    const config: EdgeConfig = {
      strokeColor: '#ff0000',
      strokeWidth: 3,
      markerEnd: MarkerType.Arrow,
      label: 'test',
    };

    const EdgeComponent = createSysMLEdge(config, 'SPECIALIZATION');
    expect(EdgeComponent.displayName).toBe('SpecializationEdge');
  });

  test('created edge has unique displayName for multi-word edge types', () => {
    const config: EdgeConfig = {
      strokeColor: '#ff0000',
      strokeWidth: 3,
      markerEnd: MarkerType.Arrow,
      label: 'test',
    };

    const EdgeComponent = createSysMLEdge(config, 'CROSS_SUBSETTING');
    expect(EdgeComponent.displayName).toBe('CrossSubsettingEdge');
  });

  test('created edge renders with correct styles', () => {
    const config: EdgeConfig = {
      strokeColor: '#ff0000',
      strokeWidth: 3,
      markerEnd: MarkerType.Arrow,
      label: 'test-label',
    };

    const EdgeComponent = createSysMLEdge(config);

    const { container, getByText } = render(
      <EdgeComponent
        id="test-edge"
        source="node1"
        target="node2"
        sourceX={0}
        sourceY={0}
        targetX={100}
        targetY={100}
        sourcePosition={Position.Bottom}
        targetPosition={Position.Top}
      />
    );

    // Edge path should be rendered
    const path = container.querySelector('[data-testid="edge-test-edge"]');
    expect(path).not.toBeNull();

    // Label should be rendered
    expect(getByText('test-label')).not.toBeNull();
  });

  test('created edge shows data label over config label', () => {
    const config: EdgeConfig = {
      strokeColor: '#ff0000',
      strokeWidth: 3,
      markerEnd: MarkerType.Arrow,
      label: 'config-label',
    };

    const EdgeComponent = createSysMLEdge(config);

    const { getByText, queryByText } = render(
      <EdgeComponent
        id="test-edge"
        source="node1"
        target="node2"
        sourceX={0}
        sourceY={0}
        targetX={100}
        targetY={100}
        sourcePosition={Position.Bottom}
        targetPosition={Position.Top}
        data={{ label: 'data-label' }}
      />
    );

    // Data label should be used instead of config label
    expect(getByText('data-label')).not.toBeNull();
    expect(queryByText('config-label')).toBeNull();
  });

  test('created edge shows multiplicity for composition', () => {
    const config: EdgeConfig = {
      strokeColor: '#2563eb',
      strokeWidth: 2,
      markerEnd: MarkerType.ArrowClosed,
    };

    const EdgeComponent = createSysMLEdge(config);

    const { getByText } = render(
      <EdgeComponent
        id="test-edge"
        source="node1"
        target="node2"
        sourceX={0}
        sourceY={0}
        targetX={100}
        targetY={100}
        sourcePosition={Position.Bottom}
        targetPosition={Position.Top}
        data={{ multiplicity: '4' }}
      />
    );

    expect(getByText('[4]')).not.toBeNull();
  });
});

describe('edgeTypes', () => {
  test('has entry for each EDGE_TYPE', () => {
    const edgeTypeValues = Object.values(EDGE_TYPES);

    for (const edgeType of edgeTypeValues) {
      expect(edgeTypes[edgeType]).toBeDefined();
      expect(typeof edgeTypes[edgeType]).toBe('function');
    }
  });

  test('specialization edge renders correctly', () => {
    const SpecializationEdge = edgeTypes[EDGE_TYPES.SPECIALIZATION];

    const { container, getByText } = render(
      <SpecializationEdge
        id="spec-edge"
        source="child"
        target="parent"
        sourceX={0}
        sourceY={0}
        targetX={100}
        targetY={100}
        sourcePosition={Position.Bottom}
        targetPosition={Position.Top}
      />
    );

    const path = container.querySelector('[data-testid="edge-spec-edge"]');
    expect(path).not.toBeNull();
    expect(getByText('specializes')).not.toBeNull();
  });

  test('composition edge renders correctly', () => {
    const CompositionEdge = edgeTypes[EDGE_TYPES.COMPOSITION];

    const { container } = render(
      <CompositionEdge
        id="comp-edge"
        source="container"
        target="contained"
        sourceX={0}
        sourceY={0}
        targetX={100}
        targetY={100}
        sourcePosition={Position.Bottom}
        targetPosition={Position.Top}
        data={{ multiplicity: '*' }}
      />
    );

    const path = container.querySelector('[data-testid="edge-comp-edge"]');
    expect(path).not.toBeNull();
  });

  test('typing edge renders with dashed style', () => {
    const TypingEdge = edgeTypes[EDGE_TYPES.TYPING];

    const { container, getByText } = render(
      <TypingEdge
        id="typing-edge"
        source="usage"
        target="def"
        sourceX={0}
        sourceY={0}
        targetX={100}
        targetY={100}
        sourcePosition={Position.Bottom}
        targetPosition={Position.Top}
      />
    );

    const path = container.querySelector('[data-testid="edge-typing-edge"]');
    expect(path).not.toBeNull();
    expect(getByText(':')).not.toBeNull();
  });

  test('satisfy edge renders with stereotype label', () => {
    const SatisfyEdge = edgeTypes[EDGE_TYPES.SATISFY];

    const { getByText } = render(
      <SatisfyEdge
        id="satisfy-edge"
        source="implementation"
        target="requirement"
        sourceX={0}
        sourceY={0}
        targetX={100}
        targetY={100}
        sourcePosition={Position.Bottom}
        targetPosition={Position.Top}
      />
    );

    expect(getByText('«satisfy»')).not.toBeNull();
  });

  test('selected edge has highlight color', () => {
    const SpecEdge = edgeTypes[EDGE_TYPES.SPECIALIZATION];

    const { container } = render(
      <SpecEdge
        id="selected-edge"
        source="child"
        target="parent"
        sourceX={0}
        sourceY={0}
        targetX={100}
        targetY={100}
        sourcePosition={Position.Bottom}
        targetPosition={Position.Top}
        selected={true}
      />
    );

    const path = container.querySelector('[data-testid="edge-selected-edge"]');
    expect(path).not.toBeNull();
  });

  test('edge without label or multiplicity renders path only', () => {
    const config: EdgeConfig = {
      strokeColor: '#000000',
      strokeWidth: 1,
      markerEnd: MarkerType.Arrow,
    };

    const EdgeComponent = createSysMLEdge(config);

    const { container, queryByTestId } = render(
      <EdgeComponent
        id="no-label-edge"
        source="a"
        target="b"
        sourceX={0}
        sourceY={0}
        targetX={100}
        targetY={100}
        sourcePosition={Position.Bottom}
        targetPosition={Position.Top}
      />
    );

    const path = container.querySelector('[data-testid="edge-no-label-edge"]');
    expect(path).not.toBeNull();
    // No label renderer should be present
    expect(queryByTestId('edge-label-renderer')).toBeNull();
  });
});
