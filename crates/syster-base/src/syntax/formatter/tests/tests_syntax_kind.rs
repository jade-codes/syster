#![allow(clippy::unwrap_used)]

use crate::syntax::formatter::syntax_kind::{SyntaxKind, SysMLLanguage};
use crate::syntax::formatter::{FormatOptions, format_async};
use tokio_util::sync::CancellationToken;

// ============================================================================
// Tests for SysMLLanguage::kind_to_raw and kind_from_raw (#532, #533)
// ============================================================================
// These tests verify the trait implementation of rowan::Language for SysMLLanguage.
// We test both the conversion functions and their usage through the formatter API.

#[test]
fn test_kind_to_raw_via_formatter_simple_package() {
    // Tests that kind_to_raw correctly converts PackageKw, LBrace, RBrace, etc.
    let source = "package Test { }";
    let result = format_async(source, &FormatOptions::default(), &CancellationToken::new());
    assert!(result.is_some());
    assert!(result.unwrap().contains("package"));
}

#[test]
fn test_kind_to_raw_via_formatter_keywords() {
    // Tests that kind_to_raw handles various SysML keywords
    let source = "part def MyPart { }";
    let result = format_async(source, &FormatOptions::default(), &CancellationToken::new());
    assert!(result.is_some());
    let output = result.unwrap();
    assert!(output.contains("part"));
    assert!(output.contains("def"));
}

#[test]
fn test_kind_to_raw_via_formatter_punctuation() {
    // Tests that kind_to_raw handles punctuation tokens
    let source = "package A::B { }";
    let result = format_async(source, &FormatOptions::default(), &CancellationToken::new());
    assert!(result.is_some());
    assert!(result.unwrap().contains("::"));
}

#[test]
fn test_kind_to_raw_via_formatter_comments() {
    // Tests that kind_to_raw handles comment tokens
    let source = "// Comment\npackage Test { }";
    let result = format_async(source, &FormatOptions::default(), &CancellationToken::new());
    assert!(result.is_some());
    assert!(result.unwrap().contains("// Comment"));
}

#[test]
fn test_kind_to_raw_via_formatter_import() {
    // Tests that kind_to_raw handles import statements
    let source = "import Package::*;";
    let result = format_async(source, &FormatOptions::default(), &CancellationToken::new());
    assert!(result.is_some());
    assert!(result.unwrap().contains("import"));
}

#[test]
fn test_kind_to_raw_via_formatter_with_cancellation() {
    // Tests that the formatter (which uses kind_to_raw) respects cancellation
    let source = "package Test { }";
    let cancel = CancellationToken::new();
    cancel.cancel();
    let result = format_async(source, &FormatOptions::default(), &cancel);
    assert!(result.is_none());
}

// ============================================================================
// Direct tests for kind_to_raw and kind_from_raw (#532, #533)
// ============================================================================

/// Helper function to test round-trip conversion for multiple SyntaxKind variants
fn assert_roundtrip_conversion(kinds: &[SyntaxKind]) {
    for kind in kinds {
        let raw = <SysMLLanguage as rowan::Language>::kind_to_raw(*kind);
        let back = <SysMLLanguage as rowan::Language>::kind_from_raw(raw);
        assert_eq!(*kind, back, "Round-trip failed for {:?}", kind);
    }
}

/// Test round-trip conversion for trivia tokens
#[test]
fn test_roundtrip_trivia_tokens() {
    assert_roundtrip_conversion(&[
        SyntaxKind::Whitespace,
        SyntaxKind::LineComment,
        SyntaxKind::BlockComment,
    ]);
}

/// Test round-trip conversion for literal tokens
#[test]
fn test_roundtrip_literal_tokens() {
    assert_roundtrip_conversion(&[
        SyntaxKind::Identifier,
        SyntaxKind::Number,
        SyntaxKind::String,
    ]);
}

/// Test round-trip conversion for punctuation tokens
#[test]
fn test_roundtrip_punctuation_tokens() {
    assert_roundtrip_conversion(&[
        SyntaxKind::LBrace,
        SyntaxKind::RBrace,
        SyntaxKind::LBracket,
        SyntaxKind::RBracket,
        SyntaxKind::LParen,
        SyntaxKind::RParen,
        SyntaxKind::Semicolon,
        SyntaxKind::Colon,
        SyntaxKind::ColonColon,
        SyntaxKind::Dot,
        SyntaxKind::Comma,
        SyntaxKind::Eq,
        SyntaxKind::EqEq,
        SyntaxKind::NotEq,
        SyntaxKind::Lt,
        SyntaxKind::Gt,
        SyntaxKind::LtEq,
        SyntaxKind::GtEq,
        SyntaxKind::Arrow,
        SyntaxKind::At,
        SyntaxKind::Star,
        SyntaxKind::Plus,
        SyntaxKind::Minus,
        SyntaxKind::Slash,
        SyntaxKind::Percent,
        SyntaxKind::Caret,
        SyntaxKind::Tilde,
        SyntaxKind::Question,
        SyntaxKind::Bang,
        SyntaxKind::Pipe,
        SyntaxKind::Ampersand,
        SyntaxKind::Hash,
    ]);
}

/// Test round-trip conversion for common SysML keywords
#[test]
fn test_roundtrip_sysml_keywords() {
    assert_roundtrip_conversion(&[
        SyntaxKind::PackageKw,
        SyntaxKind::PartKw,
        SyntaxKind::DefKw,
        SyntaxKind::ImportKw,
        SyntaxKind::AttributeKw,
        SyntaxKind::PortKw,
        SyntaxKind::ItemKw,
        SyntaxKind::ActionKw,
        SyntaxKind::StateKw,
        SyntaxKind::RequirementKw,
        SyntaxKind::ConstraintKw,
        SyntaxKind::ConnectionKw,
        SyntaxKind::AllocationKw,
        SyntaxKind::InterfaceKw,
        SyntaxKind::FlowKw,
        SyntaxKind::UseCaseKw,
        SyntaxKind::ViewKw,
        SyntaxKind::ViewpointKw,
        SyntaxKind::RenderingKw,
        SyntaxKind::MetadataKw,
        SyntaxKind::OccurrenceKw,
        SyntaxKind::AnalysisKw,
        SyntaxKind::VerificationKw,
        SyntaxKind::ConcernKw,
        SyntaxKind::EnumKw,
        SyntaxKind::CalcKw,
        SyntaxKind::CaseKw,
        SyntaxKind::IndividualKw,
        SyntaxKind::EndKw,
    ]);
}

/// Test round-trip conversion for SysML modifier keywords
#[test]
fn test_roundtrip_sysml_modifier_keywords() {
    assert_roundtrip_conversion(&[
        SyntaxKind::AbstractKw,
        SyntaxKind::RefKw,
        SyntaxKind::ReadonlyKw,
        SyntaxKind::DerivedKw,
        SyntaxKind::InKw,
        SyntaxKind::OutKw,
        SyntaxKind::InoutKw,
        SyntaxKind::PrivateKw,
        SyntaxKind::ProtectedKw,
        SyntaxKind::PublicKw,
    ]);
}

/// Test round-trip conversion for SysML relationship keywords
#[test]
fn test_roundtrip_sysml_relationship_keywords() {
    assert_roundtrip_conversion(&[
        SyntaxKind::SpecializesKw,
        SyntaxKind::SubsetsKw,
        SyntaxKind::RedefinesKw,
        SyntaxKind::TypedByKw,
        SyntaxKind::ReferencesKw,
    ]);
}

/// Test round-trip conversion for SysML action and behavior keywords
#[test]
fn test_roundtrip_sysml_action_behavior_keywords() {
    assert_roundtrip_conversion(&[
        SyntaxKind::AssertKw,
        SyntaxKind::AssumeKw,
        SyntaxKind::RequireKw,
        SyntaxKind::PerformKw,
        SyntaxKind::ExhibitKw,
        SyntaxKind::IncludeKw,
        SyntaxKind::SatisfyKw,
        SyntaxKind::EntryKw,
        SyntaxKind::ExitKw,
        SyntaxKind::DoKw,
        SyntaxKind::ForkKw,
        SyntaxKind::JoinKw,
        SyntaxKind::MergeKw,
        SyntaxKind::DecideKw,
        SyntaxKind::AcceptKw,
        SyntaxKind::SendKw,
    ]);
}

/// Test round-trip conversion for SysML connection and reference keywords
#[test]
fn test_roundtrip_sysml_connection_reference_keywords() {
    assert_roundtrip_conversion(&[
        SyntaxKind::ViaKw,
        SyntaxKind::ToKw,
        SyntaxKind::FromKw,
        SyntaxKind::DependencyKw,
        SyntaxKind::FilterKw,
        SyntaxKind::ExposeKw,
        SyntaxKind::AllKw,
        SyntaxKind::FirstKw,
        SyntaxKind::HasTypeKw,
        SyntaxKind::IsTypeKw,
        SyntaxKind::AsKw,
        SyntaxKind::MetaKw,
    ]);
}

/// Test round-trip conversion for KerML keywords
#[test]
fn test_roundtrip_kerml_keywords() {
    assert_roundtrip_conversion(&[
        SyntaxKind::StructKw,
        SyntaxKind::ClassKw,
        SyntaxKind::DataTypeKw,
        SyntaxKind::AssocKw,
        SyntaxKind::BehaviorKw,
        SyntaxKind::FunctionKw,
        SyntaxKind::TypeKw,
        SyntaxKind::FeatureKw,
        SyntaxKind::StepKw,
        SyntaxKind::ExprKw,
        SyntaxKind::BindingKw,
        SyntaxKind::SuccessionKw,
        SyntaxKind::ConnectorKw,
        SyntaxKind::InvKw,
        SyntaxKind::NonuniqueKw,
        SyntaxKind::OrderedKw,
        SyntaxKind::UnorderedKw,
    ]);
}

/// Test round-trip conversion for composite node kinds
#[test]
fn test_roundtrip_composite_nodes() {
    assert_roundtrip_conversion(&[
        SyntaxKind::SourceFile,
        SyntaxKind::Package,
        SyntaxKind::Definition,
        SyntaxKind::Usage,
        SyntaxKind::Import,
        SyntaxKind::Alias,
        SyntaxKind::Annotation,
        SyntaxKind::Name,
        SyntaxKind::Body,
        SyntaxKind::Relationship,
    ]);
}

/// Test round-trip conversion for special tokens
#[test]
fn test_roundtrip_special_tokens() {
    assert_roundtrip_conversion(&[SyntaxKind::Error, SyntaxKind::Eof]);
}

/// Test that kind_to_raw produces unique raw values for different kinds
#[test]
fn test_kind_to_raw_uniqueness() {
    let kinds = [
        SyntaxKind::Whitespace,
        SyntaxKind::PackageKw,
        SyntaxKind::PartKw,
        SyntaxKind::DefKw,
        SyntaxKind::LBrace,
        SyntaxKind::RBrace,
        SyntaxKind::Identifier,
        SyntaxKind::SourceFile,
        SyntaxKind::Error,
        SyntaxKind::Eof,
    ];

    let mut raw_values = std::collections::HashSet::new();
    for kind in kinds {
        let raw = <SysMLLanguage as rowan::Language>::kind_to_raw(kind);
        assert!(
            raw_values.insert(raw.0),
            "Duplicate raw value {} for {:?}",
            raw.0,
            kind
        );
    }
}

/// Test boundary values - first and last enum variants
#[test]
fn test_roundtrip_boundary_values() {
    assert_roundtrip_conversion(&[
        SyntaxKind::Whitespace, // First variant (0)
        SyntaxKind::Eof,        // Last variant
    ]);
}

/// Test that raw values preserve the numeric representation
#[test]
fn test_kind_to_raw_numeric_value() {
    // Test that the numeric value is preserved correctly
    let kind = SyntaxKind::Whitespace;
    let raw = <SysMLLanguage as rowan::Language>::kind_to_raw(kind);
    assert_eq!(
        raw.0, kind as u16,
        "Raw value should match enum discriminant"
    );

    let kind = SyntaxKind::PackageKw;
    let raw = <SysMLLanguage as rowan::Language>::kind_to_raw(kind);
    assert_eq!(
        raw.0, kind as u16,
        "Raw value should match enum discriminant"
    );
}

/// Test round-trip with boolean and control flow keywords
#[test]
fn test_roundtrip_boolean_control_keywords() {
    assert_roundtrip_conversion(&[
        SyntaxKind::TrueKw,
        SyntaxKind::FalseKw,
        SyntaxKind::NullKw,
        SyntaxKind::AndKw,
        SyntaxKind::OrKw,
        SyntaxKind::NotKw,
        SyntaxKind::XorKw,
        SyntaxKind::ImpliesKw,
        SyntaxKind::IfKw,
        SyntaxKind::ElseKw,
        SyntaxKind::ThenKw,
        SyntaxKind::LoopKw,
        SyntaxKind::WhileKw,
        SyntaxKind::UntilKw,
        SyntaxKind::ForKw,
    ]);
}

/// Test round-trip for documentation and metadata keywords
#[test]
fn test_roundtrip_documentation_keywords() {
    assert_roundtrip_conversion(&[
        SyntaxKind::DocKw,
        SyntaxKind::CommentKw,
        SyntaxKind::AboutKw,
        SyntaxKind::RepKw,
        SyntaxKind::LanguageKw,
        SyntaxKind::AliasKw,
        SyntaxKind::ModelKw,
        SyntaxKind::LibraryKw,
        SyntaxKind::StandardKw,
    ]);
}
