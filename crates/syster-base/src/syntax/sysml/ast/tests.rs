#![allow(clippy::unwrap_used)]
#![allow(clippy::panic)]

use super::*;
use crate::core::traits::{AstNode, Named};
use crate::parser::sysml::{Rule, SysMLParser};
use crate::syntax::sysml::visitor::{AstVisitor, Visitable};
use ::from_pest::FromPest;
use pest::Parser;

#[test]
fn test_package_from_pest() {
    let source = "package MyPackage;";
    let mut pairs = SysMLParser::parse(Rule::package_declaration, source).unwrap();

    let package = Package::from_pest(&mut pairs).unwrap();

    assert_eq!(package.name, Some("MyPackage".to_string()));
    assert_eq!(package.elements.len(), 0);
}

#[test]
fn test_part_definition_from_pest() {
    let source = "part def Vehicle;";
    let mut pairs = SysMLParser::parse(Rule::part_definition, source).unwrap();

    let definition = Definition::from_pest(&mut pairs).unwrap();

    assert_eq!(definition.kind, DefinitionKind::Part);
    assert_eq!(definition.name, Some("Vehicle".to_string()));
    assert_eq!(definition.body.len(), 0);
}

#[test]
fn test_action_definition_from_pest() {
    let source = "action def Drive;";
    let mut pairs = SysMLParser::parse(Rule::action_definition, source).unwrap();

    let definition = Definition::from_pest(&mut pairs).unwrap();

    assert_eq!(definition.kind, DefinitionKind::Action);
    assert_eq!(definition.name, Some("Drive".to_string()));
    assert_eq!(definition.body.len(), 0);
}

#[test]
fn test_requirement_definition_from_pest() {
    let source = "requirement def SafetyReq;";
    let mut pairs = SysMLParser::parse(Rule::requirement_definition, source).unwrap();

    let definition = Definition::from_pest(&mut pairs).unwrap();

    assert_eq!(definition.kind, DefinitionKind::Requirement);
    assert_eq!(definition.name, Some("SafetyReq".to_string()));
    assert_eq!(definition.body.len(), 0);
}

#[test]
fn test_port_definition_from_pest() {
    let source = "port def PowerPort;";
    let mut pairs = SysMLParser::parse(Rule::port_definition, source).unwrap();

    let definition = Definition::from_pest(&mut pairs).unwrap();

    assert_eq!(definition.kind, DefinitionKind::Port);
    assert_eq!(definition.name, Some("PowerPort".to_string()));
}

#[test]
fn test_item_definition_from_pest() {
    let source = "item def Fuel;";
    let mut pairs = SysMLParser::parse(Rule::item_definition, source).unwrap();

    let definition = Definition::from_pest(&mut pairs).unwrap();

    assert_eq!(definition.kind, DefinitionKind::Item);
    assert_eq!(definition.name, Some("Fuel".to_string()));
}

#[test]
fn test_attribute_definition_from_pest() {
    let source = "attribute def Mass;";
    let mut pairs = SysMLParser::parse(Rule::attribute_definition, source).unwrap();

    let definition = Definition::from_pest(&mut pairs).unwrap();

    assert_eq!(definition.kind, DefinitionKind::Attribute);
    assert_eq!(definition.name, Some("Mass".to_string()));
}

#[test]
fn test_part_usage_from_pest() {
    let source = "part engine;";
    let mut pairs = SysMLParser::parse(Rule::part_usage, source).unwrap();

    let usage = Usage::from_pest(&mut pairs).unwrap();

    assert_eq!(usage.kind, UsageKind::Part);
    assert_eq!(usage.name, Some("engine".to_string()));
}

#[test]
fn test_action_usage_from_pest() {
    let source = "action start;";
    let mut pairs = SysMLParser::parse(Rule::action_usage, source).unwrap();

    let usage = Usage::from_pest(&mut pairs).unwrap();

    assert_eq!(usage.kind, UsageKind::Action);
    assert_eq!(usage.name, Some("start".to_string()));
}

#[test]
fn test_port_usage_from_pest() {
    let source = "port power;";
    let mut pairs = SysMLParser::parse(Rule::port_usage, source).unwrap();

    let usage = Usage::from_pest(&mut pairs).unwrap();

    assert_eq!(usage.kind, UsageKind::Port);
    assert_eq!(usage.name, Some("power".to_string()));
}

#[test]
fn test_item_usage_from_pest() {
    let source = "item fuel;";
    let mut pairs = SysMLParser::parse(Rule::item_usage, source).unwrap();

    let usage = Usage::from_pest(&mut pairs).unwrap();

    assert_eq!(usage.kind, UsageKind::Item);
    assert_eq!(usage.name, Some("fuel".to_string()));
}

#[test]
fn test_attribute_usage_from_pest() {
    let source = "attribute weight;";
    let mut pairs = SysMLParser::parse(Rule::attribute_usage, source).unwrap();

    let usage = Usage::from_pest(&mut pairs).unwrap();

    assert_eq!(usage.kind, UsageKind::Attribute);
    assert_eq!(usage.name, Some("weight".to_string()));
}

#[test]
fn test_requirement_usage_from_pest() {
    let source = "requirement safetyReq;";
    let mut pairs = SysMLParser::parse(Rule::requirement_usage, source).unwrap();

    let usage = Usage::from_pest(&mut pairs).unwrap();

    assert_eq!(usage.kind, UsageKind::Requirement);
    assert_eq!(usage.name, Some("safetyReq".to_string()));
}

#[test]
fn test_concern_definition_from_pest() {
    let source = "concern def PerformanceConcern;";
    let mut pairs = SysMLParser::parse(Rule::concern_definition, source).unwrap();

    let definition = Definition::from_pest(&mut pairs).unwrap();

    assert_eq!(definition.kind, DefinitionKind::Concern);
    assert_eq!(definition.name, Some("PerformanceConcern".to_string()));
}

#[test]
fn test_case_definition_from_pest() {
    let source = "case def TestCase;";
    let mut pairs = SysMLParser::parse(Rule::case_definition, source).unwrap();

    let definition = Definition::from_pest(&mut pairs).unwrap();

    assert_eq!(definition.kind, DefinitionKind::Case);
    assert_eq!(definition.name, Some("TestCase".to_string()));
}

#[test]
fn test_analysis_case_definition_from_pest() {
    let source = "analysis case def StressAnalysis;";
    let mut pairs = SysMLParser::parse(Rule::analysis_case_definition, source).unwrap();

    let definition = Definition::from_pest(&mut pairs).unwrap();

    assert_eq!(definition.kind, DefinitionKind::AnalysisCase);
    assert_eq!(definition.name, Some("StressAnalysis".to_string()));
}

#[test]
fn test_verification_case_definition_from_pest() {
    let source = "verification case def SafetyVerification;";
    let mut pairs = SysMLParser::parse(Rule::verification_case_definition, source).unwrap();

    let definition = Definition::from_pest(&mut pairs).unwrap();

    assert_eq!(definition.kind, DefinitionKind::VerificationCase);
    assert_eq!(definition.name, Some("SafetyVerification".to_string()));
}

#[test]
fn test_use_case_definition_from_pest() {
    let source = "use case def UserLogin;";
    let mut pairs = SysMLParser::parse(Rule::use_case_definition, source).unwrap();

    let definition = Definition::from_pest(&mut pairs).unwrap();

    assert_eq!(definition.kind, DefinitionKind::UseCase);
    assert_eq!(definition.name, Some("UserLogin".to_string()));
}

#[test]
fn test_view_definition_from_pest() {
    let source = "view def SystemView;";
    let mut pairs = SysMLParser::parse(Rule::view_definition, source).unwrap();

    let definition = Definition::from_pest(&mut pairs).unwrap();

    assert_eq!(definition.kind, DefinitionKind::View);
    assert_eq!(definition.name, Some("SystemView".to_string()));
}

#[test]
fn test_viewpoint_definition_from_pest() {
    let source = "viewpoint def ArchitecturalViewpoint;";
    let mut pairs = SysMLParser::parse(Rule::viewpoint_definition, source).unwrap();

    let definition = Definition::from_pest(&mut pairs).unwrap();

    assert_eq!(definition.kind, DefinitionKind::Viewpoint);
    assert_eq!(definition.name, Some("ArchitecturalViewpoint".to_string()));
}

#[test]
fn test_rendering_definition_from_pest() {
    let source = "rendering def DiagramRendering;";
    let mut pairs = SysMLParser::parse(Rule::rendering_definition, source).unwrap();

    let definition = Definition::from_pest(&mut pairs).unwrap();

    assert_eq!(definition.kind, DefinitionKind::Rendering);
    assert_eq!(definition.name, Some("DiagramRendering".to_string()));
}

#[test]
fn test_concern_usage_from_pest() {
    let source = "concern perfConcern;";
    let mut pairs = SysMLParser::parse(Rule::concern_usage, source).unwrap();

    let usage = Usage::from_pest(&mut pairs).unwrap();

    assert_eq!(usage.kind, UsageKind::Concern);
    assert_eq!(usage.name, Some("perfConcern".to_string()));
}

#[test]
fn test_case_usage_from_pest() {
    let source = "case testCase;";
    let mut pairs = SysMLParser::parse(Rule::case_usage, source).unwrap();

    let usage = Usage::from_pest(&mut pairs).unwrap();

    assert_eq!(usage.kind, UsageKind::Case);
    assert_eq!(usage.name, Some("testCase".to_string()));
}

#[test]
fn test_view_usage_from_pest() {
    let source = "view systemView;";
    let mut pairs = SysMLParser::parse(Rule::view_usage, source).unwrap();

    let usage = Usage::from_pest(&mut pairs).unwrap();

    assert_eq!(usage.kind, UsageKind::View);
    assert_eq!(usage.name, Some("systemView".to_string()));
}

#[test]
fn test_comment_annotation_from_pest() {
    let source = "comment c1;";
    let mut pairs = SysMLParser::parse(Rule::comment_annotation, source).unwrap();

    let comment = Comment::from_pest(&mut pairs).unwrap();

    assert!(!comment.content.is_empty());
}

#[test]
fn test_import_from_pest() {
    let source = "import SomePackage::*;";
    let mut pairs = SysMLParser::parse(Rule::import, source).unwrap();

    // Import::from_pest expects the children of the import rule
    let import_pair = pairs.next().unwrap();
    let import = Import::from_pest(&mut import_pair.into_inner()).unwrap();

    assert!(!import.path.is_empty());
}

impl AstNode for Package {
    fn node_type(&self) -> &'static str {
        "Package"
    }

    fn has_children(&self) -> bool {
        !self.elements.is_empty()
    }
}

impl Named for Package {
    fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}

impl AstNode for Definition {
    fn node_type(&self) -> &'static str {
        "Definition"
    }
}

impl Named for Definition {
    fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}

#[test]
fn test_ast_node_trait() {
    let pkg = Package {
        name: Some("TestPackage".to_string()),
        elements: vec![],
        span: None,
    };

    assert_eq!(pkg.node_type(), "Package");
    assert_eq!(pkg.name(), Some("TestPackage"));
    assert!(!pkg.has_children());
}

#[test]
fn test_definition_traits() {
    let def = Definition {
        kind: DefinitionKind::Part,
        name: Some("Vehicle".to_string()),
        body: vec![],
        relationships: crate::syntax::sysml::ast::Relationships::none(),
        is_abstract: false,
        is_variation: false,
        span: None,
    };

    assert_eq!(def.node_type(), "Definition");
    assert_eq!(def.name(), Some("Vehicle"));
}

struct CountingVisitor {
    packages: usize,
    definitions: usize,
}

impl AstVisitor for CountingVisitor {
    fn visit_package(&mut self, _package: &Package) {
        self.packages += 1;
    }

    fn visit_definition(&mut self, _definition: &Definition) {
        self.definitions += 1;
    }
}

#[test]
fn test_visitor_pattern() {
    let file = SysMLFile {
        namespace: None,
        elements: vec![
            Element::Package(Package {
                name: Some("TestPkg".to_string()),
                elements: vec![],
                span: None,
            }),
            Element::Definition(Definition {
                kind: DefinitionKind::Part,
                name: Some("TestDef".to_string()),
                body: vec![],
                relationships: crate::syntax::sysml::ast::Relationships::none(),
                is_abstract: false,
                is_variation: false,
                span: None,
            }),
        ],
    };

    let mut visitor = CountingVisitor {
        packages: 0,
        definitions: 0,
    };

    file.accept(&mut visitor);

    assert_eq!(visitor.packages, 1);
    assert_eq!(visitor.definitions, 1);
}

#[test]
fn test_definition_with_specialization() {
    let source = "part def Car :> Vehicle;";
    let mut pairs = SysMLParser::parse(Rule::part_definition, source).unwrap();

    let definition = Definition::from_pest(&mut pairs).unwrap();

    assert_eq!(definition.kind, DefinitionKind::Part);
    assert_eq!(definition.name, Some("Car".to_string()));
    assert_eq!(definition.relationships.specializes.len(), 1);
    assert_eq!(definition.relationships.specializes[0], "Vehicle");
}

#[test]
fn test_definition_with_multiple_specializations() {
    let source = "part def MultipleCar :> Vehicle, Machine;";
    let mut pairs = SysMLParser::parse(Rule::part_definition, source).unwrap();

    let definition = Definition::from_pest(&mut pairs).unwrap();

    assert_eq!(definition.kind, DefinitionKind::Part);
    assert_eq!(definition.name, Some("MultipleCar".to_string()));
    assert_eq!(definition.relationships.specializes.len(), 2);
    assert!(
        definition
            .relationships
            .specializes
            .contains(&"Vehicle".to_string())
    );
    assert!(
        definition
            .relationships
            .specializes
            .contains(&"Machine".to_string())
    );
}

#[test]
fn test_usage_with_typing() {
    let source = "part myCar : Car;";
    let mut pairs = SysMLParser::parse(Rule::part_usage, source).unwrap();

    let usage = Usage::from_pest(&mut pairs).unwrap();

    assert_eq!(usage.kind, UsageKind::Part);
    assert_eq!(usage.name, Some("myCar".to_string()));
    assert_eq!(usage.relationships.typed_by, Some("Car".to_string()));
}

#[test]
fn test_usage_with_subsetting() {
    let source = "part specialCar : Car :> baseCar;";
    let mut pairs = SysMLParser::parse(Rule::part_usage, source).unwrap();

    let usage = Usage::from_pest(&mut pairs).unwrap();

    assert_eq!(usage.kind, UsageKind::Part);
    assert_eq!(usage.name, Some("specialCar".to_string()));
    assert_eq!(usage.relationships.typed_by, Some("Car".to_string()));
    assert_eq!(usage.relationships.subsets.len(), 1);
    assert_eq!(usage.relationships.subsets[0], "baseCar");
}

#[test]
fn test_usage_with_redefinition() {
    let source = "part redefinedCar : Car :>> originalCar;";
    let mut pairs = SysMLParser::parse(Rule::part_usage, source).unwrap();

    let usage = Usage::from_pest(&mut pairs).unwrap();

    assert_eq!(usage.kind, UsageKind::Part);
    assert_eq!(usage.name, Some("redefinedCar".to_string()));
    assert_eq!(usage.relationships.typed_by, Some("Car".to_string()));
    assert_eq!(usage.relationships.redefines.len(), 1);
    assert_eq!(usage.relationships.redefines[0], "originalCar");
}

#[test]
fn test_usage_with_multiple_subsettings() {
    let source = "part multiCar : Car :> car1, car2, car3;";
    let mut pairs = SysMLParser::parse(Rule::part_usage, source).unwrap();

    let usage = Usage::from_pest(&mut pairs).unwrap();

    assert_eq!(usage.kind, UsageKind::Part);
    assert_eq!(usage.relationships.subsets.len(), 3);
    assert!(usage.relationships.subsets.contains(&"car1".to_string()));
    assert!(usage.relationships.subsets.contains(&"car2".to_string()));
    assert!(usage.relationships.subsets.contains(&"car3".to_string()));
}

#[test]
fn test_anonymous_definition() {
    let source = "part def;";
    let mut pairs = SysMLParser::parse(Rule::part_definition, source).unwrap();

    let definition = Definition::from_pest(&mut pairs).unwrap();

    assert_eq!(definition.kind, DefinitionKind::Part);
    assert_eq!(definition.name, None);
}

#[test]
fn test_usage_with_name_and_typing() {
    // Test a usage with both an explicit name and a type
    let source = "part vehicle : Vehicle;";
    let mut pairs = SysMLParser::parse(Rule::part_usage, source).unwrap();

    let usage = Usage::from_pest(&mut pairs).unwrap();

    assert_eq!(usage.kind, UsageKind::Part);
    assert_eq!(usage.name, Some("vehicle".to_string()));
    assert_eq!(usage.relationships.typed_by, Some("Vehicle".to_string()));
}

#[test]
fn test_action_usage_with_relationships() {
    let source = "action myDrive : Drive :> baseAction;";
    let mut pairs = SysMLParser::parse(Rule::action_usage, source).unwrap();

    let usage = Usage::from_pest(&mut pairs).unwrap();

    assert_eq!(usage.kind, UsageKind::Action);
    assert_eq!(usage.name, Some("myDrive".to_string()));
    assert_eq!(usage.relationships.typed_by, Some("Drive".to_string()));
    assert_eq!(usage.relationships.subsets.len(), 1);
    assert_eq!(usage.relationships.subsets[0], "baseAction");
}

#[test]
fn test_requirement_with_specialization() {
    let source = "requirement def SafetyReq :> BaseReq;";
    let mut pairs = SysMLParser::parse(Rule::requirement_definition, source).unwrap();

    let definition = Definition::from_pest(&mut pairs).unwrap();

    assert_eq!(definition.kind, DefinitionKind::Requirement);
    assert_eq!(definition.name, Some("SafetyReq".to_string()));
    assert_eq!(definition.relationships.specializes.len(), 1);
    assert_eq!(definition.relationships.specializes[0], "BaseReq");
}

#[test]
fn test_all_definition_kinds() {
    let test_cases = vec![
        (
            "part def Test;",
            DefinitionKind::Part,
            Rule::part_definition,
        ),
        (
            "action def Test;",
            DefinitionKind::Action,
            Rule::action_definition,
        ),
        (
            "requirement def Test;",
            DefinitionKind::Requirement,
            Rule::requirement_definition,
        ),
        (
            "port def Test;",
            DefinitionKind::Port,
            Rule::port_definition,
        ),
        (
            "item def Test;",
            DefinitionKind::Item,
            Rule::item_definition,
        ),
        (
            "attribute def Test;",
            DefinitionKind::Attribute,
            Rule::attribute_definition,
        ),
        (
            "concern def Test;",
            DefinitionKind::Concern,
            Rule::concern_definition,
        ),
        (
            "case def Test;",
            DefinitionKind::Case,
            Rule::case_definition,
        ),
        (
            "analysis case def Test;",
            DefinitionKind::AnalysisCase,
            Rule::analysis_case_definition,
        ),
        (
            "verification case def Test;",
            DefinitionKind::VerificationCase,
            Rule::verification_case_definition,
        ),
        (
            "use case def Test;",
            DefinitionKind::UseCase,
            Rule::use_case_definition,
        ),
        (
            "view def Test;",
            DefinitionKind::View,
            Rule::view_definition,
        ),
        (
            "viewpoint def Test;",
            DefinitionKind::Viewpoint,
            Rule::viewpoint_definition,
        ),
        (
            "rendering def Test;",
            DefinitionKind::Rendering,
            Rule::rendering_definition,
        ),
    ];

    for (source, expected_kind, rule) in test_cases {
        let mut pairs = SysMLParser::parse(rule, source).unwrap();
        let definition = Definition::from_pest(&mut pairs).unwrap();
        assert_eq!(definition.kind, expected_kind, "Failed for: {source}");
        assert_eq!(definition.name, Some("Test".to_string()));
    }
}

#[test]
fn test_all_usage_kinds() {
    let test_cases = vec![
        ("part test;", UsageKind::Part, Rule::part_usage),
        ("action test;", UsageKind::Action, Rule::action_usage),
        (
            "requirement test;",
            UsageKind::Requirement,
            Rule::requirement_usage,
        ),
        ("port test;", UsageKind::Port, Rule::port_usage),
        ("item test;", UsageKind::Item, Rule::item_usage),
        (
            "attribute test;",
            UsageKind::Attribute,
            Rule::attribute_usage,
        ),
        ("concern test;", UsageKind::Concern, Rule::concern_usage),
        ("case test;", UsageKind::Case, Rule::case_usage),
        ("view test;", UsageKind::View, Rule::view_usage),
    ];

    for (source, expected_kind, rule) in test_cases {
        let mut pairs = SysMLParser::parse(rule, source).unwrap();
        let usage = Usage::from_pest(&mut pairs).unwrap();
        assert_eq!(usage.kind, expected_kind, "Failed for: {source}");
        assert_eq!(usage.name, Some("test".to_string()));
    }
}

#[test]
fn test_relationships_none() {
    let relationships = crate::syntax::sysml::ast::Relationships::none();

    assert_eq!(relationships.specializes.len(), 0);
    assert_eq!(relationships.typed_by, None);
    assert_eq!(relationships.subsets.len(), 0);
    assert_eq!(relationships.redefines.len(), 0);
    assert_eq!(relationships.references.len(), 0);
}

#[test]
fn test_element_is_package() {
    let element = Element::Package(Package {
        name: Some("Test".to_string()),
        elements: vec![],
        span: None,
    });

    match element {
        Element::Package(_) => {}
        _ => panic!("Expected Package variant"),
    }
}

#[test]
fn test_element_is_definition() {
    let element = Element::Definition(Definition {
        kind: DefinitionKind::Part,
        name: Some("Test".to_string()),
        body: vec![],
        relationships: crate::syntax::sysml::ast::Relationships::none(),
        is_abstract: false,
        is_variation: false,
        span: None,
    });

    match element {
        Element::Definition(def) => {
            assert_eq!(def.kind, DefinitionKind::Part);
            assert_eq!(def.name, Some("Test".to_string()));
        }
        _ => panic!("Expected Definition variant"),
    }
}

#[test]
fn test_element_is_usage() {
    let element = Element::Usage(Usage {
        kind: UsageKind::Part,
        name: Some("test".to_string()),
        body: vec![],
        relationships: crate::syntax::sysml::ast::Relationships::none(),
        is_derived: false,
        is_readonly: false,
        span: None,
    });

    match element {
        Element::Usage(usage) => {
            assert_eq!(usage.kind, UsageKind::Part);
            assert_eq!(usage.name, Some("test".to_string()));
        }
        _ => panic!("Expected Usage variant"),
    }
}

#[test]
fn test_complex_usage_all_relationships() {
    let source = "part complexPart : PartType :> base1, base2 :>> redefined1;";
    let mut pairs = SysMLParser::parse(Rule::part_usage, source).unwrap();

    let usage = Usage::from_pest(&mut pairs).unwrap();

    assert_eq!(usage.name, Some("complexPart".to_string()));
    assert_eq!(usage.relationships.typed_by, Some("PartType".to_string()));
    assert_eq!(usage.relationships.subsets.len(), 2);
    assert!(usage.relationships.subsets.contains(&"base1".to_string()));
    assert!(usage.relationships.subsets.contains(&"base2".to_string()));
    assert_eq!(usage.relationships.redefines.len(), 1);
    assert_eq!(usage.relationships.redefines[0], "redefined1");
}

#[test]
fn test_named_trait_for_definition() {
    let definition = Definition {
        kind: DefinitionKind::Part,
        name: Some("TestDef".to_string()),
        body: vec![],
        relationships: crate::syntax::sysml::ast::Relationships::none(),
        is_abstract: false,
        is_variation: false,
        span: None,
    };

    assert_eq!(definition.name(), Some("TestDef"));
}

#[test]
fn test_named_trait_for_usage() {
    let usage = Usage {
        kind: UsageKind::Part,
        name: Some("testUsage".to_string()),
        body: vec![],
        relationships: crate::syntax::sysml::ast::Relationships::none(),
        is_derived: false,
        is_readonly: false,
        span: None,
    };

    assert_eq!(usage.name.as_deref(), Some("testUsage"));
}

#[test]
fn test_named_trait_for_package() {
    let package = Package {
        name: Some("TestPackage".to_string()),
        elements: vec![],
        span: None,
    };

    assert_eq!(package.name(), Some("TestPackage"));
}

#[test]
fn test_named_trait_none() {
    let definition = Definition {
        kind: DefinitionKind::Part,
        name: None,
        body: vec![],
        relationships: crate::syntax::sysml::ast::Relationships::none(),
        is_abstract: false,
        is_variation: false,
        span: None,
    };

    assert_eq!(definition.name(), None);
}

#[test]
fn test_kind_constants_are_valid_strings() {
    assert_eq!(SYSML_KIND_PART, "Part");
    assert_eq!(SYSML_KIND_PORT, "Port");
    assert_eq!(SYSML_KIND_ITEM, "Item");
    assert_eq!(SYSML_KIND_ATTRIBUTE, "Attribute");
    assert_eq!(SYSML_KIND_ACTION, "Action");
    assert_eq!(SYSML_KIND_STATE, "State");
    assert_eq!(SYSML_KIND_REQUIREMENT, "Requirement");
    assert_eq!(SYSML_KIND_CONCERN, "UseCase");
    assert_eq!(SYSML_KIND_CASE, "UseCase");
    assert_eq!(SYSML_KIND_ANALYSIS_CASE, "UseCase");
    assert_eq!(SYSML_KIND_VERIFICATION_CASE, "UseCase");
    assert_eq!(SYSML_KIND_USE_CASE, "UseCase");
    assert_eq!(SYSML_KIND_VIEW, "View");
    assert_eq!(SYSML_KIND_VIEWPOINT, "Viewpoint");
    assert_eq!(SYSML_KIND_RENDERING, "Rendering");
}

#[test]
fn test_primary_kinds_unique() {
    let primary_kinds = vec![
        SYSML_KIND_PART,
        SYSML_KIND_PORT,
        SYSML_KIND_ITEM,
        SYSML_KIND_ATTRIBUTE,
        SYSML_KIND_ACTION,
        SYSML_KIND_STATE,
        SYSML_KIND_REQUIREMENT,
        SYSML_KIND_VIEW,
        SYSML_KIND_VIEWPOINT,
        SYSML_KIND_RENDERING,
        "UseCase",
    ];

    let mut unique = primary_kinds.clone();
    unique.sort();
    unique.dedup();
    assert_eq!(
        unique.len(),
        primary_kinds.len(),
        "Primary kind constants should be unique"
    );
}

#[test]
fn test_case_kinds_all_map_to_use_case() {
    assert_eq!(SYSML_KIND_CONCERN, "UseCase");
    assert_eq!(SYSML_KIND_CASE, "UseCase");
    assert_eq!(SYSML_KIND_ANALYSIS_CASE, "UseCase");
    assert_eq!(SYSML_KIND_VERIFICATION_CASE, "UseCase");
    assert_eq!(SYSML_KIND_USE_CASE, "UseCase");
}
