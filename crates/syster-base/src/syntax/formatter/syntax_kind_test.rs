//! Tests for SyntaxKind and SysMLLanguage trait implementations

use super::syntax_kind::{SyntaxKind, SysMLLanguage};
use rowan::Language;

// ============================================================================
// Tests for kind_to_raw function
// ============================================================================

#[test]
fn test_kind_to_raw_trivia() {
    // Test trivia token conversions
    let raw = SysMLLanguage::kind_to_raw(SyntaxKind::Whitespace);
    assert_eq!(raw.0, SyntaxKind::Whitespace as u16);

    let raw = SysMLLanguage::kind_to_raw(SyntaxKind::LineComment);
    assert_eq!(raw.0, SyntaxKind::LineComment as u16);

    let raw = SysMLLanguage::kind_to_raw(SyntaxKind::BlockComment);
    assert_eq!(raw.0, SyntaxKind::BlockComment as u16);
}

#[test]
fn test_kind_to_raw_literals() {
    // Test literal token conversions
    let raw = SysMLLanguage::kind_to_raw(SyntaxKind::Identifier);
    assert_eq!(raw.0, SyntaxKind::Identifier as u16);

    let raw = SysMLLanguage::kind_to_raw(SyntaxKind::Number);
    assert_eq!(raw.0, SyntaxKind::Number as u16);

    let raw = SysMLLanguage::kind_to_raw(SyntaxKind::String);
    assert_eq!(raw.0, SyntaxKind::String as u16);
}

#[test]
fn test_kind_to_raw_punctuation() {
    // Test punctuation token conversions
    let raw = SysMLLanguage::kind_to_raw(SyntaxKind::LBrace);
    assert_eq!(raw.0, SyntaxKind::LBrace as u16);

    let raw = SysMLLanguage::kind_to_raw(SyntaxKind::RBrace);
    assert_eq!(raw.0, SyntaxKind::RBrace as u16);

    let raw = SysMLLanguage::kind_to_raw(SyntaxKind::Semicolon);
    assert_eq!(raw.0, SyntaxKind::Semicolon as u16);

    let raw = SysMLLanguage::kind_to_raw(SyntaxKind::ColonColon);
    assert_eq!(raw.0, SyntaxKind::ColonColon as u16);

    let raw = SysMLLanguage::kind_to_raw(SyntaxKind::Arrow);
    assert_eq!(raw.0, SyntaxKind::Arrow as u16);
}

#[test]
fn test_kind_to_raw_sysml_keywords() {
    // Test SysML keyword conversions
    let raw = SysMLLanguage::kind_to_raw(SyntaxKind::PackageKw);
    assert_eq!(raw.0, SyntaxKind::PackageKw as u16);

    let raw = SysMLLanguage::kind_to_raw(SyntaxKind::PartKw);
    assert_eq!(raw.0, SyntaxKind::PartKw as u16);

    let raw = SysMLLanguage::kind_to_raw(SyntaxKind::DefKw);
    assert_eq!(raw.0, SyntaxKind::DefKw as u16);

    let raw = SysMLLanguage::kind_to_raw(SyntaxKind::ImportKw);
    assert_eq!(raw.0, SyntaxKind::ImportKw as u16);

    let raw = SysMLLanguage::kind_to_raw(SyntaxKind::AttributeKw);
    assert_eq!(raw.0, SyntaxKind::AttributeKw as u16);

    let raw = SysMLLanguage::kind_to_raw(SyntaxKind::RequirementKw);
    assert_eq!(raw.0, SyntaxKind::RequirementKw as u16);

    let raw = SysMLLanguage::kind_to_raw(SyntaxKind::ConstraintKw);
    assert_eq!(raw.0, SyntaxKind::ConstraintKw as u16);
}

#[test]
fn test_kind_to_raw_kerml_keywords() {
    // Test KerML keyword conversions
    let raw = SysMLLanguage::kind_to_raw(SyntaxKind::StructKw);
    assert_eq!(raw.0, SyntaxKind::StructKw as u16);

    let raw = SysMLLanguage::kind_to_raw(SyntaxKind::ClassKw);
    assert_eq!(raw.0, SyntaxKind::ClassKw as u16);

    let raw = SysMLLanguage::kind_to_raw(SyntaxKind::DataTypeKw);
    assert_eq!(raw.0, SyntaxKind::DataTypeKw as u16);

    let raw = SysMLLanguage::kind_to_raw(SyntaxKind::FunctionKw);
    assert_eq!(raw.0, SyntaxKind::FunctionKw as u16);
}

#[test]
fn test_kind_to_raw_composite_nodes() {
    // Test composite node conversions
    let raw = SysMLLanguage::kind_to_raw(SyntaxKind::SourceFile);
    assert_eq!(raw.0, SyntaxKind::SourceFile as u16);

    let raw = SysMLLanguage::kind_to_raw(SyntaxKind::Package);
    assert_eq!(raw.0, SyntaxKind::Package as u16);

    let raw = SysMLLanguage::kind_to_raw(SyntaxKind::Definition);
    assert_eq!(raw.0, SyntaxKind::Definition as u16);

    let raw = SysMLLanguage::kind_to_raw(SyntaxKind::Usage);
    assert_eq!(raw.0, SyntaxKind::Usage as u16);

    let raw = SysMLLanguage::kind_to_raw(SyntaxKind::Import);
    assert_eq!(raw.0, SyntaxKind::Import as u16);

    let raw = SysMLLanguage::kind_to_raw(SyntaxKind::Body);
    assert_eq!(raw.0, SyntaxKind::Body as u16);
}

#[test]
fn test_kind_to_raw_special() {
    // Test special token conversions
    let raw = SysMLLanguage::kind_to_raw(SyntaxKind::Error);
    assert_eq!(raw.0, SyntaxKind::Error as u16);

    let raw = SysMLLanguage::kind_to_raw(SyntaxKind::Eof);
    assert_eq!(raw.0, SyntaxKind::Eof as u16);
}

// ============================================================================
// Round-trip conversion tests (kind_to_raw -> kind_from_raw)
// ============================================================================

#[test]
fn test_round_trip_conversion_trivia() {
    // Test round-trip conversions for trivia
    let original = SyntaxKind::Whitespace;
    let raw = SysMLLanguage::kind_to_raw(original);
    let recovered = SysMLLanguage::kind_from_raw(raw);
    assert_eq!(original, recovered);

    let original = SyntaxKind::LineComment;
    let raw = SysMLLanguage::kind_to_raw(original);
    let recovered = SysMLLanguage::kind_from_raw(raw);
    assert_eq!(original, recovered);

    let original = SyntaxKind::BlockComment;
    let raw = SysMLLanguage::kind_to_raw(original);
    let recovered = SysMLLanguage::kind_from_raw(raw);
    assert_eq!(original, recovered);
}

#[test]
fn test_round_trip_conversion_literals() {
    let original = SyntaxKind::Identifier;
    let raw = SysMLLanguage::kind_to_raw(original);
    let recovered = SysMLLanguage::kind_from_raw(raw);
    assert_eq!(original, recovered);

    let original = SyntaxKind::Number;
    let raw = SysMLLanguage::kind_to_raw(original);
    let recovered = SysMLLanguage::kind_from_raw(raw);
    assert_eq!(original, recovered);

    let original = SyntaxKind::String;
    let raw = SysMLLanguage::kind_to_raw(original);
    let recovered = SysMLLanguage::kind_from_raw(raw);
    assert_eq!(original, recovered);
}

#[test]
fn test_round_trip_conversion_punctuation() {
    let punctuation_kinds = [
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
        SyntaxKind::Arrow,
    ];

    for original in punctuation_kinds {
        let raw = SysMLLanguage::kind_to_raw(original);
        let recovered = SysMLLanguage::kind_from_raw(raw);
        assert_eq!(
            original, recovered,
            "Round-trip failed for punctuation: {:?}",
            original
        );
    }
}

#[test]
fn test_round_trip_conversion_sysml_keywords() {
    let keywords = [
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
    ];

    for original in keywords {
        let raw = SysMLLanguage::kind_to_raw(original);
        let recovered = SysMLLanguage::kind_from_raw(raw);
        assert_eq!(
            original, recovered,
            "Round-trip failed for SysML keyword: {:?}",
            original
        );
    }
}

#[test]
fn test_round_trip_conversion_kerml_keywords() {
    let keywords = [
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
    ];

    for original in keywords {
        let raw = SysMLLanguage::kind_to_raw(original);
        let recovered = SysMLLanguage::kind_from_raw(raw);
        assert_eq!(
            original, recovered,
            "Round-trip failed for KerML keyword: {:?}",
            original
        );
    }
}

#[test]
fn test_round_trip_conversion_composite_nodes() {
    let nodes = [
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
    ];

    for original in nodes {
        let raw = SysMLLanguage::kind_to_raw(original);
        let recovered = SysMLLanguage::kind_from_raw(raw);
        assert_eq!(
            original, recovered,
            "Round-trip failed for composite node: {:?}",
            original
        );
    }
}

#[test]
fn test_round_trip_conversion_special() {
    let original = SyntaxKind::Error;
    let raw = SysMLLanguage::kind_to_raw(original);
    let recovered = SysMLLanguage::kind_from_raw(raw);
    assert_eq!(original, recovered);

    let original = SyntaxKind::Eof;
    let raw = SysMLLanguage::kind_to_raw(original);
    let recovered = SysMLLanguage::kind_from_raw(raw);
    assert_eq!(original, recovered);
}

// ============================================================================
// Edge case tests
// ============================================================================

#[test]
fn test_kind_to_raw_first_variant() {
    // Test the first enum variant (Whitespace = 0)
    let raw = SysMLLanguage::kind_to_raw(SyntaxKind::Whitespace);
    assert_eq!(raw.0, 0);
}

#[test]
fn test_kind_to_raw_preserves_ordering() {
    // Verify that the numeric values increase as expected
    let whitespace_raw = SysMLLanguage::kind_to_raw(SyntaxKind::Whitespace);
    let line_comment_raw = SysMLLanguage::kind_to_raw(SyntaxKind::LineComment);
    let block_comment_raw = SysMLLanguage::kind_to_raw(SyntaxKind::BlockComment);

    assert!(whitespace_raw.0 < line_comment_raw.0);
    assert!(line_comment_raw.0 < block_comment_raw.0);
}

#[test]
fn test_kind_to_raw_all_distinct() {
    // Sample check that different kinds produce different raw values
    let kinds = [
        SyntaxKind::Whitespace,
        SyntaxKind::Identifier,
        SyntaxKind::PackageKw,
        SyntaxKind::PartKw,
        SyntaxKind::SourceFile,
        SyntaxKind::Error,
    ];

    let mut raw_values = std::collections::HashSet::new();
    for kind in kinds {
        let raw = SysMLLanguage::kind_to_raw(kind);
        assert!(
            raw_values.insert(raw.0),
            "Duplicate raw value for kind: {:?}",
            kind
        );
    }
}

// ============================================================================
// Comprehensive round-trip test for all variants
// ============================================================================

#[test]
fn test_round_trip_all_syntax_kinds() {
    // Test ALL syntax kinds to ensure complete coverage
    let all_kinds = [
        // Trivia
        SyntaxKind::Whitespace,
        SyntaxKind::LineComment,
        SyntaxKind::BlockComment,
        // Literals
        SyntaxKind::Identifier,
        SyntaxKind::Number,
        SyntaxKind::String,
        // Punctuation
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
        // SysML Keywords
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
        SyntaxKind::AbstractKw,
        SyntaxKind::RefKw,
        SyntaxKind::ReadonlyKw,
        SyntaxKind::DerivedKw,
        SyntaxKind::EndKw,
        SyntaxKind::InKw,
        SyntaxKind::OutKw,
        SyntaxKind::InoutKw,
        SyntaxKind::AliasKw,
        SyntaxKind::DocKw,
        SyntaxKind::CommentKw,
        SyntaxKind::AboutKw,
        SyntaxKind::RepKw,
        SyntaxKind::LanguageKw,
        SyntaxKind::SpecializesKw,
        SyntaxKind::SubsetsKw,
        SyntaxKind::RedefinesKw,
        SyntaxKind::TypedByKw,
        SyntaxKind::ReferencesKw,
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
        SyntaxKind::IfKw,
        SyntaxKind::ElseKw,
        SyntaxKind::ThenKw,
        SyntaxKind::LoopKw,
        SyntaxKind::WhileKw,
        SyntaxKind::UntilKw,
        SyntaxKind::ForKw,
        SyntaxKind::ForkKw,
        SyntaxKind::JoinKw,
        SyntaxKind::MergeKw,
        SyntaxKind::DecideKw,
        SyntaxKind::AcceptKw,
        SyntaxKind::SendKw,
        SyntaxKind::ViaKw,
        SyntaxKind::ToKw,
        SyntaxKind::FromKw,
        SyntaxKind::DependencyKw,
        SyntaxKind::FilterKw,
        SyntaxKind::ExposeKw,
        SyntaxKind::AllKw,
        SyntaxKind::FirstKw,
        SyntaxKind::ModelKw,
        SyntaxKind::LibraryKw,
        SyntaxKind::StandardKw,
        SyntaxKind::PrivateKw,
        SyntaxKind::ProtectedKw,
        SyntaxKind::PublicKw,
        SyntaxKind::TrueKw,
        SyntaxKind::FalseKw,
        SyntaxKind::NullKw,
        SyntaxKind::AndKw,
        SyntaxKind::OrKw,
        SyntaxKind::NotKw,
        SyntaxKind::XorKw,
        SyntaxKind::ImpliesKw,
        SyntaxKind::HasTypeKw,
        SyntaxKind::IsTypeKw,
        SyntaxKind::AsKw,
        SyntaxKind::MetaKw,
        // KerML Keywords
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
        // Composite Nodes
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
        // Special
        SyntaxKind::Error,
        SyntaxKind::Eof,
    ];

    for original in all_kinds {
        let raw = SysMLLanguage::kind_to_raw(original);
        let recovered = SysMLLanguage::kind_from_raw(raw);
        assert_eq!(
            original, recovered,
            "Round-trip conversion failed for: {:?}",
            original
        );
    }
}
