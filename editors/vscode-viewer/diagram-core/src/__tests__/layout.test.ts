import { describe, it, expect } from "bun:test";
import {
  LAYOUT_DIRECTIONS,
  LAYOUT_ALGORITHMS,
  createDefaultLayoutConfig,
  type LayoutConfig,
  type Position,
  type Dimensions,
  type PositionedNode,
  type PositionedEdge,
  type LayoutResult
} from "../layout";

describe("Layout Constants", () => {
  it("should define all layout directions", () => {
    expect(LAYOUT_DIRECTIONS.TOP_BOTTOM).toBe("TB");
    expect(LAYOUT_DIRECTIONS.BOTTOM_TOP).toBe("BT");
    expect(LAYOUT_DIRECTIONS.LEFT_RIGHT).toBe("LR");
    expect(LAYOUT_DIRECTIONS.RIGHT_LEFT).toBe("RL");
  });

  it("should define all layout algorithms", () => {
    expect(LAYOUT_ALGORITHMS.DAGRE).toBe("dagre");
    expect(LAYOUT_ALGORITHMS.ELK).toBe("elk");
  });
});

describe("Layout Configuration", () => {
  describe("LayoutConfig", () => {
    it("should create config with required fields", () => {
      const config: LayoutConfig = {
        algorithm: LAYOUT_ALGORITHMS.DAGRE,
        direction: LAYOUT_DIRECTIONS.TOP_BOTTOM
      };
      
      expect(config.algorithm).toBe("dagre");
      expect(config.direction).toBe("TB");
    });

    it("should support optional node spacing", () => {
      const config: LayoutConfig = {
        algorithm: LAYOUT_ALGORITHMS.DAGRE,
        direction: LAYOUT_DIRECTIONS.LEFT_RIGHT,
        nodeSpacing: 50
      };
      
      expect(config.nodeSpacing).toBe(50);
    });

    it("should support optional rank spacing", () => {
      const config: LayoutConfig = {
        algorithm: LAYOUT_ALGORITHMS.DAGRE,
        direction: LAYOUT_DIRECTIONS.TOP_BOTTOM,
        rankSpacing: 100
      };
      
      expect(config.rankSpacing).toBe(100);
    });

    it("should support optional edge spacing", () => {
      const config: LayoutConfig = {
        algorithm: LAYOUT_ALGORITHMS.ELK,
        direction: LAYOUT_DIRECTIONS.TOP_BOTTOM,
        edgeSpacing: 20
      };
      
      expect(config.edgeSpacing).toBe(20);
    });

    it("should support all spacing options together", () => {
      const config: LayoutConfig = {
        algorithm: LAYOUT_ALGORITHMS.DAGRE,
        direction: LAYOUT_DIRECTIONS.LEFT_RIGHT,
        nodeSpacing: 50,
        rankSpacing: 100,
        edgeSpacing: 20
      };
      
      expect(config.nodeSpacing).toBe(50);
      expect(config.rankSpacing).toBe(100);
      expect(config.edgeSpacing).toBe(20);
    });
  });
});

describe("Position and Dimensions", () => {
  describe("Position", () => {
    it("should define x and y coordinates", () => {
      const pos: Position = { x: 100, y: 200 };
      
      expect(pos.x).toBe(100);
      expect(pos.y).toBe(200);
    });

    it("should support negative coordinates", () => {
      const pos: Position = { x: -50, y: -100 };
      
      expect(pos.x).toBe(-50);
      expect(pos.y).toBe(-100);
    });
  });

  describe("Dimensions", () => {
    it("should define width and height", () => {
      const dims: Dimensions = { width: 200, height: 100 };
      
      expect(dims.width).toBe(200);
      expect(dims.height).toBe(100);
    });
  });
});

describe("Positioned Elements", () => {
  describe("PositionedNode", () => {
    it("should combine node with position", () => {
      const node: PositionedNode = {
        id: "node1",
        position: { x: 100, y: 200 },
        data: { type: "PartDef", name: "Vehicle" }
      };
      
      expect(node.id).toBe("node1");
      expect(node.position.x).toBe(100);
      expect(node.position.y).toBe(200);
    });

    it("should support optional dimensions", () => {
      const node: PositionedNode = {
        id: "node1",
        position: { x: 100, y: 200 },
        dimensions: { width: 150, height: 80 },
        data: { type: "PartDef" }
      };
      
      expect(node.dimensions?.width).toBe(150);
      expect(node.dimensions?.height).toBe(80);
    });
  });

  describe("PositionedEdge", () => {
    it("should define edge with source and target", () => {
      const edge: PositionedEdge = {
        id: "edge1",
        source: "node1",
        target: "node2",
        type: "specialization"
      };
      
      expect(edge.source).toBe("node1");
      expect(edge.target).toBe("node2");
    });

    it("should support optional label", () => {
      const edge: PositionedEdge = {
        id: "edge1",
        source: "node1",
        target: "node2",
        type: "specialization",
        label: "specializes"
      };
      
      expect(edge.label).toBe("specializes");
    });
  });
});

describe("Layout Result", () => {
  it("should contain positioned nodes and edges", () => {
    const result: LayoutResult = {
      nodes: [
        {
          id: "node1",
          position: { x: 0, y: 0 },
          data: { type: "PartDef" }
        }
      ],
      edges: [
        {
          id: "edge1",
          source: "node1",
          target: "node2",
          type: "specialization"
        }
      ]
    };
    
    expect(result.nodes).toHaveLength(1);
    expect(result.edges).toHaveLength(1);
  });

  it("should support optional graph dimensions", () => {
    const result: LayoutResult = {
      nodes: [],
      edges: [],
      dimensions: { width: 1000, height: 800 }
    };
    
    expect(result.dimensions?.width).toBe(1000);
    expect(result.dimensions?.height).toBe(800);
  });

  it("should handle empty results", () => {
    const result: LayoutResult = {
      nodes: [],
      edges: []
    };
    
    expect(result.nodes).toHaveLength(0);
    expect(result.edges).toHaveLength(0);
  });
});

describe("Default Layout Configuration", () => {
  it("should provide sensible defaults", () => {
    const config = createDefaultLayoutConfig();
    
    expect(config.algorithm).toBe(LAYOUT_ALGORITHMS.DAGRE);
    expect(config.direction).toBe(LAYOUT_DIRECTIONS.TOP_BOTTOM);
    expect(config.nodeSpacing).toBe(50);
    expect(config.rankSpacing).toBe(100);
  });

  it("should allow overriding defaults", () => {
    const config = createDefaultLayoutConfig({
      direction: LAYOUT_DIRECTIONS.LEFT_RIGHT,
      nodeSpacing: 80
    });
    
    expect(config.direction).toBe(LAYOUT_DIRECTIONS.LEFT_RIGHT);
    expect(config.nodeSpacing).toBe(80);
    expect(config.algorithm).toBe(LAYOUT_ALGORITHMS.DAGRE); // Still default
  });
});
