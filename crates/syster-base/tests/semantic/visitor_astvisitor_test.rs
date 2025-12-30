#![allow(clippy::unwrap_used)]

use from_pest::FromPest;
use pest::Parser;
use syster::parser::sysml::{Rule, SysMLParser};
use syster::syntax::sysml::ast::{
    Alias, Comment, Definition, Element, Import, NamespaceDeclaration, Package, SysMLFile, Usage,
};
use syster::syntax::sysml::visitor::{AstVisitor, Visitable};

// Helper visitor that counts all visit method invocations
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

// Helper visitor that collects element names
struct NameCollector {
    names: Vec<String>,
}

impl NameCollector {
    fn new() -> Self {
        Self { names: Vec::new() }
    }
}

impl AstVisitor for NameCollector {
    fn visit_package(&mut self, package: &Package) {
        if let Some(ref name) = package.name {
            self.names.push(name.clone());
        }
    }

    fn visit_definition(&mut self, definition: &Definition) {
        if let Some(ref name) = definition.name {
            self.names.push(name.clone());
        }
    }

    fn visit_usage(&mut self, usage: &Usage) {
        if let Some(ref name) = usage.name {
            self.names.push(name.clone());
        }
    }
}

// ============================================================================
// visit_file tests
// ============================================================================

#[test]
fn test_visit_file_is_called() {
    let source = "part def Vehicle;";
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(visitor.file_visits, 1, "visit_file should be called once");
}

#[test]
fn test_visit_file_traverses_elements() {
    let source = r#"
        part def Car;
        part def Truck;
    "#;
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(visitor.file_visits, 1);
    assert_eq!(
        visitor.element_visits, 2,
        "Should visit both element wrappers"
    );
    assert_eq!(
        visitor.definition_visits, 2,
        "Should visit both definitions"
    );
}

#[test]
fn test_visit_file_with_namespace() {
    let source = "package MyPackage;";
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(visitor.file_visits, 1);
    assert_eq!(
        visitor.namespace_visits, 1,
        "Should visit namespace declaration"
    );
}

#[test]
fn test_visit_file_with_empty_file() {
    let source = "";
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(visitor.file_visits, 1, "Should still call visit_file");
    assert_eq!(
        visitor.element_visits, 0,
        "Should not visit any elements in empty file"
    );
}

#[test]
fn test_visit_file_with_namespace_and_elements() {
    let source = r#"
        package Vehicles;
        part def Car;
    "#;
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(visitor.file_visits, 1);
    assert_eq!(visitor.namespace_visits, 1);
    // Note: "package Vehicles;" creates both a namespace and a Package element
    assert_eq!(
        visitor.element_visits, 2,
        "Should visit Package element and Definition element"
    );
    assert_eq!(visitor.package_visits, 1);
    assert_eq!(visitor.definition_visits, 1);
}

// ============================================================================
// visit_namespace tests
// ============================================================================

#[test]
fn test_visit_namespace_is_called() {
    let source = "package TestPackage;";
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(
        visitor.namespace_visits, 1,
        "visit_namespace should be called"
    );
}

#[test]
fn test_visit_namespace_with_no_namespace() {
    let source = "part def Vehicle;";
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(
        visitor.namespace_visits, 0,
        "Should not call visit_namespace when no namespace present"
    );
}

#[test]
fn test_visit_namespace_receives_correct_data() {
    struct NamespaceChecker {
        namespace_name: Option<String>,
    }

    impl AstVisitor for NamespaceChecker {
        fn visit_namespace(&mut self, namespace: &NamespaceDeclaration) {
            self.namespace_name = Some(namespace.name.clone());
        }
    }

    let source = "package MySpecialPackage;";
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut visitor = NamespaceChecker {
        namespace_name: None,
    };
    file.accept(&mut visitor);

    assert_eq!(
        visitor.namespace_name,
        Some("MySpecialPackage".to_string()),
        "Should receive correct namespace name"
    );
}

// ============================================================================
// visit_element tests
// ============================================================================

#[test]
fn test_visit_element_is_called_for_each_element() {
    let source = r#"
        package Pkg;
        part def Car;
        part engine;
    "#;
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(
        visitor.element_visits, 3,
        "Should call visit_element for package, definition, and usage"
    );
}

#[test]
fn test_visit_element_dispatches_to_package() {
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
fn test_visit_element_dispatches_to_definition() {
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
fn test_visit_element_dispatches_to_usage() {
    let source = "part myCar;";
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(visitor.element_visits, 1);
    assert_eq!(visitor.usage_visits, 1, "Should dispatch to visit_usage");
}

#[test]
fn test_visit_element_dispatches_to_import() {
    let source = "import Package::*;";
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(visitor.element_visits, 1);
    assert_eq!(visitor.import_visits, 1, "Should dispatch to visit_import");
}

#[test]
fn test_visit_element_dispatches_to_alias() {
    let source = "alias MyAlias for SomeType;";
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(visitor.element_visits, 1);
    assert_eq!(visitor.alias_visits, 1, "Should dispatch to visit_alias");
}

#[test]
fn test_visit_element_with_mixed_element_types() {
    let source = r#"
        import Base::*;
        package MyPkg { 
            
        }
        part def Car;
        part myCar;
        alias MyAlias for Type;
    "#;
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(
        visitor.element_visits, 5,
        "Should visit import, package, definition, usage, and alias"
    );
    assert_eq!(visitor.import_visits, 1);
    assert_eq!(visitor.package_visits, 1);

    assert_eq!(visitor.definition_visits, 1);
    assert_eq!(visitor.usage_visits, 1);
    assert_eq!(visitor.alias_visits, 1);
}

// ============================================================================
// visit_package tests
// ============================================================================

#[test]
fn test_visit_package_is_called() {
    let source = "package TestPackage { }";
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(visitor.package_visits, 1, "Should call visit_package");
}

#[test]
fn test_visit_package_traverses_nested_elements() {
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
        visitor.element_visits, 3,
        "Should visit package element + 2 nested definition elements"
    );
    assert_eq!(
        visitor.definition_visits, 2,
        "Should visit both nested definitions"
    );
}

#[test]
fn test_visit_package_with_empty_body() {
    let source = "package EmptyPackage { }";
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(visitor.package_visits, 1);
    assert_eq!(
        visitor.element_visits, 1,
        "Should only visit the package element itself"
    );
}

#[test]
fn test_visit_package_with_nested_packages() {
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
        "Should visit outer package and both inner packages"
    );
}

#[test]
fn test_visit_package_receives_correct_data() {
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

    let source = r#"
        package First { }
        package Second { }
    "#;
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
fn test_visit_package_with_deeply_nested_structure() {
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

// ============================================================================
// visit_definition tests
// ============================================================================

#[test]
fn test_visit_definition_is_called() {
    let source = "part def Vehicle;";
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(visitor.definition_visits, 1, "Should call visit_definition");
}

#[test]
fn test_visit_definition_with_multiple_definitions() {
    let source = r#"
        part def Car;
        action def Drive;
        requirement def Safety;
    "#;
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(
        visitor.definition_visits, 3,
        "Should visit all 3 definitions"
    );
}

#[test]
fn test_visit_definition_receives_correct_data() {
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

    let source = r#"
        part def Car;
        action def Drive;
    "#;
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
fn test_visit_definition_with_anonymous_definition() {
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
fn test_visit_definition_all_definition_kinds() {
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

// ============================================================================
// visit_usage tests
// ============================================================================

#[test]
fn test_visit_usage_is_called() {
    let source = "part myCar;";
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(visitor.usage_visits, 1, "Should call visit_usage");
}

#[test]
fn test_visit_usage_with_multiple_usages() {
    let source = r#"
        part car1;
        part car2;
        action drive;
    "#;
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(visitor.usage_visits, 3, "Should visit all 3 usages");
}

#[test]
fn test_visit_usage_receives_correct_data() {
    struct UsageChecker {
        usage_names: Vec<String>,
    }

    impl AstVisitor for UsageChecker {
        fn visit_usage(&mut self, usage: &Usage) {
            if let Some(ref name) = usage.name {
                self.usage_names.push(name.clone());
            }
        }
    }

    let source = r#"
        part car;
        action drive;
    "#;
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut visitor = UsageChecker {
        usage_names: Vec::new(),
    };
    file.accept(&mut visitor);

    assert_eq!(visitor.usage_names.len(), 2);
    assert!(visitor.usage_names.contains(&"car".to_string()));
    assert!(visitor.usage_names.contains(&"drive".to_string()));
}

#[test]
fn test_visit_usage_with_typed_usage() {
    let source = "part myCar : Vehicle;";
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(visitor.usage_visits, 1, "Should visit typed usage");
}

#[test]
fn test_visit_usage_all_usage_kinds() {
    let source = r#"
        part myPart;
        action myAction;
        requirement myReq;
        port myPort;
        item myItem;
        attribute myAttr;
    "#;
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(visitor.usage_visits, 6, "Should visit all usage kinds");
}

// ============================================================================
// visit_import tests
// ============================================================================

#[test]
fn test_visit_import_is_called() {
    let source = "import Package::*;";
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(visitor.import_visits, 1, "Should call visit_import");
}

#[test]
fn test_visit_import_with_multiple_imports() {
    let source = r#"
        import Package1::*;
        import Package2::Type;
    "#;
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(visitor.import_visits, 2, "Should visit both imports");
}

#[test]
fn test_visit_import_receives_correct_data() {
    struct ImportChecker {
        import_paths: Vec<String>,
        recursive_flags: Vec<bool>,
    }

    impl AstVisitor for ImportChecker {
        fn visit_import(&mut self, import: &Import) {
            self.import_paths.push(import.path.clone());
            self.recursive_flags.push(import.is_recursive);
        }
    }

    let source = r#"
        import Package::*;
        import Package::Type;
    "#;
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut visitor = ImportChecker {
        import_paths: Vec::new(),
        recursive_flags: Vec::new(),
    };
    file.accept(&mut visitor);

    assert_eq!(visitor.import_paths.len(), 2);
    assert!(visitor.import_paths.contains(&"Package::*".to_string()));
    assert!(visitor.import_paths.contains(&"Package::Type".to_string()));
}

#[test]
fn test_visit_import_in_package() {
    let source = r#"
        package MyPackage {
            import Base::*;
        }
    "#;
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(
        visitor.import_visits, 1,
        "Should visit import inside package"
    );
    assert_eq!(visitor.package_visits, 1);
}

#[test]
fn test_visit_import_mixed_with_other_elements() {
    let source = r#"
        import Base::*;
        part def Car;
        import Types::*;
    "#;
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(visitor.import_visits, 2);
    assert_eq!(visitor.definition_visits, 1);
}

// ============================================================================
// visit_alias tests
// ============================================================================

#[test]
fn test_visit_alias_is_called() {
    let source = "alias MyAlias for SomeType;";
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(visitor.alias_visits, 1, "Should call visit_alias");
}

#[test]
fn test_visit_alias_with_multiple_aliases() {
    let source = r#"
        alias Alias1 for Type1;
        alias Alias2 for Type2;
    "#;
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(visitor.alias_visits, 2, "Should visit both aliases");
}

#[test]
fn test_visit_alias_receives_correct_data() {
    struct AliasChecker {
        alias_names: Vec<String>,
        alias_targets: Vec<String>,
    }

    impl AstVisitor for AliasChecker {
        fn visit_alias(&mut self, alias: &Alias) {
            if let Some(ref name) = alias.name {
                self.alias_names.push(name.clone());
            }
            self.alias_targets.push(alias.target.clone());
        }
    }

    let source = r#"
        alias MyAlias for TargetType;
        alias AnotherAlias for AnotherType;
    "#;
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut visitor = AliasChecker {
        alias_names: Vec::new(),
        alias_targets: Vec::new(),
    };
    file.accept(&mut visitor);

    assert_eq!(visitor.alias_names.len(), 2);
    assert!(visitor.alias_names.contains(&"MyAlias".to_string()));
    assert!(visitor.alias_names.contains(&"AnotherAlias".to_string()));
    assert_eq!(visitor.alias_targets.len(), 2);
    assert!(visitor.alias_targets.contains(&"TargetType".to_string()));
    assert!(visitor.alias_targets.contains(&"AnotherType".to_string()));
}

#[test]
fn test_visit_alias_mixed_with_other_elements() {
    let source = r#"
        part def Car;
        alias Vehicle for Car;
        part myCar;
    "#;
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(visitor.alias_visits, 1);
    assert_eq!(visitor.definition_visits, 1);
    assert_eq!(visitor.usage_visits, 1);
}

// ============================================================================
// visit_comment tests
// ============================================================================

#[test]
fn test_visit_comment_with_manually_constructed_ast() {
    // Since comment parsing from source is complex, test the visitor with manually constructed AST
    // This test verifies the visitor callback works when called directly
    struct CommentVisitor {
        comment_count: usize,
    }

    impl AstVisitor for CommentVisitor {
        fn visit_comment(&mut self, _comment: &Comment) {
            self.comment_count += 1;
        }
    }

    // Test that the visitor callback works by calling it directly
    let comment = Comment {
        content: "test comment".to_string(),
        span: None,
    };

    let mut visitor = CommentVisitor { comment_count: 0 };
    visitor.visit_comment(&comment);

    assert_eq!(
        visitor.comment_count, 1,
        "visit_comment callback should work when called directly"
    );
}

#[test]
fn test_visit_comment_receives_comment_data() {
    // Test that visit_comment receives correct comment data
    struct CommentContentChecker {
        content: String,
    }

    impl AstVisitor for CommentContentChecker {
        fn visit_comment(&mut self, comment: &Comment) {
            self.content = comment.content.clone();
        }
    }

    let comment = Comment {
        content: "This is a test comment".to_string(),
        span: None,
    };

    let mut visitor = CommentContentChecker {
        content: String::new(),
    };
    visitor.visit_comment(&comment);

    assert_eq!(visitor.content, "This is a test comment");
}

// ============================================================================
// Integration tests - Complex scenarios
// ============================================================================

#[test]
fn test_visitor_with_complex_nested_structure() {
    let source = r#"
        package Vehicles {
            import Base::*;
            
            part def Car {
                part engine;
                attribute mass;
            }
            
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
    assert_eq!(visitor.package_visits, 2, "Should visit both packages");
    assert_eq!(visitor.import_visits, 1);
    assert_eq!(
        visitor.definition_visits, 2,
        "Should visit both definitions"
    );
    assert_eq!(
        visitor.usage_visits, 0,
        "Nested usages should not be visited"
    );
}

#[test]
fn test_visitor_collects_all_names_in_order() {
    let source = r#"
        package First { }
        part def Second;
        part third;
    "#;
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut visitor = NameCollector::new();
    file.accept(&mut visitor);

    assert_eq!(visitor.names.len(), 3);
    assert_eq!(visitor.names[0], "First");
    assert_eq!(visitor.names[1], "Second");
    assert_eq!(visitor.names[2], "third");
}

#[test]
fn test_visitor_with_all_element_types() {
    let source = r#"
        package Pkg { 
            
        }
        part def PartDef;
        part partUsage;
        import Base::*;
        alias MyAlias for Type;
    "#;
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(visitor.file_visits, 1);
    assert_eq!(
        visitor.element_visits, 5,
        "Package, definition, usage, import, alias"
    );
    assert_eq!(visitor.package_visits, 1);

    assert_eq!(visitor.definition_visits, 1);
    assert_eq!(visitor.usage_visits, 1);
    assert_eq!(visitor.import_visits, 1);
    assert_eq!(visitor.alias_visits, 1);
}

#[test]
fn test_visitor_default_implementations_do_nothing() {
    struct EmptyVisitor {}

    impl AstVisitor for EmptyVisitor {}

    let source = r#"
        package Pkg;
        part def Car;
    "#;
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut visitor = EmptyVisitor {};
    // Should not panic with default implementations
    file.accept(&mut visitor);
}

#[test]
fn test_visitor_selective_override() {
    struct DefinitionOnlyVisitor {
        definitions: Vec<String>,
    }

    impl AstVisitor for DefinitionOnlyVisitor {
        fn visit_definition(&mut self, definition: &Definition) {
            if let Some(ref name) = definition.name {
                self.definitions.push(name.clone());
            }
        }
        // All other methods use default (empty) implementations
    }

    let source = r#"
        package Pkg;
        part def Car;
        part def Truck;
        part myCar;
    "#;
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut visitor = DefinitionOnlyVisitor {
        definitions: Vec::new(),
    };
    file.accept(&mut visitor);

    assert_eq!(
        visitor.definitions.len(),
        2,
        "Should only collect definitions"
    );
    assert!(visitor.definitions.contains(&"Car".to_string()));
    assert!(visitor.definitions.contains(&"Truck".to_string()));
}

#[test]
fn test_visitor_with_multiple_packages_at_top_level() {
    let source = r#"
        package First { }
        package Second { }
        package Third { }
    "#;
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(
        visitor.package_visits, 3,
        "Should visit all top-level packages"
    );
}
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
    // File-level package declaration creates a namespace declaration and may also create a Package element
    assert!(
        visitor.element_visits >= 1,
        "Should visit at least one element (part def)"
    );
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
    assert!(
        visitor.element_count >= 1,
        "Should count at least one element (part def)"
    );
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
