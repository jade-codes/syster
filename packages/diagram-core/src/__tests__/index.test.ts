import { describe, expect, it } from "bun:test";
import { createEmptyDiagram, type Diagram, type DiagramNode, type DiagramEdge } from "../index";

describe("diagram-core", () => {
  describe("createEmptyDiagram", () => {
    it("should create a diagram with empty nodes array", () => {
      const diagram = createEmptyDiagram();
      expect(diagram.nodes).toEqual([]);
    });

    it("should create a diagram with empty edges array", () => {
      const diagram = createEmptyDiagram();
      expect(diagram.edges).toEqual([]);
    });

    it("should return a valid Diagram type", () => {
      const diagram = createEmptyDiagram();
      expect(diagram).toHaveProperty("nodes");
      expect(diagram).toHaveProperty("edges");
    });
  });

  describe("DiagramNode", () => {
    it("should allow creating a node with required properties", () => {
      const node: DiagramNode = {
        id: "node-1",
        type: "default",
        position: { x: 0, y: 0 },
        data: {},
      };
      expect(node.id).toBe("node-1");
      expect(node.type).toBe("default");
      expect(node.position).toEqual({ x: 0, y: 0 });
    });

    it("should allow storing arbitrary data in node", () => {
      const node: DiagramNode = {
        id: "node-1",
        type: "partDef",
        position: { x: 100, y: 200 },
        data: { label: "Vehicle", kind: "part def" },
      };
      expect(node.data).toEqual({ label: "Vehicle", kind: "part def" });
    });
  });

  describe("DiagramEdge", () => {
    it("should allow creating an edge with required properties", () => {
      const edge: DiagramEdge = {
        id: "edge-1",
        source: "node-1",
        target: "node-2",
      };
      expect(edge.id).toBe("edge-1");
      expect(edge.source).toBe("node-1");
      expect(edge.target).toBe("node-2");
    });

    it("should allow optional edge type", () => {
      const edge: DiagramEdge = {
        id: "edge-1",
        source: "node-1",
        target: "node-2",
        type: "specialization",
      };
      expect(edge.type).toBe("specialization");
    });
  });
});
