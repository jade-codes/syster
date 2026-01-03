import { describe, it, expect } from "bun:test";
import {
  EDGE_TYPES,
  type SpecializationEdge,
  type RedefinitionEdge,
  type SubsettingEdge,
  type TypingEdge,
  type ReferenceSubsettingEdge,
  type CrossSubsettingEdge,
  type SatisfyEdge,
  type PerformEdge,
  type ExhibitEdge,
  type IncludeEdge,
  type AssertEdge,
  type VerifyEdge,
  type CompositionEdge
} from "../sysml-edges";

describe("SysML Core Relationship Edges", () => {
  describe("SpecializationEdge", () => {
    it("should have correct type discriminator", () => {
      const edge: SpecializationEdge = {
        id: "edge1",
        type: EDGE_TYPES.SPECIALIZATION,
        source: "Car",
        target: "Vehicle",
        label: "specializes"
      };
      
      expect(edge.type).toBe(EDGE_TYPES.SPECIALIZATION);
      expect(edge.label).toBe("specializes");
    });

    it("should allow optional label", () => {
      const edge: SpecializationEdge = {
        id: "edge1",
        type: EDGE_TYPES.SPECIALIZATION,
        source: "Car",
        target: "Vehicle"
      };
      
      expect(edge.label).toBeUndefined();
    });
  });

  describe("RedefinitionEdge", () => {
    it("should have correct type discriminator", () => {
      const edge: RedefinitionEdge = {
        id: "edge1",
        type: EDGE_TYPES.REDEFINITION,
        source: "ElectricVehicle::engine",
        target: "Vehicle::engine",
        label: "redefines"
      };
      
      expect(edge.type).toBe(EDGE_TYPES.REDEFINITION);
    });
  });

  describe("SubsettingEdge", () => {
    it("should have correct type discriminator", () => {
      const edge: SubsettingEdge = {
        id: "edge1",
        type: EDGE_TYPES.SUBSETTING,
        source: "Car::wheels",
        target: "Vehicle::components",
        label: "subsets"
      };
      
      expect(edge.type).toBe(EDGE_TYPES.SUBSETTING);
    });
  });

  describe("TypingEdge", () => {
    it("should have correct type discriminator", () => {
      const edge: TypingEdge = {
        id: "edge1",
        type: EDGE_TYPES.TYPING,
        source: "myCar",
        target: "Car",
        label: "typed by"
      };
      
      expect(edge.type).toBe(EDGE_TYPES.TYPING);
    });
  });

  describe("ReferenceSubsettingEdge", () => {
    it("should have correct type discriminator", () => {
      const edge: ReferenceSubsettingEdge = {
        id: "edge1",
        type: EDGE_TYPES.REFERENCE_SUBSETTING,
        source: "ref1",
        target: "ref2",
        label: "reference subsets"
      };
      
      expect(edge.type).toBe(EDGE_TYPES.REFERENCE_SUBSETTING);
    });
  });

  describe("CrossSubsettingEdge", () => {
    it("should have correct type discriminator", () => {
      const edge: CrossSubsettingEdge = {
        id: "edge1",
        type: EDGE_TYPES.CROSS_SUBSETTING,
        source: "ref1",
        target: "ref2",
        label: "cross subsets"
      };
      
      expect(edge.type).toBe(EDGE_TYPES.CROSS_SUBSETTING);
    });
  });
});

describe("SysML Domain-Specific Relationship Edges", () => {
  describe("SatisfyEdge", () => {
    it("should have correct type discriminator", () => {
      const edge: SatisfyEdge = {
        id: "edge1",
        type: EDGE_TYPES.SATISFY,
        source: "BrakingSystem",
        target: "SafetyRequirement",
        label: "satisfies"
      };
      
      expect(edge.type).toBe(EDGE_TYPES.SATISFY);
    });
  });

  describe("PerformEdge", () => {
    it("should have correct type discriminator", () => {
      const edge: PerformEdge = {
        id: "edge1",
        type: EDGE_TYPES.PERFORM,
        source: "System",
        target: "Accelerate",
        label: "performs"
      };
      
      expect(edge.type).toBe(EDGE_TYPES.PERFORM);
    });
  });

  describe("ExhibitEdge", () => {
    it("should have correct type discriminator", () => {
      const edge: ExhibitEdge = {
        id: "edge1",
        type: EDGE_TYPES.EXHIBIT,
        source: "Engine",
        target: "Running",
        label: "exhibits"
      };
      
      expect(edge.type).toBe(EDGE_TYPES.EXHIBIT);
    });
  });

  describe("IncludeEdge", () => {
    it("should have correct type discriminator", () => {
      const edge: IncludeEdge = {
        id: "edge1",
        type: EDGE_TYPES.INCLUDE,
        source: "MainScenario",
        target: "DriveVehicle",
        label: "includes"
      };
      
      expect(edge.type).toBe(EDGE_TYPES.INCLUDE);
    });
  });

  describe("AssertEdge", () => {
    it("should have correct type discriminator", () => {
      const edge: AssertEdge = {
        id: "edge1",
        type: EDGE_TYPES.ASSERT,
        source: "VerificationCase",
        target: "Constraint",
        label: "asserts"
      };
      
      expect(edge.type).toBe(EDGE_TYPES.ASSERT);
    });
  });

  describe("VerifyEdge", () => {
    it("should have correct type discriminator", () => {
      const edge: VerifyEdge = {
        id: "edge1",
        type: EDGE_TYPES.VERIFY,
        source: "TestCase",
        target: "Requirement",
        label: "verifies"
      };
      
      expect(edge.type).toBe(EDGE_TYPES.VERIFY);
    });
  });
});

describe("SysML Structural Edges", () => {
  describe("CompositionEdge", () => {
    it("should have correct type discriminator for part containment", () => {
      const edge: CompositionEdge = {
        id: "edge1",
        type: EDGE_TYPES.COMPOSITION,
        source: "Car",
        target: "Engine",
        label: "contains"
      };
      
      expect(edge.type).toBe(EDGE_TYPES.COMPOSITION);
    });

    it("should support multiplicity information", () => {
      const edge: CompositionEdge = {
        id: "edge1",
        type: EDGE_TYPES.COMPOSITION,
        source: "Car",
        target: "Wheel",
        multiplicity: "4"
      };
      
      expect(edge.multiplicity).toBe("4");
    });
  });
});

describe("Edge Type Discrimination", () => {
  it("should distinguish between different edge types", () => {
    const specialization: SpecializationEdge = {
      id: "e1",
      type: EDGE_TYPES.SPECIALIZATION,
      source: "Car",
      target: "Vehicle"
    };

    const typing: TypingEdge = {
      id: "e2",
      type: EDGE_TYPES.TYPING,
      source: "myCar",
      target: "Car"
    };

    const composition: CompositionEdge = {
      id: "e3",
      type: EDGE_TYPES.COMPOSITION,
      source: "Car",
      target: "Engine"
    };

    expect(specialization.type).not.toBe(typing.type);
    expect(typing.type).not.toBe(composition.type);
    expect(specialization.type).toBe(EDGE_TYPES.SPECIALIZATION);
    expect(typing.type).toBe(EDGE_TYPES.TYPING);
    expect(composition.type).toBe(EDGE_TYPES.COMPOSITION);
  });
});

describe("Type Guard Functions", () => {
  const specializationEdge: SpecializationEdge = {
    id: "e1",
    type: EDGE_TYPES.SPECIALIZATION,
    source: "Car",
    target: "Vehicle"
  };

  const satisfyEdge: SatisfyEdge = {
    id: "e2",
    type: EDGE_TYPES.SATISFY,
    source: "System",
    target: "Requirement"
  };

  const compositionEdge: CompositionEdge = {
    id: "e3",
    type: EDGE_TYPES.COMPOSITION,
    source: "Car",
    target: "Engine"
  };

  it("should correctly identify relationship edges", () => {
    const { isRelationshipEdge } = require("../sysml-edges");
    
    expect(isRelationshipEdge(specializationEdge)).toBe(true);
    expect(isRelationshipEdge(satisfyEdge)).toBe(true);
    expect(isRelationshipEdge(compositionEdge)).toBe(false);
  });

  it("should correctly identify structural edges", () => {
    const { isStructuralEdge } = require("../sysml-edges");
    
    expect(isStructuralEdge(compositionEdge)).toBe(true);
    expect(isStructuralEdge(specializationEdge)).toBe(false);
  });
});
