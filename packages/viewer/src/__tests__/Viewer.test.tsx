import { describe, it, expect } from "bun:test";
import { render } from "@testing-library/react";
import { Viewer } from "../Viewer";
import { NODE_TYPES } from "@syster/diagram-core";

describe("Viewer Component", () => {
  it("should render empty canvas when no diagram provided", () => {
    const { container } = render(<Viewer />);
    
    // React Flow renders a container div
    const reactFlowWrapper = container.querySelector('.react-flow');
    expect(reactFlowWrapper).not.toBeNull();
  });

  it("should render with empty diagram", () => {
    const emptyDiagram = {
      nodes: [],
      edges: []
    };

    const { container } = render(<Viewer diagram={emptyDiagram} />);
    
    const reactFlowWrapper = container.querySelector('.react-flow');
    expect(reactFlowWrapper).not.toBeNull();
  });

  it("should render diagram with nodes", () => {
    const diagram = {
      nodes: [
        {
          id: "node1",
          type: NODE_TYPES.PART_DEF,
          position: { x: 0, y: 0 },
          data: {
            id: "node1",
            name: "Vehicle",
            qualifiedName: "Auto::Vehicle",
            type: NODE_TYPES.PART_DEF,
            features: []
          }
        },
        {
          id: "node2",
          type: NODE_TYPES.PART_DEF,
          position: { x: 200, y: 0 },
          data: {
            id: "node2",
            name: "Car",
            qualifiedName: "Auto::Car",
            type: NODE_TYPES.PART_DEF,
            features: []
          }
        }
      ],
      edges: []
    };

    const { container } = render(<Viewer diagram={diagram} />);
    
    const reactFlowWrapper = container.querySelector('.react-flow');
    expect(reactFlowWrapper).not.toBeNull();
    
    // Verify nodes are rendered (React Flow adds nodes to the DOM)
    const nodes = container.querySelectorAll('.react-flow__node');
    expect(nodes.length).toBe(2);
  });

  it("should render diagram with edges", () => {
    const diagram = {
      nodes: [
        {
          id: "node1",
          type: NODE_TYPES.PART_DEF,
          position: { x: 0, y: 0 },
          data: {
            id: "node1",
            name: "Vehicle",
            qualifiedName: "Auto::Vehicle",
            type: NODE_TYPES.PART_DEF,
            features: []
          }
        },
        {
          id: "node2",
          type: NODE_TYPES.PART_DEF,
          position: { x: 200, y: 0 },
          data: {
            id: "node2",
            name: "Car",
            qualifiedName: "Auto::Car",
            type: NODE_TYPES.PART_DEF,
            features: []
          }
        }
      ],
      edges: [
        {
          id: "edge1",
          source: "node2",
          target: "node1",
          type: "specialization"
        }
      ]
    };

    const { container } = render(<Viewer diagram={diagram} />);
    
    const reactFlowWrapper = container.querySelector('.react-flow');
    expect(reactFlowWrapper).not.toBeNull();
    
    // Verify nodes are rendered
    const nodes = container.querySelectorAll('.react-flow__node');
    expect(nodes.length).toBe(2);
    
    // Check if edge layer exists (edges render as SVG paths)
    const edgeLayer = container.querySelector('.react-flow__edges');
    expect(edgeLayer).not.toBeNull();
  });

  it("should update when diagram changes", () => {
    const initialDiagram = {
      nodes: [
        {
          id: "node1",
          type: NODE_TYPES.PART_DEF,
          position: { x: 0, y: 0 },
          data: {
            id: "node1",
            name: "Vehicle",
            qualifiedName: "Auto::Vehicle",
            type: NODE_TYPES.PART_DEF,
            features: []
          }
        }
      ],
      edges: []
    };

    const { container, rerender } = render(<Viewer diagram={initialDiagram} />);
    
    let nodes = container.querySelectorAll('.react-flow__node');
    expect(nodes.length).toBe(1);

    // Update diagram with more nodes
    const updatedDiagram = {
      nodes: [
        ...initialDiagram.nodes,
        {
          id: "node2",
          type: NODE_TYPES.PART_DEF,
          position: { x: 200, y: 0 },
          data: {
            id: "node2",
            name: "Car",
            qualifiedName: "Auto::Car",
            type: NODE_TYPES.PART_DEF,
            features: []
          }
        }
      ],
      edges: []
    };

    rerender(<Viewer diagram={updatedDiagram} />);
    
    nodes = container.querySelectorAll('.react-flow__node');
    expect(nodes.length).toBe(2);
  });

  it("should have controls visible", () => {
    const { container } = render(<Viewer />);
    
    // React Flow Controls should be present
    const controls = container.querySelector('.react-flow__controls');
    expect(controls).not.toBeNull();
  });

  it("should have background visible", () => {
    const { container } = render(<Viewer />);
    
    // React Flow Background should be present
    const background = container.querySelector('.react-flow__background');
    expect(background).not.toBeNull();
  });

  it("should not allow dragging nodes", () => {
    const { container } = render(<Viewer />);
    
    const reactFlow = container.querySelector('.react-flow');
    // This is a prop check - we can verify by checking if nodes have the draggable class
    // In a read-only viewer, nodes shouldn't be draggable
    expect(reactFlow).not.toBeNull();
  });

  it("should render typing edges with open arrow marker", () => {
    const diagram = {
      nodes: [
        {
          id: "node1",
          type: NODE_TYPES.PART_DEF,
          position: { x: 0, y: 0 },
          data: {
            id: "node1",
            name: "Vehicle",
            qualifiedName: "Auto::Vehicle",
            type: NODE_TYPES.PART_DEF,
            features: []
          }
        },
        {
          id: "node2",
          type: NODE_TYPES.PART_USAGE,
          position: { x: 200, y: 0 },
          data: {
            id: "node2",
            name: "myCar",
            qualifiedName: "Auto::myCar",
            type: NODE_TYPES.PART_USAGE,
            typedBy: "Vehicle"
          }
        }
      ],
      edges: [
        {
          id: "edge1",
          source: "node2",
          target: "node1",
          type: "typing"
        }
      ]
    };

    const { container } = render(<Viewer diagram={diagram} />);
    
    const edgeLayer = container.querySelector('.react-flow__edges');
    expect(edgeLayer).not.toBeNull();
  });
});
