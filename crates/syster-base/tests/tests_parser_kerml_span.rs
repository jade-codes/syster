#![allow(clippy::unwrap_used)]

//! Tests for KerML source position tracking (span) in AST nodes
//!
//! These tests verify that the KerML parser correctly captures source locations
//! for package identifiers, which is essential for semantic token highlighting.
//!
//! Bug fix: Package span was incorrectly pointing to "standard" or "library"
//! keywords instead of the package name identifier.

use std::path::PathBuf;
use syster::syntax::SyntaxFile;
use syster::syntax::kerml::ast::KerMLFile;
use syster::syntax::parser::parse_with_result;

/// Helper to parse KerML content and extract the KerMLFile
fn parse_kerml(source: &str) -> KerMLFile {
    let path = PathBuf::from("test.kerml");
    let parse_result = parse_with_result(source, &path);
    let language_file = parse_result.content.expect("Parse should succeed");
    match language_file {
        SyntaxFile::KerML(file) => file,
        _ => panic!("Expected KerML file"),
    }
}

#[test]
fn test_kerml_package_span_points_to_identifier() {
    // Simple package - span should point to "MyPackage"
    let source = "package MyPackage;";
    //            01234567890123456
    //                    ^^^^^^^^^  columns 8-17

    let file = parse_kerml(source);

    assert!(!file.elements.is_empty() || file.namespace.is_some());

    // Get the package (either as namespace or element)
    let span = if let Some(ref ns) = file.namespace {
        ns.span.expect("Package should have span")
    } else {
        let syster::syntax::kerml::ast::Element::Package(ref pkg) = file.elements[0] else {
            panic!("Expected Package");
        };
        pkg.span.expect("Package should have span")
    };

    // Span should capture "MyPackage" at columns 8-17
    assert_eq!(span.start.line, 0, "Should be on line 0");
    assert_eq!(
        span.start.column, 8,
        "Should start at column 8 (after 'package ')"
    );
    assert_eq!(span.end.line, 0, "Should end on line 0");
    assert_eq!(span.end.column, 17, "Should end at column 17");
}

#[test]
fn test_kerml_library_package_span_points_to_identifier_not_library() {
    // Library package - span should point to "MyLibrary", NOT "library"
    let source = "library package MyLibrary;";
    //            0123456789012345678901234
    //                            ^^^^^^^^^  columns 16-25

    let file = parse_kerml(source);

    let span = if let Some(ref ns) = file.namespace {
        ns.span.expect("Package should have span")
    } else {
        let syster::syntax::kerml::ast::Element::Package(ref pkg) = file.elements[0] else {
            panic!("Expected Package");
        };
        pkg.span.expect("Package should have span")
    };

    // Span should capture "MyLibrary" at columns 16-25, NOT "library" at columns 0-7
    assert_eq!(span.start.line, 0, "Should be on line 0");
    assert_eq!(
        span.start.column, 16,
        "Should start at column 16 (after 'library package '), not at 0 where 'library' starts"
    );
    assert_eq!(span.end.line, 0, "Should end on line 0");
    assert_eq!(span.end.column, 25, "Should end at column 25");
}

#[test]
fn test_kerml_standard_library_package_span_points_to_identifier_not_standard() {
    // Standard library package - span should point to "Base", NOT "standard"
    let source = "standard library package Base;";
    //            0         1         2
    //            0123456789012345678901234567890
    //                                     ^^^^  columns 25-29

    let file = parse_kerml(source);

    let span = if let Some(ref ns) = file.namespace {
        ns.span.expect("Package should have span")
    } else {
        let syster::syntax::kerml::ast::Element::Package(ref pkg) = file.elements[0] else {
            panic!("Expected Package");
        };
        pkg.span.expect("Package should have span")
    };

    // Span should capture "Base" NOT "standard" at columns 0-8
    assert_eq!(span.start.line, 0, "Should be on line 0");
    assert!(
        span.start.column >= 24,
        "Should start after 'standard library package ', not at 0 where 'standard' starts. Got column {}",
        span.start.column
    );
}
