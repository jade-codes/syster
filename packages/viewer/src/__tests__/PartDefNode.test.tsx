import { describe, test, expect, mock } from 'bun:test';
import { render } from '@testing-library/react';
import { screen } from '@testing-library/dom';
import { NODE_TYPES } from '@syster/diagram-core';

// Mock reactflow Handle before importing component
mock.module('reactflow', () => ({
  Handle: () => null,
  Position: { Top: 'top', Bottom: 'bottom', Left: 'left', Right: 'right' },
}));

import { PartDefNode } from '../nodes/PartDefNode';

describe('PartDefNode', () => {
  test('renders node with qualified name', () => {
    const mockData = {
      type: NODE_TYPES.PART_DEF,
      qualifiedName: 'Vehicle::Car',
      name: 'Car',
      kind: 'Definition' as const,
      features: [],
    };

    render(<PartDefNode id="test-1" data={mockData} />);
    
    const nameElement = screen.getByText('Car');
    expect(nameElement).not.toBeNull();
  });

  test('renders node with features list', () => {
    const mockData = {
      type: NODE_TYPES.PART_DEF,
      qualifiedName: 'Vehicle',
      name: 'Vehicle',
      kind: 'Definition' as const,
      features: ['speed', 'mass', 'color'],
    };

    render(<PartDefNode id="test-2" data={mockData} />);
    
    expect(screen.getByText('speed')).not.toBeNull();
    expect(screen.getByText('mass')).not.toBeNull();
    expect(screen.getByText('color')).not.toBeNull();
  });

  test('renders with empty features array', () => {
    const mockData = {
      type: NODE_TYPES.PART_DEF,
      qualifiedName: 'EmptyPart',
      name: 'EmptyPart',
      kind: 'Definition' as const,
      features: [],
    };

    render(<PartDefNode id="test-3" data={mockData} />);
    
    const nameElement = screen.getByText('EmptyPart');
    expect(nameElement).not.toBeNull();
  });

  test('renders with stereotype label', () => {
    const mockData = {
      type: NODE_TYPES.PART_DEF,
      qualifiedName: 'Vehicle',
      name: 'Vehicle',
      kind: 'Definition' as const,
      features: [],
    };

    const { container } = render(<PartDefNode id="test-4" data={mockData} />);
    
    const stereotype = container.querySelector('div[style*="italic"]');
    expect(stereotype).not.toBeNull();
    expect(stereotype?.textContent).toBe('«part def»');
  });
});
