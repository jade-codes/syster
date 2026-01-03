import { describe, test, expect, mock } from 'bun:test';
import { render } from '@testing-library/react';
import { screen } from '@testing-library/dom';
import { NODE_TYPES, FEATURE_DIRECTIONS } from '@syster/diagram-core';

// Mock reactflow Handle before importing component
mock.module('reactflow', () => ({
  Handle: () => null,
  Position: { Top: 'top', Bottom: 'bottom', Left: 'left', Right: 'right' },
}));

import { PortDefNode } from '../nodes/PortDefNode';

describe('PortDefNode', () => {
  test('renders node with name', () => {
    const mockData = {
      type: NODE_TYPES.PORT_DEF,
      qualifiedName: 'DataPort',
      name: 'DataPort',
      kind: 'Definition' as const,
      direction: FEATURE_DIRECTIONS.INOUT,
    };

    render(<PortDefNode id="test-1" data={mockData} />);
    
    const nameElement = screen.getByText('DataPort');
    expect(nameElement).not.toBeNull();
  });

  test('renders with in direction', () => {
    const mockData = {
      type: NODE_TYPES.PORT_DEF,
      qualifiedName: 'InputPort',
      name: 'InputPort',
      kind: 'Definition' as const,
      direction: FEATURE_DIRECTIONS.IN,
    };

    render(<PortDefNode id="test-2" data={mockData} />);
    
    expect(screen.getByText('InputPort')).not.toBeNull();
    expect(screen.getByText('in')).not.toBeNull();
  });

  test('renders with out direction', () => {
    const mockData = {
      type: NODE_TYPES.PORT_DEF,
      qualifiedName: 'OutputPort',
      name: 'OutputPort',
      kind: 'Definition' as const,
      direction: FEATURE_DIRECTIONS.OUT,
    };

    render(<PortDefNode id="test-3" data={mockData} />);
    
    expect(screen.getByText('OutputPort')).not.toBeNull();
    expect(screen.getByText('out')).not.toBeNull();
  });

  test('renders with inout direction', () => {
    const mockData = {
      type: NODE_TYPES.PORT_DEF,
      qualifiedName: 'BidirectionalPort',
      name: 'BidirectionalPort',
      kind: 'Definition' as const,
      direction: FEATURE_DIRECTIONS.INOUT,
    };

    const { container } = render(<PortDefNode id="test-4" data={mockData} />);
    
    expect(screen.getByText('BidirectionalPort')).not.toBeNull();
    const directionElement = container.querySelector('div[style*="#7c3aed"][style*="font-weight"]');
    expect(directionElement).not.toBeNull();
    expect(directionElement?.textContent).toBe('inout');
  });

  test('renders with stereotype label', () => {
    const mockData = {
      type: NODE_TYPES.PORT_DEF,
      qualifiedName: 'DataPort',
      name: 'DataPort',
      kind: 'Definition' as const,
      direction: FEATURE_DIRECTIONS.IN,
    };

    const { container } = render(<PortDefNode id="test-5" data={mockData} />);
    
    const stereotype = container.querySelector('div[style*="italic"]');
    expect(stereotype).not.toBeNull();
    expect(stereotype?.textContent).toBe('«port def»');
  });
});
