use super::utils::{
    extract_definition_flags, extract_flags, extract_relationships, find_identifier_span,
    find_name, is_body_rule, is_usage_rule, to_def_kind, to_span, to_usage_kind,
};
use super::{
    enums::{DefinitionMember, UsageMember},
    types::{Comment, Definition, Usage},
};
use crate::parser::sysml::Rule;
use from_pest::{ConversionError, Void};
use pest::iterators::Pair;

// ============================================================================
// Body parsing
// ============================================================================

/// Parse definition body members
pub fn parse_def_body(pair: &Pair<Rule>) -> Vec<DefinitionMember> {
    let mut members = Vec::new();
    extract_def_members(pair, &mut members);
    members
}

fn extract_def_members(pair: &Pair<Rule>, members: &mut Vec<DefinitionMember>) {
    if is_usage_rule(pair.as_rule()) {
        members.push(DefinitionMember::Usage(Box::new(parse_usage(pair.clone()))));
    } else {
        for inner in pair.clone().into_inner() {
            extract_def_members(&inner, members);
        }
    }
}

/// Parse usage body members
pub fn parse_usage_body(pair: &Pair<Rule>) -> Vec<UsageMember> {
    let mut members = Vec::new();
    extract_usage_members(pair, &mut members);
    members
}

fn extract_usage_members(pair: &Pair<Rule>, members: &mut Vec<UsageMember>) {
    match pair.as_rule() {
        Rule::documentation | Rule::block_comment => {
            members.push(UsageMember::Comment(Comment {
                content: pair.as_str().to_string(),
                span: Some(to_span(pair.as_span())),
            }));
        }
        _ if is_usage_rule(pair.as_rule()) => {
            // Parse nested usage
            members.push(UsageMember::Usage(Box::new(parse_usage(pair.clone()))));
        }
        _ => {
            for inner in pair.clone().into_inner() {
                extract_usage_members(&inner, members);
            }
        }
    }
}

// ============================================================================
// Main parsers
// ============================================================================

/// Parse a definition from a pest pair
pub fn parse_definition(pair: Pair<Rule>) -> Result<Definition, ConversionError<Void>> {
    let kind = to_def_kind(pair.as_rule())?;

    // Recursively find the body rule (it may be nested in definition_suffix)
    let body = find_body_in_tree(&pair)
        .map(|p| parse_def_body(&p))
        .unwrap_or_default();

    // Find the identifier and its span
    let pairs: Vec<_> = pair.clone().into_inner().collect();
    let (name, span) = find_identifier_span(pairs.iter().cloned());
    let name = name.or_else(|| find_name(pairs.iter().cloned()));

    // Extract definition flags (abstract, variation)
    let (is_abstract, is_variation) = extract_definition_flags(&pairs);

    Ok(Definition {
        kind,
        name,
        relationships: extract_relationships(&pair),
        body,
        span,
        is_abstract,
        is_variation,
    })
}

fn find_body_in_tree<'i>(pair: &Pair<'i, Rule>) -> Option<Pair<'i, Rule>> {
    if is_body_rule(pair.as_rule()) {
        return Some(pair.clone());
    }
    for inner in pair.clone().into_inner() {
        if let Some(found) = find_body_in_tree(&inner) {
            return Some(found);
        }
    }
    None
}

/// Parse a usage from a pest pair
pub fn parse_usage(pair: Pair<Rule>) -> Usage {
    let kind = to_usage_kind(pair.as_rule()).unwrap();
    let pairs: Vec<_> = pair.clone().into_inner().collect();

    let body = pairs
        .iter()
        .find_map(|p| find_body_in_tree(p))
        .map(|p| parse_usage_body(&p))
        .unwrap_or_default();

    let (is_derived, is_readonly) = extract_flags(&pairs);

    // Find the identifier and its span
    let (name, span) = find_identifier_span(pairs.iter().cloned());
    let name = name.or_else(|| find_name(pairs.iter().cloned()));

    Usage {
        kind,
        name,
        relationships: extract_relationships(&pair),
        body,
        span,
        is_derived,
        is_readonly,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::sysml::SysMLParser;
    use pest::Parser;

    #[test]
    fn test_metadata_def_with_attributes() {
        let source = r#"metadata def ToolExecution {
            attribute toolName : String;
            attribute uri : String;
        }"#;

        let parsed = SysMLParser::parse(Rule::metadata_definition, source)
            .expect("Should parse")
            .next()
            .expect("Should have pair");

        let def = parse_definition(parsed).expect("Should convert to Definition");

        assert_eq!(def.name, Some("ToolExecution".to_string()));
        assert_eq!(
            def.body.len(),
            2,
            "Definition should have 2 attribute members"
        );

        // Check first attribute
        if let DefinitionMember::Usage(usage) = &def.body[0] {
            assert_eq!(usage.name, Some("toolName".to_string()));
            assert_eq!(usage.relationships.typed_by, Some("String".to_string()));
            assert!(
                usage.relationships.typed_by_span.is_some(),
                "Should have span for type reference"
            );
        } else {
            panic!("First member should be a Usage");
        }

        // Check second attribute
        if let DefinitionMember::Usage(usage) = &def.body[1] {
            assert_eq!(usage.name, Some("uri".to_string()));
            assert_eq!(usage.relationships.typed_by, Some("String".to_string()));
            assert!(
                usage.relationships.typed_by_span.is_some(),
                "Should have span for type reference"
            );
        } else {
            panic!("Second member should be a Usage");
        }
    }

    #[test]
    fn test_import_with_span() {
        use crate::syntax::SyntaxFile;
        use crate::syntax::parser::parse_content;
        use std::path::PathBuf;

        let source = r#"package TestPkg {
    private import ScalarValues::Real;
    private import Quantities::*;
}"#;

        let path = PathBuf::from("test.sysml");
        let syntax_file = parse_content(source, &path).expect("Parse should succeed");

        if let SyntaxFile::SysML(file) = syntax_file {
            // Find the package
            let pkg = file
                .elements
                .iter()
                .find_map(|e| match e {
                    crate::syntax::sysml::ast::enums::Element::Package(p) => Some(p),
                    _ => None,
                })
                .expect("Should have package");

            assert_eq!(pkg.name, Some("TestPkg".to_string()));

            // Find imports
            let imports: Vec<_> = pkg
                .elements
                .iter()
                .filter_map(|e| match e {
                    crate::syntax::sysml::ast::enums::Element::Import(imp) => Some(imp),
                    _ => None,
                })
                .collect();

            assert_eq!(imports.len(), 2, "Should have 2 imports");

            // Check first import
            assert_eq!(imports[0].path, "ScalarValues::Real");
            assert!(
                imports[0].span.is_some(),
                "First import should have span for 'ScalarValues::Real'"
            );
            if let Some(span) = &imports[0].span {
                // Span should point to "ScalarValues::Real" not the keywords
                assert_eq!(span.start.line, 1);
                assert!(
                    span.start.column >= 19,
                    "Should start after 'private import '"
                );
                assert!(span.end.column <= 37, "Should end at the path");
                println!("Import 1 span: {span:?}");
            }

            // Check second import
            assert_eq!(imports[1].path, "Quantities::*");
            assert!(
                imports[1].span.is_some(),
                "Second import should have span for 'Quantities::*'"
            );
            if let Some(span) = &imports[1].span {
                // Span should point to "Quantities::*" not the keywords
                assert_eq!(span.start.line, 2);
                assert!(
                    span.start.column >= 19,
                    "Should start after 'private import '"
                );
                println!("Import 2 span: {span:?}");
            }
        } else {
            panic!("Expected SysML file");
        }
    }

    #[test]
    fn test_attribute_usage_with_type_and_span() {
        use crate::syntax::SyntaxFile;
        use crate::syntax::parser::parse_content;
        use std::path::PathBuf;

        let source = r#"attribute def SoundPressureLevelValue;
attribute def SoundPressureLevelUnit;
attribute def DimensionOneUnit;

attribute soundPressureLevel: SoundPressureLevelValue[*] nonunique;
"#;

        let path = PathBuf::from("test.sysml");
        let syntax_file = parse_content(source, &path).expect("Parse should succeed");

        if let SyntaxFile::SysML(file) = syntax_file {
            // Find attribute usage (not definition)
            let usage = file
                .elements
                .iter()
                .find_map(|e| match e {
                    crate::syntax::sysml::ast::enums::Element::Usage(u)
                        if u.name == Some("soundPressureLevel".to_string()) =>
                    {
                        Some(u)
                    }
                    _ => None,
                })
                .expect("Should find soundPressureLevel attribute usage");

            // Check it has a name span
            assert!(
                usage.span.is_some(),
                "Attribute usage should have span for 'soundPressureLevel'"
            );
            if let Some(span) = &usage.span {
                println!("Attribute usage span: {span:?}");
            }

            // Check it has a type reference
            assert_eq!(
                usage.relationships.typed_by,
                Some("SoundPressureLevelValue".to_string())
            );
            assert!(
                usage.relationships.typed_by_span.is_some(),
                "Should have span for type 'SoundPressureLevelValue'"
            );
            if let Some(span) = &usage.relationships.typed_by_span {
                println!("Type reference span: {span:?}");
            }
        } else {
            panic!("Expected SysML file");
        }
    }
}
