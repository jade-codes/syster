#![allow(clippy::unwrap_used)]
#![allow(clippy::panic)]

use super::*;
use crate::language::sysml::syntax::{DefinitionKind, Element, Package, SysMLFile, UsageKind};

#[test]
fn test_populate_empty_file() {
    let mut table = SymbolTable::new();
    let mut populator = SymbolTablePopulator::new(&mut table);

    let file = SysMLFile {
        namespace: None,
        elements: vec![],
    };

    let result = populator.populate(&file);
    assert!(result.is_ok());
}

#[test]
fn test_populate_single_package() {
    let mut table = SymbolTable::new();
    let mut populator = SymbolTablePopulator::new(&mut table);

    let file = SysMLFile {
        namespace: None,
        elements: vec![Element::Package(Package {
            name: Some("TestPackage".to_string()),
            elements: vec![],
        })],
    };

    let result = populator.populate(&file);
    assert!(result.is_ok());

    let symbol = table.lookup("TestPackage");
    assert!(symbol.is_some());

    let Some(Symbol::Package {
        name,
        qualified_name,
        ..
    }) = symbol
    else {
        panic!("Expected Package symbol, got: {symbol:?}");
    };
    assert_eq!(name, "TestPackage");
    assert_eq!(qualified_name, "TestPackage");
}

#[test]
fn test_populate_nested_packages() {
    let mut table = SymbolTable::new();
    let mut populator = SymbolTablePopulator::new(&mut table);

    let file = SysMLFile {
        namespace: None,
        elements: vec![Element::Package(Package {
            name: Some("Outer".to_string()),
            elements: vec![Element::Package(Package {
                name: Some("Inner".to_string()),
                elements: vec![],
            })],
        })],
    };

    let result = populator.populate(&file);
    assert!(result.is_ok());

    let outer = table.lookup("Outer");
    assert!(outer.is_some());

    // Verify Inner package exists in the symbol table with correct qualified name
    let all_symbols = table.all_symbols();
    let inner = all_symbols
        .iter()
        .find(|(name, _)| *name == "Inner")
        .map(|(_, symbol)| *symbol);
    assert!(inner.is_some());

    let Some(Symbol::Package { qualified_name, .. }) = inner else {
        panic!("Expected Package symbol");
    };
    assert_eq!(qualified_name, "Outer::Inner");
}

#[test]
fn test_populate_definition() {
    let mut table = SymbolTable::new();
    let mut populator = SymbolTablePopulator::new(&mut table);

    let file = SysMLFile {
        namespace: None,
        elements: vec![Element::Definition(
            crate::language::sysml::syntax::Definition {
                kind: DefinitionKind::Part,
                name: Some("MyPart".to_string()),
                body: vec![],
                relationships: crate::language::sysml::syntax::Relationships::none(),
            },
        )],
    };

    let result = populator.populate(&file);
    assert!(result.is_ok());

    let symbol = table.lookup("MyPart");
    assert!(symbol.is_some());

    let Some(Symbol::Definition { name, kind, .. }) = symbol else {
        panic!("Expected Definition symbol, got: {symbol:?}");
    };
    assert_eq!(name, "MyPart");
    assert_eq!(kind, "Part");
}

#[test]
fn test_populate_usage() {
    let mut table = SymbolTable::new();
    let mut populator = SymbolTablePopulator::new(&mut table);

    let file = SysMLFile {
        namespace: None,
        elements: vec![Element::Usage(crate::language::sysml::syntax::Usage {
            kind: UsageKind::Action,
            name: Some("myAction".to_string()),
            body: vec![],
            relationships: crate::language::sysml::syntax::Relationships::none(),
        })],
    };

    let result = populator.populate(&file);
    assert!(result.is_ok());

    let symbol = table.lookup("myAction");
    assert!(symbol.is_some());

    let Some(Symbol::Usage { name, kind, .. }) = symbol else {
        panic!("Expected Usage symbol, got: {symbol:?}");
    };
    assert_eq!(name, "myAction");
    assert_eq!(kind, "Action");
}

#[test]
fn test_populate_duplicate_definition_error() {
    let mut table = SymbolTable::new();
    let mut populator = SymbolTablePopulator::new(&mut table);

    let file = SysMLFile {
        namespace: None,
        elements: vec![
            Element::Package(Package {
                name: Some("Duplicate".to_string()),
                elements: vec![],
            }),
            Element::Package(Package {
                name: Some("Duplicate".to_string()),
                elements: vec![],
            }),
        ],
    };

    let result = populator.populate(&file);
    assert!(result.is_err());

    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 1);
    assert!(errors[0].message.contains("Duplicate"));
}

#[test]
fn test_populate_definition_in_package() {
    let mut table = SymbolTable::new();
    let mut populator = SymbolTablePopulator::new(&mut table);

    let file = SysMLFile {
        namespace: None,
        elements: vec![Element::Package(Package {
            name: Some("MyPackage".to_string()),
            elements: vec![Element::Definition(
                crate::language::sysml::syntax::Definition {
                    kind: DefinitionKind::Part,
                    name: Some("NestedPart".to_string()),
                    body: vec![],
                    relationships: crate::language::sysml::syntax::Relationships::none(),
                },
            )],
        })],
    };

    let result = populator.populate(&file);
    assert!(result.is_ok());

    // Verify the definition was created with correct qualified name
    let all_symbols = table.all_symbols();
    let symbol = all_symbols
        .iter()
        .find(|(name, _)| *name == "NestedPart")
        .map(|(_, symbol)| *symbol);
    assert!(symbol.is_some());

    let Some(Symbol::Definition { qualified_name, .. }) = symbol else {
        panic!("Expected Definition symbol");
    };
    assert_eq!(qualified_name, "MyPackage::NestedPart");
}

#[test]
fn test_populate_anonymous_package() {
    let mut table = SymbolTable::new();
    let mut populator = SymbolTablePopulator::new(&mut table);

    let file = SysMLFile {
        namespace: None,
        elements: vec![Element::Package(Package {
            name: None,
            elements: vec![],
        })],
    };

    let result = populator.populate(&file);
    assert!(result.is_ok());
}

#[test]
fn test_populate_anonymous_definition() {
    let mut table = SymbolTable::new();
    let mut populator = SymbolTablePopulator::new(&mut table);

    let file = SysMLFile {
        namespace: None,
        elements: vec![Element::Definition(
            crate::language::sysml::syntax::Definition {
                kind: DefinitionKind::Part,
                name: None,
                body: vec![],
                relationships: crate::language::sysml::syntax::Relationships::none(),
            },
        )],
    };

    let result = populator.populate(&file);
    assert!(result.is_ok());
}

#[test]
fn test_populate_multiple_definitions() {
    let mut table = SymbolTable::new();
    let mut populator = SymbolTablePopulator::new(&mut table);

    let file = SysMLFile {
        namespace: None,
        elements: vec![
            Element::Definition(crate::language::sysml::syntax::Definition {
                kind: DefinitionKind::Part,
                name: Some("Part1".to_string()),
                body: vec![],
                relationships: crate::language::sysml::syntax::Relationships::none(),
            }),
            Element::Definition(crate::language::sysml::syntax::Definition {
                kind: DefinitionKind::Port,
                name: Some("Port1".to_string()),
                body: vec![],
                relationships: crate::language::sysml::syntax::Relationships::none(),
            }),
            Element::Usage(crate::language::sysml::syntax::Usage {
                kind: UsageKind::Action,
                name: Some("Action1".to_string()),
                body: vec![],
                relationships: crate::language::sysml::syntax::Relationships::none(),
            }),
        ],
    };

    let result = populator.populate(&file);
    assert!(result.is_ok());

    assert!(table.lookup("Part1").is_some());
    assert!(table.lookup("Port1").is_some());
    assert!(table.lookup("Action1").is_some());
}

#[test]
fn test_populate_deeply_nested_structure() {
    let mut table = SymbolTable::new();
    let mut populator = SymbolTablePopulator::new(&mut table);

    let file = SysMLFile {
        namespace: None,
        elements: vec![Element::Package(Package {
            name: Some("L1".to_string()),
            elements: vec![Element::Package(Package {
                name: Some("L2".to_string()),
                elements: vec![Element::Package(Package {
                    name: Some("L3".to_string()),
                    elements: vec![Element::Definition(
                        crate::language::sysml::syntax::Definition {
                            kind: DefinitionKind::Part,
                            name: Some("DeepPart".to_string()),
                            body: vec![],
                            relationships: crate::language::sysml::syntax::Relationships::none(),
                        },
                    )],
                })],
            })],
        })],
    };

    let result = populator.populate(&file);
    assert!(result.is_ok());

    // Verify deep nesting was created correctly
    let all_symbols = table.all_symbols();
    let symbol = all_symbols
        .iter()
        .find(|(name, _)| *name == "DeepPart")
        .map(|(_, symbol)| *symbol);
    assert!(symbol.is_some());

    let Some(Symbol::Definition { qualified_name, .. }) = symbol else {
        panic!("Expected Definition symbol");
    };
    assert_eq!(qualified_name, "L1::L2::L3::DeepPart");
}

#[test]
fn test_populate_all_definition_kinds() {
    let mut table = SymbolTable::new();
    let mut populator = SymbolTablePopulator::new(&mut table);

    let kinds = vec![
        (DefinitionKind::Part, "Part"),
        (DefinitionKind::Port, "Port"),
        (DefinitionKind::Action, "Action"),
        (DefinitionKind::Item, "Item"),
        (DefinitionKind::Attribute, "Attribute"),
        (DefinitionKind::Requirement, "Requirement"),
    ];

    let mut elements = vec![];
    for (kind, name) in kinds {
        elements.push(Element::Definition(
            crate::language::sysml::syntax::Definition {
                kind,
                name: Some(name.to_string()),
                body: vec![],
                relationships: crate::language::sysml::syntax::Relationships::none(),
            },
        ));
    }

    let file = SysMLFile {
        namespace: None,
        elements,
    };

    let result = populator.populate(&file);
    assert!(result.is_ok());

    assert!(table.lookup("Part").is_some());
    assert!(table.lookup("Port").is_some());
    assert!(table.lookup("Action").is_some());
    assert!(table.lookup("Item").is_some());
    assert!(table.lookup("Attribute").is_some());
    assert!(table.lookup("Requirement").is_some());
}

#[test]
fn test_populate_mixed_elements_in_package() {
    let mut table = SymbolTable::new();
    let mut populator = SymbolTablePopulator::new(&mut table);

    let file = SysMLFile {
        namespace: None,
        elements: vec![Element::Package(Package {
            name: Some("MixedPackage".to_string()),
            elements: vec![
                Element::Definition(crate::language::sysml::syntax::Definition {
                    kind: DefinitionKind::Part,
                    name: Some("PartDef".to_string()),
                    body: vec![],
                    relationships: crate::language::sysml::syntax::Relationships::none(),
                }),
                Element::Usage(crate::language::sysml::syntax::Usage {
                    kind: UsageKind::Part,
                    name: Some("partUsage".to_string()),
                    body: vec![],
                    relationships: crate::language::sysml::syntax::Relationships::none(),
                }),
                Element::Package(Package {
                    name: Some("NestedPkg".to_string()),
                    elements: vec![],
                }),
            ],
        })],
    };

    let result = populator.populate(&file);
    assert!(result.is_ok());

    // Verify all symbols created with correct qualified names
    let all_symbols = table.all_symbols();

    let part_def = all_symbols
        .iter()
        .find(|(name, _)| *name == "PartDef")
        .map(|(_, symbol)| *symbol);
    let Some(Symbol::Definition { qualified_name, .. }) = part_def else {
        panic!("Expected Definition symbol for PartDef");
    };
    assert_eq!(qualified_name, "MixedPackage::PartDef");

    let part_usage = all_symbols
        .iter()
        .find(|(name, _)| *name == "partUsage")
        .map(|(_, symbol)| *symbol);
    let Some(Symbol::Usage { qualified_name, .. }) = part_usage else {
        panic!("Expected Usage symbol for partUsage");
    };
    assert_eq!(qualified_name, "MixedPackage::partUsage");

    let nested_pkg = all_symbols
        .iter()
        .find(|(name, _)| *name == "NestedPkg")
        .map(|(_, symbol)| *symbol);
    let Some(Symbol::Package { qualified_name, .. }) = nested_pkg else {
        panic!("Expected Package symbol for NestedPkg");
    };
    assert_eq!(qualified_name, "MixedPackage::NestedPkg");
}

#[test]
fn test_populate_sibling_packages() {
    let mut table = SymbolTable::new();
    let mut populator = SymbolTablePopulator::new(&mut table);

    let file = SysMLFile {
        namespace: None,
        elements: vec![
            Element::Package(Package {
                name: Some("Package1".to_string()),
                elements: vec![Element::Definition(
                    crate::language::sysml::syntax::Definition {
                        kind: DefinitionKind::Part,
                        name: Some("Part1".to_string()),
                        body: vec![],
                        relationships: crate::language::sysml::syntax::Relationships::none(),
                    },
                )],
            }),
            Element::Package(Package {
                name: Some("Package2".to_string()),
                elements: vec![Element::Definition(
                    crate::language::sysml::syntax::Definition {
                        kind: DefinitionKind::Part,
                        name: Some("Part2".to_string()),
                        body: vec![],
                        relationships: crate::language::sysml::syntax::Relationships::none(),
                    },
                )],
            }),
        ],
    };

    let result = populator.populate(&file);
    assert!(result.is_ok());

    let all_symbols = table.all_symbols();

    let part1 = all_symbols
        .iter()
        .find(|(name, _)| *name == "Part1")
        .map(|(_, symbol)| *symbol);
    let Some(Symbol::Definition { qualified_name, .. }) = part1 else {
        panic!("Expected Definition symbol for Part1");
    };
    assert_eq!(qualified_name, "Package1::Part1");

    let part2 = all_symbols
        .iter()
        .find(|(name, _)| *name == "Part2")
        .map(|(_, symbol)| *symbol);
    let Some(Symbol::Definition { qualified_name, .. }) = part2 else {
        panic!("Expected Definition symbol for Part2");
    };
    assert_eq!(qualified_name, "Package2::Part2");
}

#[test]
fn test_populate_duplicate_in_nested_scope() {
    let mut table = SymbolTable::new();
    let mut populator = SymbolTablePopulator::new(&mut table);

    let file = SysMLFile {
        namespace: None,
        elements: vec![Element::Package(Package {
            name: Some("Outer".to_string()),
            elements: vec![
                Element::Definition(crate::language::sysml::syntax::Definition {
                    kind: DefinitionKind::Part,
                    name: Some("Duplicate".to_string()),
                    body: vec![],
                    relationships: crate::language::sysml::syntax::Relationships::none(),
                }),
                Element::Definition(crate::language::sysml::syntax::Definition {
                    kind: DefinitionKind::Port,
                    name: Some("Duplicate".to_string()),
                    body: vec![],
                    relationships: crate::language::sysml::syntax::Relationships::none(),
                }),
            ],
        })],
    };

    let result = populator.populate(&file);
    assert!(result.is_err());

    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 1);
    assert!(errors[0].message.contains("Duplicate"));
}

#[test]
fn test_populate_same_name_different_scopes() {
    let mut table = SymbolTable::new();
    let mut populator = SymbolTablePopulator::new(&mut table);

    let file = SysMLFile {
        namespace: None,
        elements: vec![
            Element::Package(Package {
                name: Some("Package1".to_string()),
                elements: vec![Element::Definition(
                    crate::language::sysml::syntax::Definition {
                        kind: DefinitionKind::Part,
                        name: Some("Common".to_string()),
                        body: vec![],
                        relationships: crate::language::sysml::syntax::Relationships::none(),
                    },
                )],
            }),
            Element::Package(Package {
                name: Some("Package2".to_string()),
                elements: vec![Element::Definition(
                    crate::language::sysml::syntax::Definition {
                        kind: DefinitionKind::Part,
                        name: Some("Common".to_string()),
                        body: vec![],
                        relationships: crate::language::sysml::syntax::Relationships::none(),
                    },
                )],
            }),
        ],
    };

    let result = populator.populate(&file);
    assert!(
        result.is_ok(),
        "Same name in different scopes should be allowed"
    );

    let all_symbols = table.all_symbols();
    let common_symbols: Vec<_> = all_symbols
        .iter()
        .filter(|(name, _)| *name == "Common")
        .collect();

    // Both "Common" symbols should exist with different qualified names
    assert_eq!(common_symbols.len(), 2);
}

#[test]
fn test_populate_all_usage_kinds() {
    let mut table = SymbolTable::new();
    let mut populator = SymbolTablePopulator::new(&mut table);

    let kinds = vec![
        (UsageKind::Part, "partUsage"),
        (UsageKind::Port, "portUsage"),
        (UsageKind::Action, "actionUsage"),
        (UsageKind::Item, "itemUsage"),
        (UsageKind::Requirement, "reqUsage"),
        (UsageKind::View, "viewUsage"),
    ];

    let mut elements = vec![];
    for (kind, name) in kinds {
        elements.push(Element::Usage(crate::language::sysml::syntax::Usage {
            kind,
            name: Some(name.to_string()),
            body: vec![],
            relationships: crate::language::sysml::syntax::Relationships::none(),
        }));
    }

    let file = SysMLFile {
        namespace: None,
        elements,
    };

    let result = populator.populate(&file);
    assert!(result.is_ok());

    assert!(table.lookup("partUsage").is_some());
    assert!(table.lookup("portUsage").is_some());
    assert!(table.lookup("actionUsage").is_some());
    assert!(table.lookup("itemUsage").is_some());
    assert!(table.lookup("reqUsage").is_some());
    assert!(table.lookup("viewUsage").is_some());
}

#[test]
fn test_populate_complex_hierarchy() {
    let mut table = SymbolTable::new();
    let mut populator = SymbolTablePopulator::new(&mut table);

    let file = SysMLFile {
        namespace: None,
        elements: vec![
            Element::Package(Package {
                name: Some("Root".to_string()),
                elements: vec![
                    Element::Definition(crate::language::sysml::syntax::Definition {
                        kind: DefinitionKind::Part,
                        name: Some("RootPart".to_string()),
                        body: vec![],
                        relationships: crate::language::sysml::syntax::Relationships::none(),
                    }),
                    Element::Package(Package {
                        name: Some("Sub1".to_string()),
                        elements: vec![
                            Element::Usage(crate::language::sysml::syntax::Usage {
                                kind: UsageKind::Part,
                                name: Some("sub1Usage".to_string()),
                                body: vec![],
                                relationships: crate::language::sysml::syntax::Relationships::none(
                                ),
                            }),
                            Element::Package(Package {
                                name: Some("Sub2".to_string()),
                                elements: vec![Element::Definition(
                                    crate::language::sysml::syntax::Definition {
                                        kind: DefinitionKind::Action,
                                        name: Some("DeepAction".to_string()),
                                        body: vec![],
                                        relationships:
                                            crate::language::sysml::syntax::Relationships::none(),
                                    },
                                )],
                            }),
                        ],
                    }),
                    Element::Package(Package {
                        name: Some("Sub3".to_string()),
                        elements: vec![],
                    }),
                ],
            }),
            Element::Definition(crate::language::sysml::syntax::Definition {
                kind: DefinitionKind::Port,
                name: Some("TopLevelPort".to_string()),
                body: vec![],
                relationships: crate::language::sysml::syntax::Relationships::none(),
            }),
        ],
    };

    let result = populator.populate(&file);
    assert!(result.is_ok());

    let all_symbols = table.all_symbols();

    // Verify qualified names at different levels
    let root_part = all_symbols
        .iter()
        .find(|(name, _)| *name == "RootPart")
        .map(|(_, symbol)| *symbol);
    let Some(Symbol::Definition { qualified_name, .. }) = root_part else {
        panic!("Expected Definition symbol for RootPart");
    };
    assert_eq!(qualified_name, "Root::RootPart");

    let sub1_usage = all_symbols
        .iter()
        .find(|(name, _)| *name == "sub1Usage")
        .map(|(_, symbol)| *symbol);
    let Some(Symbol::Usage { qualified_name, .. }) = sub1_usage else {
        panic!("Expected Usage symbol for sub1Usage");
    };
    assert_eq!(qualified_name, "Root::Sub1::sub1Usage");

    let deep_action = all_symbols
        .iter()
        .find(|(name, _)| *name == "DeepAction")
        .map(|(_, symbol)| *symbol);
    let Some(Symbol::Definition { qualified_name, .. }) = deep_action else {
        panic!("Expected Definition symbol for DeepAction");
    };
    assert_eq!(qualified_name, "Root::Sub1::Sub2::DeepAction");

    let top_level_port = all_symbols
        .iter()
        .find(|(name, _)| *name == "TopLevelPort")
        .map(|(_, symbol)| *symbol);
    let Some(Symbol::Definition { qualified_name, .. }) = top_level_port else {
        panic!("Expected Definition symbol for TopLevelPort");
    };
    assert_eq!(qualified_name, "TopLevelPort");
}

#[test]
fn test_populate_multiple_errors() {
    let mut table = SymbolTable::new();
    let mut populator = SymbolTablePopulator::new(&mut table);

    let file = SysMLFile {
        namespace: None,
        elements: vec![
            Element::Definition(crate::language::sysml::syntax::Definition {
                kind: DefinitionKind::Part,
                name: Some("Dup1".to_string()),
                body: vec![],
                relationships: crate::language::sysml::syntax::Relationships::none(),
            }),
            Element::Definition(crate::language::sysml::syntax::Definition {
                kind: DefinitionKind::Part,
                name: Some("Dup1".to_string()),
                body: vec![],
                relationships: crate::language::sysml::syntax::Relationships::none(),
            }),
            Element::Definition(crate::language::sysml::syntax::Definition {
                kind: DefinitionKind::Part,
                name: Some("Dup2".to_string()),
                body: vec![],
                relationships: crate::language::sysml::syntax::Relationships::none(),
            }),
            Element::Definition(crate::language::sysml::syntax::Definition {
                kind: DefinitionKind::Part,
                name: Some("Dup2".to_string()),
                body: vec![],
                relationships: crate::language::sysml::syntax::Relationships::none(),
            }),
        ],
    };

    let result = populator.populate(&file);
    assert!(result.is_err());

    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 2, "Should have 2 duplicate errors");
}

#[test]
fn test_populate_empty_package_hierarchy() {
    let mut table = SymbolTable::new();
    let mut populator = SymbolTablePopulator::new(&mut table);

    let file = SysMLFile {
        namespace: None,
        elements: vec![Element::Package(Package {
            name: Some("Empty1".to_string()),
            elements: vec![
                Element::Package(Package {
                    name: Some("Empty2".to_string()),
                    elements: vec![],
                }),
                Element::Package(Package {
                    name: Some("Empty3".to_string()),
                    elements: vec![],
                }),
            ],
        })],
    };

    let result = populator.populate(&file);
    assert!(result.is_ok());

    let all_symbols = table.all_symbols();

    let empty2 = all_symbols
        .iter()
        .find(|(name, _)| *name == "Empty2")
        .map(|(_, symbol)| *symbol);
    let Some(Symbol::Package { qualified_name, .. }) = empty2 else {
        panic!("Expected Package symbol for Empty2");
    };
    assert_eq!(qualified_name, "Empty1::Empty2");

    let empty3 = all_symbols
        .iter()
        .find(|(name, _)| *name == "Empty3")
        .map(|(_, symbol)| *symbol);
    let Some(Symbol::Package { qualified_name, .. }) = empty3 else {
        panic!("Expected Package symbol for Empty3");
    };
    assert_eq!(qualified_name, "Empty1::Empty3");
}

#[test]
fn test_populate_with_relationship_graph() {
    use crate::semantic::RelationshipGraph;

    let mut table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();
    let mut populator = SymbolTablePopulator::with_relationships(&mut table, &mut graph);

    let file = SysMLFile {
        namespace: None,
        elements: vec![Element::Definition(Definition {
            kind: DefinitionKind::Part,
            name: Some("Vehicle".to_string()),
            body: vec![],
            relationships: crate::language::sysml::syntax::Relationships::none(),
        })],
    };

    let result = populator.populate(&file);
    assert!(result.is_ok());

    // Verify symbol was added
    let symbol = table.lookup("Vehicle");
    assert!(symbol.is_some());

    // Graph is empty for now (no relationships in AST yet)
    // But the infrastructure is in place
}

#[test]
fn test_populator_without_relationship_graph() {
    // Verify backward compatibility - can still use without graph
    let mut table = SymbolTable::new();
    let mut populator = SymbolTablePopulator::new(&mut table);

    let file = SysMLFile {
        namespace: None,
        elements: vec![Element::Definition(Definition {
            kind: DefinitionKind::Part,
            name: Some("Test".to_string()),
            body: vec![],
            relationships: crate::language::sysml::syntax::Relationships::none(),
        })],
    };

    let result = populator.populate(&file);
    assert!(result.is_ok());
    assert!(table.lookup("Test").is_some());
}
