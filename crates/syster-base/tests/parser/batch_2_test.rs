#![allow(clippy::unwrap_used)]

use from_pest::FromPest;
use pest::Parser;
use std::path::PathBuf;
use syster::parser::sysml::{Rule, SysMLParser};
use syster::syntax::sysml::ast::{
    Alias, Comment, Definition, Element, Import, NamespaceDeclaration, Package, SysMLFile, Usage,
};
use syster::syntax::sysml::parser::load_and_parse;
use syster::syntax::sysml::visitor::{AstVisitor, Visitable};

// ============================================================================
// Tests for load_and_parse function (Issue #193)
// syster::syntax::sysml::parser::load_and_parse
// ============================================================================

#[test]
fn test_load_and_parse_valid_sysml_file() {
    // Create a temporary valid .sysml file
    let test_dir = std::env::temp_dir().join("batch_2_tests");
    std::fs::create_dir_all(&test_dir).unwrap();

    let test_file = test_dir.join("valid_load.sysml");
    std::fs::write(&test_file, "part def Vehicle;").unwrap();

    let result = load_and_parse(&test_file);
    assert!(
        result.is_ok(),
        "Should successfully parse valid .sysml file"
    );

    let sysml_file = result.unwrap();
    assert_eq!(
        sysml_file.elements.len(),
        1,
        "Should have one element parsed"
    );
}

#[test]
fn test_load_and_parse_valid_kerml_extension() {
    // Test that .kerml extension is accepted (even though content is SysML)
    // The SysML parser accepts both .sysml and .kerml extensions
    let test_dir = std::env::temp_dir().join("batch_2_tests");
    std::fs::create_dir_all(&test_dir).unwrap();

    let test_file = test_dir.join("valid_load.kerml");
    std::fs::write(
        &test_file,
        "package TestPackage {\n    part def TestPart;\n}\n",
    )
    .unwrap();

    let result = load_and_parse(&test_file);
    assert!(
        result.is_ok(),
        "Should accept .kerml extension and parse SysML content"
    );

    let sysml_file = result.unwrap();
    assert_eq!(
        sysml_file.elements.len(),
        1,
        "Should have one top-level element"
    );
}

#[test]
fn test_load_and_parse_invalid_extension() {
    // Test that invalid file extensions are rejected
    let test_dir = std::env::temp_dir().join("batch_2_tests");
    std::fs::create_dir_all(&test_dir).unwrap();

    let test_file = test_dir.join("invalid.txt");
    std::fs::write(&test_file, "part def Vehicle;").unwrap();

    let result = load_and_parse(&test_file);
    assert!(result.is_err(), "Should fail for invalid file extension");

    let error_msg = result.unwrap_err();
    assert!(
        error_msg.contains("Unsupported file extension"),
        "Error should mention unsupported extension: {}",
        error_msg
    );
}

#[test]
fn test_load_and_parse_nonexistent_file() {
    // Test error handling for non-existent files
    let test_file = PathBuf::from("/tmp/nonexistent_batch2_test_12345.sysml");

    let result = load_and_parse(&test_file);
    assert!(result.is_err(), "Should fail for non-existent file");

    let error_msg = result.unwrap_err();
    assert!(
        error_msg.contains("Failed to read"),
        "Error should mention failed read: {}",
        error_msg
    );
}

#[test]
fn test_load_and_parse_invalid_syntax() {
    // Test that parse errors are properly reported
    let test_dir = std::env::temp_dir().join("batch_2_tests");
    std::fs::create_dir_all(&test_dir).unwrap();

    let test_file = test_dir.join("invalid_syntax.sysml");
    // Missing semicolon
    std::fs::write(&test_file, "part def Vehicle").unwrap();

    let result = load_and_parse(&test_file);
    assert!(result.is_err(), "Should fail for invalid syntax");

    let error_msg = result.unwrap_err();
    assert!(
        error_msg.contains("Parse error"),
        "Error should mention parse error: {}",
        error_msg
    );
}

#[test]
fn test_load_and_parse_empty_file() {
    // Empty files are valid SysML
    let test_dir = std::env::temp_dir().join("batch_2_tests");
    std::fs::create_dir_all(&test_dir).unwrap();

    let test_file = test_dir.join("empty.sysml");
    std::fs::write(&test_file, "").unwrap();

    let result = load_and_parse(&test_file);
    assert!(result.is_ok(), "Should successfully parse empty file");

    let sysml_file = result.unwrap();
    assert_eq!(
        sysml_file.elements.len(),
        0,
        "Should have no elements in empty file"
    );
}

#[test]
fn test_load_and_parse_with_package() {
    // Test parsing file with package declaration
    let test_dir = std::env::temp_dir().join("batch_2_tests");
    std::fs::create_dir_all(&test_dir).unwrap();

    let test_file = test_dir.join("with_package.sysml");
    std::fs::write(&test_file, "package MyPackage {\n    part def Vehicle;\n}").unwrap();

    let result = load_and_parse(&test_file);
    assert!(result.is_ok(), "Should parse file with package");

    let sysml_file = result.unwrap();
    assert_eq!(
        sysml_file.elements.len(),
        1,
        "Should have one package element"
    );
}

#[test]
fn test_load_and_parse_multiple_elements() {
    // Test parsing file with multiple elements
    let test_dir = std::env::temp_dir().join("batch_2_tests");
    std::fs::create_dir_all(&test_dir).unwrap();

    let test_file = test_dir.join("multiple.sysml");
    std::fs::write(
        &test_file,
        "part def Car;\npart def Truck;\naction def Drive;",
    )
    .unwrap();

    let result = load_and_parse(&test_file);
    assert!(result.is_ok(), "Should parse multiple elements");

    let sysml_file = result.unwrap();
    assert_eq!(sysml_file.elements.len(), 3, "Should have three elements");
}

#[test]
fn test_load_and_parse_with_unicode() {
    // Test handling of unicode content
    let test_dir = std::env::temp_dir().join("batch_2_tests");
    std::fs::create_dir_all(&test_dir).unwrap();

    let test_file = test_dir.join("unicode.sysml");
    std::fs::write(&test_file, "part def Vehicle; // 中文注释").unwrap();

    let result = load_and_parse(&test_file);
    assert!(result.is_ok(), "Should handle unicode content");
}

#[test]
fn test_load_and_parse_with_crlf_line_endings() {
    // Test handling of Windows-style line endings
    let test_dir = std::env::temp_dir().join("batch_2_tests");
    std::fs::create_dir_all(&test_dir).unwrap();

    let test_file = test_dir.join("crlf.sysml");
    std::fs::write(&test_file, "part def Vehicle;\r\npart def Engine;\r\n").unwrap();

    let result = load_and_parse(&test_file);
    assert!(result.is_ok(), "Should handle CRLF line endings");

    let sysml_file = result.unwrap();
    assert_eq!(sysml_file.elements.len(), 2, "Should parse both elements");
}

#[test]
fn test_load_and_parse_with_imports() {
    // Test parsing file with import statements
    let test_dir = std::env::temp_dir().join("batch_2_tests");
    std::fs::create_dir_all(&test_dir).unwrap();

    let test_file = test_dir.join("with_imports.sysml");
    std::fs::write(&test_file, "import Base::*;\npart def Vehicle;").unwrap();

    let result = load_and_parse(&test_file);
    assert!(result.is_ok(), "Should parse file with imports");
}

#[test]
fn test_load_and_parse_ast_construction() {
    // Test that the AST is properly constructed from loaded file
    let test_dir = std::env::temp_dir().join("batch_2_tests");
    std::fs::create_dir_all(&test_dir).unwrap();

    let test_file = test_dir.join("ast_test.sysml");
    std::fs::write(&test_file, "package TestPkg;\npart def TestDef;").unwrap();

    let result = load_and_parse(&test_file);
    assert!(result.is_ok(), "Should successfully construct AST");

    let sysml_file = result.unwrap();
    // Should have namespace and elements
    assert!(
        sysml_file.namespace.is_some(),
        "Should have namespace declaration"
    );
    assert!(!sysml_file.elements.is_empty(), "Should have elements");
}

// ============================================================================
// Helper visitor for testing accept methods
// ============================================================================

struct CountingVisitor {
    file_visits: usize,
    namespace_visits: usize,
    element_visits: usize,
    package_visits: usize,
    definition_visits: usize,
    usage_visits: usize,
    comment_visits: usize,
    import_visits: usize,
    alias_visits: usize,
}

impl CountingVisitor {
    fn new() -> Self {
        Self {
            file_visits: 0,
            namespace_visits: 0,
            element_visits: 0,
            package_visits: 0,
            definition_visits: 0,
            usage_visits: 0,
            comment_visits: 0,
            import_visits: 0,
            alias_visits: 0,
        }
    }
}

impl AstVisitor for CountingVisitor {
    fn visit_file(&mut self, _file: &SysMLFile) {
        self.file_visits += 1;
    }

    fn visit_namespace(&mut self, _namespace: &NamespaceDeclaration) {
        self.namespace_visits += 1;
    }

    fn visit_element(&mut self, _element: &Element) {
        self.element_visits += 1;
    }

    fn visit_package(&mut self, _package: &Package) {
        self.package_visits += 1;
    }

    fn visit_definition(&mut self, _definition: &Definition) {
        self.definition_visits += 1;
    }

    fn visit_usage(&mut self, _usage: &Usage) {
        self.usage_visits += 1;
    }

    fn visit_comment(&mut self, _comment: &Comment) {
        self.comment_visits += 1;
    }

    fn visit_import(&mut self, _import: &Import) {
        self.import_visits += 1;
    }

    fn visit_alias(&mut self, _alias: &Alias) {
        self.alias_visits += 1;
    }
}

// ============================================================================
// Tests for SysMLFile::accept (Issue #173)
// <syster::syntax::sysml::ast::types::SysMLFile as syster::syntax::sysml::visitor::Visitable>::accept::<_>
// ============================================================================

#[test]
fn test_sysmlfile_accept_calls_visit_file() {
    // Test that accept calls visit_file on the visitor
    let source = "part def Vehicle;";
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(
        visitor.file_visits, 1,
        "accept should call visit_file exactly once"
    );
}

#[test]
fn test_sysmlfile_accept_traverses_namespace() {
    // Test that accept traverses the namespace declaration
    let source = "package MyPackage;";
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(visitor.file_visits, 1);
    assert_eq!(
        visitor.namespace_visits, 1,
        "accept should traverse namespace"
    );
}

#[test]
fn test_sysmlfile_accept_traverses_elements() {
    // Test that accept traverses all elements
    let source = "part def Car;\npart def Truck;";
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(visitor.file_visits, 1);
    assert_eq!(
        visitor.element_visits, 2,
        "accept should traverse all elements"
    );
    assert_eq!(
        visitor.definition_visits, 2,
        "accept should traverse to definition level"
    );
}

#[test]
fn test_sysmlfile_accept_with_empty_file() {
    // Test accept with empty file (no namespace, no elements)
    let source = "";
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(visitor.file_visits, 1, "Should still call visit_file");
    assert_eq!(visitor.namespace_visits, 0, "Should not visit namespace");
    assert_eq!(visitor.element_visits, 0, "Should not visit any elements");
}

#[test]
fn test_sysmlfile_accept_with_namespace_and_elements() {
    // Test accept with both namespace and elements
    let source = "package Vehicles;\npart def Car;";
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(visitor.file_visits, 1);
    assert_eq!(visitor.namespace_visits, 1);
    // Package declaration creates both namespace and Package element
    assert!(visitor.element_visits >= 1, "Should visit elements");
}

#[test]
fn test_sysmlfile_accept_with_mixed_elements() {
    // Test accept with various element types
    let source = r#"
        import Base::*;
        part def Vehicle;
        part myCar;
        alias MyAlias for Type;
    "#;
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(visitor.file_visits, 1);
    assert_eq!(
        visitor.element_visits, 4,
        "Should visit import, definition, usage, and alias"
    );
    assert_eq!(visitor.import_visits, 1);
    assert_eq!(visitor.definition_visits, 1);
    assert_eq!(visitor.usage_visits, 1);
    assert_eq!(visitor.alias_visits, 1);
}

#[test]
fn test_sysmlfile_accept_with_complex_nested_structure() {
    // Test accept with nested packages and definitions
    let source = r#"
        package Vehicles {
            import Base::*;
            part def Car;
            package Subsystem {
                part def Engine;
            }
        }
    "#;
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(visitor.file_visits, 1);
    assert_eq!(
        visitor.package_visits, 2,
        "Should visit both outer and inner packages"
    );
    assert_eq!(
        visitor.definition_visits, 2,
        "Should visit both definitions"
    );
    assert_eq!(visitor.import_visits, 1);
}

#[test]
fn test_sysmlfile_accept_visitor_receives_correct_data() {
    // Test that visitor receives correct file data
    struct FileChecker {
        element_count: usize,
        has_namespace: bool,
    }

    impl AstVisitor for FileChecker {
        fn visit_file(&mut self, file: &SysMLFile) {
            self.element_count = file.elements.len();
            self.has_namespace = file.namespace.is_some();
        }
    }

    let source = "package TestPkg;\npart def Vehicle;";
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut visitor = FileChecker {
        element_count: 0,
        has_namespace: false,
    };
    file.accept(&mut visitor);

    assert!(visitor.has_namespace, "Should detect namespace");
    assert!(visitor.element_count > 0, "Should count elements");
}

// ============================================================================
// Tests for Definition::accept (Issue #164)
// <syster::syntax::sysml::ast::types::Definition as syster::syntax::sysml::visitor::Visitable>::accept::<_>
// ============================================================================

#[test]
fn test_definition_accept_calls_visit_definition() {
    // Test that accept calls visit_definition
    let source = "part def Vehicle;";
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(
        visitor.definition_visits, 1,
        "accept should call visit_definition"
    );
}

#[test]
fn test_definition_accept_with_multiple_definitions() {
    // Test accept with multiple definitions
    let source = "part def Car;\naction def Drive;\nrequirement def Safety;";
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(
        visitor.definition_visits, 3,
        "Should visit all three definitions"
    );
}

#[test]
fn test_definition_accept_receives_correct_data() {
    // Test that visitor receives correct definition data
    struct DefinitionChecker {
        definition_names: Vec<String>,
    }

    impl AstVisitor for DefinitionChecker {
        fn visit_definition(&mut self, definition: &Definition) {
            if let Some(ref name) = definition.name {
                self.definition_names.push(name.clone());
            }
        }
    }

    let source = "part def Car;\naction def Drive;";
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut visitor = DefinitionChecker {
        definition_names: Vec::new(),
    };
    file.accept(&mut visitor);

    assert_eq!(visitor.definition_names.len(), 2);
    assert!(visitor.definition_names.contains(&"Car".to_string()));
    assert!(visitor.definition_names.contains(&"Drive".to_string()));
}

#[test]
fn test_definition_accept_with_anonymous_definition() {
    // Test accept with anonymous (unnamed) definition
    let source = "part def;";
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(
        visitor.definition_visits, 1,
        "Should visit anonymous definition"
    );
}

#[test]
fn test_definition_accept_all_definition_kinds() {
    // Test accept with various definition kinds
    let source = r#"
        part def PartDef;
        action def ActionDef;
        requirement def ReqDef;
        port def PortDef;
        item def ItemDef;
        attribute def AttrDef;
    "#;
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(
        visitor.definition_visits, 6,
        "Should visit all definition kinds"
    );
}

#[test]
fn test_definition_accept_nested_in_package() {
    // Test that definitions nested in packages are visited
    let source = r#"
        package MyPackage {
            part def Vehicle;
            action def Drive;
        }
    "#;
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(
        visitor.definition_visits, 2,
        "Should visit nested definitions"
    );
}

#[test]
fn test_definition_accept_with_specialization() {
    // Test accept with definition that has specialization
    let source = "part def Car :> Vehicle;";
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(
        visitor.definition_visits, 1,
        "Should visit definition with specialization"
    );
}

// ============================================================================
// Tests for Package::accept (Issue #163)
// <syster::syntax::sysml::ast::types::Package as syster::syntax::sysml::visitor::Visitable>::accept::<_>
// ============================================================================

#[test]
fn test_package_accept_calls_visit_package() {
    // Test that accept calls visit_package
    let source = "package TestPackage { }";
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(
        visitor.package_visits, 1,
        "accept should call visit_package"
    );
}

#[test]
fn test_package_accept_traverses_nested_elements() {
    // Test that accept traverses nested elements in package
    let source = r#"
        package OuterPackage {
            part def Car;
            part def Truck;
        }
    "#;
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(visitor.package_visits, 1);
    assert_eq!(
        visitor.definition_visits, 2,
        "Should traverse nested definitions"
    );
}

#[test]
fn test_package_accept_with_empty_body() {
    // Test accept with empty package
    let source = "package EmptyPackage { }";
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(visitor.package_visits, 1);
    assert_eq!(
        visitor.definition_visits, 0,
        "Should not visit any nested elements"
    );
}

#[test]
fn test_package_accept_with_nested_packages() {
    // Test accept with nested packages
    let source = r#"
        package Outer {
            package Inner1 { }
            package Inner2 { }
        }
    "#;
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(
        visitor.package_visits, 3,
        "Should visit outer and both inner packages"
    );
}

#[test]
fn test_package_accept_receives_correct_data() {
    // Test that visitor receives correct package data
    struct PackageChecker {
        package_names: Vec<String>,
    }

    impl AstVisitor for PackageChecker {
        fn visit_package(&mut self, package: &Package) {
            if let Some(ref name) = package.name {
                self.package_names.push(name.clone());
            }
        }
    }

    let source = "package First { }\npackage Second { }";
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut visitor = PackageChecker {
        package_names: Vec::new(),
    };
    file.accept(&mut visitor);

    assert_eq!(visitor.package_names.len(), 2);
    assert!(visitor.package_names.contains(&"First".to_string()));
    assert!(visitor.package_names.contains(&"Second".to_string()));
}

#[test]
fn test_package_accept_with_deeply_nested_structure() {
    // Test accept with deeply nested packages
    let source = r#"
        package Level1 {
            package Level2 {
                part def Component;
            }
        }
    "#;
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(visitor.package_visits, 2);
    assert_eq!(visitor.definition_visits, 1);
}

#[test]
fn test_package_accept_with_mixed_nested_elements() {
    // Test accept with various nested element types
    let source = r#"
        package MyPackage {
            import Base::*;
            part def Vehicle;
            part myCar;
        }
    "#;
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(visitor.package_visits, 1);
    assert_eq!(visitor.import_visits, 1);
    assert_eq!(visitor.definition_visits, 1);
    // Part usages at top level inside package body are visited
    assert_eq!(
        visitor.usage_visits, 1,
        "Should visit top-level usage in package"
    );
}

// ============================================================================
// Tests for Element::accept (Issue #160)
// <syster::syntax::sysml::ast::enums::Element as syster::syntax::sysml::visitor::Visitable>::accept::<_>
// ============================================================================

#[test]
fn test_element_accept_calls_visit_element() {
    // Test that accept calls visit_element for all element types
    let source = "part def Vehicle;";
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(
        visitor.element_visits, 1,
        "accept should call visit_element"
    );
}

#[test]
fn test_element_accept_dispatches_to_package() {
    // Test that Element::accept dispatches to Package::accept
    let source = "package MyPackage { }";
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(visitor.element_visits, 1);
    assert_eq!(
        visitor.package_visits, 1,
        "Should dispatch to visit_package"
    );
}

#[test]
fn test_element_accept_dispatches_to_definition() {
    // Test that Element::accept dispatches to Definition::accept
    let source = "part def Vehicle;";
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(visitor.element_visits, 1);
    assert_eq!(
        visitor.definition_visits, 1,
        "Should dispatch to visit_definition"
    );
}

#[test]
fn test_element_accept_dispatches_to_usage() {
    // Test that Element::accept dispatches to Usage::accept
    let source = "part myCar;";
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(visitor.element_visits, 1);
    assert_eq!(visitor.usage_visits, 1, "Should dispatch to visit_usage");
}

#[test]
fn test_element_accept_dispatches_to_import() {
    // Test that Element::accept dispatches to Import::accept
    let source = "import Package::*;";
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(visitor.element_visits, 1);
    assert_eq!(visitor.import_visits, 1, "Should dispatch to visit_import");
}

#[test]
fn test_element_accept_dispatches_to_alias() {
    // Test that Element::accept dispatches to Alias::accept
    let source = "alias MyAlias for SomeType;";
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(visitor.element_visits, 1);
    assert_eq!(visitor.alias_visits, 1, "Should dispatch to visit_alias");
}

#[test]
fn test_element_accept_with_mixed_element_types() {
    // Test accept with multiple different element types
    let source = r#"
        import Base::*;
        package MyPkg { }
        part def Car;
        part myCar;
        alias MyAlias for Type;
    "#;
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(visitor.element_visits, 5, "Should visit all 5 elements");
    assert_eq!(visitor.import_visits, 1);
    assert_eq!(visitor.package_visits, 1);
    assert_eq!(visitor.definition_visits, 1);
    assert_eq!(visitor.usage_visits, 1);
    assert_eq!(visitor.alias_visits, 1);
}

#[test]
fn test_element_accept_dispatches_correctly_for_each_variant() {
    // Test that each Element variant dispatches to correct visitor method
    let test_cases = vec![
        ("package Pkg { }", 1, 0, 0, 0, 0), // Package
        ("part def Def;", 0, 1, 0, 0, 0),   // Definition
        ("part usage;", 0, 0, 1, 0, 0),     // Usage
        ("import Pkg::*;", 0, 0, 0, 1, 0),  // Import
        ("alias A for B;", 0, 0, 0, 0, 1),  // Alias
    ];

    for (source, pkg, def, usage, import, alias) in test_cases {
        let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
        let file = SysMLFile::from_pest(&mut pairs).unwrap();

        let mut visitor = CountingVisitor::new();
        file.accept(&mut visitor);

        assert_eq!(visitor.package_visits, pkg, "Failed for: {}", source);
        assert_eq!(visitor.definition_visits, def, "Failed for: {}", source);
        assert_eq!(visitor.usage_visits, usage, "Failed for: {}", source);
        assert_eq!(visitor.import_visits, import, "Failed for: {}", source);
        assert_eq!(visitor.alias_visits, alias, "Failed for: {}", source);
    }
}

#[test]
fn test_element_accept_receives_correct_element_data() {
    // Test that visit_element receives correct Element variant
    struct ElementTypeChecker {
        has_package: bool,
        has_definition: bool,
        has_usage: bool,
        has_import: bool,
        has_alias: bool,
    }

    impl AstVisitor for ElementTypeChecker {
        fn visit_element(&mut self, element: &Element) {
            match element {
                Element::Package(_) => self.has_package = true,
                Element::Definition(_) => self.has_definition = true,
                Element::Usage(_) => self.has_usage = true,
                Element::Import(_) => self.has_import = true,
                Element::Alias(_) => self.has_alias = true,
                Element::Comment(_) => {}
            }
        }
    }

    let source = r#"
        package Pkg { }
        part def Def;
        part usage;
        import Pkg::*;
        alias A for B;
    "#;
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut visitor = ElementTypeChecker {
        has_package: false,
        has_definition: false,
        has_usage: false,
        has_import: false,
        has_alias: false,
    };
    file.accept(&mut visitor);

    assert!(visitor.has_package, "Should detect package element");
    assert!(visitor.has_definition, "Should detect definition element");
    assert!(visitor.has_usage, "Should detect usage element");
    assert!(visitor.has_import, "Should detect import element");
    assert!(visitor.has_alias, "Should detect alias element");
}

#[test]
fn test_element_accept_nested_elements_traversal() {
    // Test that nested elements are properly traversed
    let source = r#"
        package Outer {
            package Inner {
                part def Vehicle;
            }
        }
    "#;
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    // Outer package (1) + Inner package (1) + Definition (1) = 3 elements
    assert_eq!(
        visitor.element_visits, 3,
        "Should visit all nested elements"
    );
    assert_eq!(visitor.package_visits, 2);
    assert_eq!(visitor.definition_visits, 1);
}
