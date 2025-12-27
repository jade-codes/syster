//! Lexer for SysML/KerML using Logos
//!
//! Tokenizes source code into tokens including whitespace and comments.

use super::syntax_kind::SyntaxKind;
use logos::Logos;

/// Token with text and kind
pub struct Token<'a> {
    pub kind: SyntaxKind,
    pub text: &'a str,
}

/// Logos-based token types
#[derive(Logos, Debug, Clone, Copy, PartialEq)]
pub enum LogosToken {
    // === Trivia ===
    #[regex(r"[ \t\r\n]+")]
    Whitespace,

    #[regex(r"//[^\n]*", priority = 2, allow_greedy = true)]
    LineComment,

    #[regex(r"/\*([^*]|\*[^/])*\*/")]
    BlockComment,

    // === Keywords (longest first for proper matching) ===
    #[token("specializes")]
    SpecializesKw,
    #[token("verification")]
    VerificationKw,
    #[token("requirement")]
    RequirementKw,
    #[token("connection")]
    ConnectionKw,
    #[token("allocation")]
    AllocationKw,
    #[token("constraint")]
    ConstraintKw,
    #[token("occurrence")]
    OccurrenceKw,
    #[token("individual")]
    IndividualKw,
    #[token("viewpoint")]
    ViewpointKw,
    #[token("rendering")]
    RenderingKw,
    #[token("interface")]
    InterfaceKw,
    #[token("redefines")]
    RedefinesKw,
    #[token("references")]
    ReferencesKw,
    #[token("metadata")]
    MetadataKw,
    #[token("analysis")]
    AnalysisKw,
    #[token("attribute")]
    AttributeKw,
    #[token("abstract")]
    AbstractKw,
    #[token("readonly")]
    ReadonlyKw,
    #[token("behavior")]
    BehaviorKw,
    #[token("function")]
    FunctionKw,
    #[token("datatype")]
    DataTypeKw,
    #[token("language")]
    LanguageKw,
    #[token("standard")]
    StandardKw,
    #[token("protected")]
    ProtectedKw,
    #[token("nonunique")]
    NonuniqueKw,
    #[token("unordered")]
    UnorderedKw,
    #[token("succession")]
    SuccessionKw,
    #[token("connector")]
    ConnectorKw,
    #[token("use case")]
    UseCaseKw,
    #[token("typed by")]
    TypedByKw,
    #[token("has type")]
    HasTypeKw,
    #[token("is type")]
    IsTypeKw,
    #[token("package")]
    PackageKw,
    #[token("private")]
    PrivateKw,
    #[token("concern")]
    ConcernKw,
    #[token("binding")]
    BindingKw,
    #[token("ordered")]
    OrderedKw,
    #[token("implies")]
    ImpliesKw,
    #[token("satisfy")]
    SatisfyKw,
    #[token("library")]
    LibraryKw,
    #[token("include")]
    IncludeKw,
    #[token("exhibit")]
    ExhibitKw,
    #[token("perform")]
    PerformKw,
    #[token("require")]
    RequireKw,
    #[token("subsets")]
    SubsetsKw,
    #[token("derived")]
    DerivedKw,
    #[token("feature")]
    FeatureKw,
    #[token("comment")]
    CommentKw,
    #[token("strucut")]
    StructKw,
    #[token("public")]
    PublicKw,
    #[token("expose")]
    ExposeKw,
    #[token("filter")]
    FilterKw,
    #[token("decide")]
    DecideKw,
    #[token("accept")]
    AcceptKw,
    #[token("import")]
    ImportKw,
    #[token("action")]
    ActionKw,
    #[token("assert")]
    AssertKw,
    #[token("assume")]
    AssumeKw,
    #[token("class")]
    ClassKw,
    #[token("assoc")]
    AssocKw,
    #[token("state")]
    StateKw,
    #[token("about")]
    AboutKw,
    #[token("entry")]
    EntryKw,
    #[token("until")]
    UntilKw,
    #[token("while")]
    WhileKw,
    #[token("model")]
    ModelKw,
    #[token("first")]
    FirstKw,
    #[token("alias")]
    AliasKw,
    #[token("inout")]
    InoutKw,
    #[token("false")]
    FalseKw,
    #[token("merge")]
    MergeKw,
    #[token("part")]
    PartKw,
    #[token("port")]
    PortKw,
    #[token("item")]
    ItemKw,
    #[token("view")]
    ViewKw,
    #[token("flow")]
    FlowKw,
    #[token("enum")]
    EnumKw,
    #[token("calc")]
    CalcKw,
    #[token("case")]
    CaseKw,
    #[token("type")]
    TypeKw,
    #[token("step")]
    StepKw,
    #[token("expr")]
    ExprKw,
    #[token("else")]
    ElseKw,
    #[token("then")]
    ThenKw,
    #[token("loop")]
    LoopKw,
    #[token("fork")]
    ForkKw,
    #[token("join")]
    JoinKw,
    #[token("send")]
    SendKw,
    #[token("from")]
    FromKw,
    #[token("null")]
    NullKw,
    #[token("true")]
    TrueKw,
    #[token("meta")]
    MetaKw,
    #[token("exit")]
    ExitKw,
    #[token("def")]
    DefKw,
    #[token("ref")]
    RefKw,
    #[token("doc")]
    DocKw,
    #[token("rep")]
    RepKw,
    #[token("inv")]
    InvKw,
    #[token("end")]
    EndKw,
    #[token("out")]
    OutKw,
    #[token("for")]
    ForKw,
    #[token("via")]
    ViaKw,
    #[token("all")]
    AllKw,
    #[token("and")]
    AndKw,
    #[token("xor")]
    XorKw,
    #[token("not")]
    NotKw,
    #[token("if")]
    IfKw,
    #[token("in")]
    InKw,
    #[token("do")]
    DoKw,
    #[token("to")]
    ToKw,
    #[token("or")]
    OrKw,
    #[token("as")]
    AsKw,
    #[token("dependency")]
    DependencyKw,

    // === Punctuation ===
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token(";")]
    Semicolon,
    #[token("::")]
    ColonColon,
    #[token(":")]
    Colon,
    #[token("..")]
    DotDot,
    #[token(".")]
    Dot,
    #[token(",")]
    Comma,
    #[token("==")]
    EqEq,
    #[token("!=")]
    NotEq,
    #[token("=")]
    Eq,
    #[token("<=")]
    LtEq,
    #[token(">=")]
    GtEq,
    #[token("<")]
    Lt,
    #[token(">")]
    Gt,
    #[token("->")]
    Arrow,
    #[token(":>>")]
    RedefinesToken,
    #[token(":>")]
    SubsetsToken,
    #[token("@")]
    At,
    #[token("**")]
    StarStar,
    #[token("*")]
    Star,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("/")]
    Slash,
    #[token("%")]
    Percent,
    #[token("^")]
    Caret,
    #[token("~")]
    Tilde,
    #[token("??")]
    QuestionQuestion,
    #[token("?")]
    Question,
    #[token("!")]
    Bang,
    #[token("||")]
    PipePipe,
    #[token("|")]
    Pipe,
    #[token("&&")]
    AmpAmp,
    #[token("&")]
    Ampersand,
    #[token("#")]
    Hash,

    // === Literals ===
    #[regex(r"[0-9]+(\.[0-9]+)?([eE][+-]?[0-9]+)?")]
    Number,

    #[regex(r#""([^"\\]|\\.)*""#)]
    #[regex(r#"'([^'\\]|\\.)*'"#)]
    String,

    // === Identifiers (must come after keywords) ===
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,
}

/// Convert Logos token to SyntaxKind
fn to_syntax_kind(token: LogosToken) -> SyntaxKind {
    match token {
        LogosToken::Whitespace => SyntaxKind::Whitespace,
        LogosToken::LineComment => SyntaxKind::LineComment,
        LogosToken::BlockComment => SyntaxKind::BlockComment,
        LogosToken::PackageKw => SyntaxKind::PackageKw,
        LogosToken::PartKw => SyntaxKind::PartKw,
        LogosToken::DefKw => SyntaxKind::DefKw,
        LogosToken::ImportKw => SyntaxKind::ImportKw,
        LogosToken::AttributeKw => SyntaxKind::AttributeKw,
        LogosToken::PortKw => SyntaxKind::PortKw,
        LogosToken::ItemKw => SyntaxKind::ItemKw,
        LogosToken::ActionKw => SyntaxKind::ActionKw,
        LogosToken::StateKw => SyntaxKind::StateKw,
        LogosToken::RequirementKw => SyntaxKind::RequirementKw,
        LogosToken::ConstraintKw => SyntaxKind::ConstraintKw,
        LogosToken::ConnectionKw => SyntaxKind::ConnectionKw,
        LogosToken::AllocationKw => SyntaxKind::AllocationKw,
        LogosToken::InterfaceKw => SyntaxKind::InterfaceKw,
        LogosToken::FlowKw => SyntaxKind::FlowKw,
        LogosToken::UseCaseKw => SyntaxKind::UseCaseKw,
        LogosToken::ViewKw => SyntaxKind::ViewKw,
        LogosToken::ViewpointKw => SyntaxKind::ViewpointKw,
        LogosToken::RenderingKw => SyntaxKind::RenderingKw,
        LogosToken::MetadataKw => SyntaxKind::MetadataKw,
        LogosToken::OccurrenceKw => SyntaxKind::OccurrenceKw,
        LogosToken::AnalysisKw => SyntaxKind::AnalysisKw,
        LogosToken::VerificationKw => SyntaxKind::VerificationKw,
        LogosToken::ConcernKw => SyntaxKind::ConcernKw,
        LogosToken::EnumKw => SyntaxKind::EnumKw,
        LogosToken::CalcKw => SyntaxKind::CalcKw,
        LogosToken::CaseKw => SyntaxKind::CaseKw,
        LogosToken::IndividualKw => SyntaxKind::IndividualKw,
        LogosToken::AbstractKw => SyntaxKind::AbstractKw,
        LogosToken::RefKw => SyntaxKind::RefKw,
        LogosToken::ReadonlyKw => SyntaxKind::ReadonlyKw,
        LogosToken::DerivedKw => SyntaxKind::DerivedKw,
        LogosToken::EndKw => SyntaxKind::EndKw,
        LogosToken::InKw => SyntaxKind::InKw,
        LogosToken::OutKw => SyntaxKind::OutKw,
        LogosToken::InoutKw => SyntaxKind::InoutKw,
        LogosToken::AliasKw => SyntaxKind::AliasKw,
        LogosToken::DocKw => SyntaxKind::DocKw,
        LogosToken::CommentKw => SyntaxKind::CommentKw,
        LogosToken::AboutKw => SyntaxKind::AboutKw,
        LogosToken::RepKw => SyntaxKind::RepKw,
        LogosToken::LanguageKw => SyntaxKind::LanguageKw,
        LogosToken::SpecializesKw => SyntaxKind::SpecializesKw,
        LogosToken::SubsetsKw => SyntaxKind::SubsetsKw,
        LogosToken::RedefinesKw => SyntaxKind::RedefinesKw,
        LogosToken::TypedByKw => SyntaxKind::TypedByKw,
        LogosToken::ReferencesKw => SyntaxKind::ReferencesKw,
        LogosToken::AssertKw => SyntaxKind::AssertKw,
        LogosToken::AssumeKw => SyntaxKind::AssumeKw,
        LogosToken::RequireKw => SyntaxKind::RequireKw,
        LogosToken::PerformKw => SyntaxKind::PerformKw,
        LogosToken::ExhibitKw => SyntaxKind::ExhibitKw,
        LogosToken::IncludeKw => SyntaxKind::IncludeKw,
        LogosToken::SatisfyKw => SyntaxKind::SatisfyKw,
        LogosToken::EntryKw => SyntaxKind::EntryKw,
        LogosToken::ExitKw => SyntaxKind::ExitKw,
        LogosToken::DoKw => SyntaxKind::DoKw,
        LogosToken::IfKw => SyntaxKind::IfKw,
        LogosToken::ElseKw => SyntaxKind::ElseKw,
        LogosToken::ThenKw => SyntaxKind::ThenKw,
        LogosToken::LoopKw => SyntaxKind::LoopKw,
        LogosToken::WhileKw => SyntaxKind::WhileKw,
        LogosToken::UntilKw => SyntaxKind::UntilKw,
        LogosToken::ForKw => SyntaxKind::ForKw,
        LogosToken::ForkKw => SyntaxKind::ForkKw,
        LogosToken::JoinKw => SyntaxKind::JoinKw,
        LogosToken::MergeKw => SyntaxKind::MergeKw,
        LogosToken::DecideKw => SyntaxKind::DecideKw,
        LogosToken::AcceptKw => SyntaxKind::AcceptKw,
        LogosToken::SendKw => SyntaxKind::SendKw,
        LogosToken::ViaKw => SyntaxKind::ViaKw,
        LogosToken::ToKw => SyntaxKind::ToKw,
        LogosToken::FromKw => SyntaxKind::FromKw,
        LogosToken::DependencyKw => SyntaxKind::DependencyKw,
        LogosToken::FilterKw => SyntaxKind::FilterKw,
        LogosToken::ExposeKw => SyntaxKind::ExposeKw,
        LogosToken::AllKw => SyntaxKind::AllKw,
        LogosToken::FirstKw => SyntaxKind::FirstKw,
        LogosToken::ModelKw => SyntaxKind::ModelKw,
        LogosToken::LibraryKw => SyntaxKind::LibraryKw,
        LogosToken::StandardKw => SyntaxKind::StandardKw,
        LogosToken::PrivateKw => SyntaxKind::PrivateKw,
        LogosToken::ProtectedKw => SyntaxKind::ProtectedKw,
        LogosToken::PublicKw => SyntaxKind::PublicKw,
        LogosToken::TrueKw => SyntaxKind::TrueKw,
        LogosToken::FalseKw => SyntaxKind::FalseKw,
        LogosToken::NullKw => SyntaxKind::NullKw,
        LogosToken::AndKw => SyntaxKind::AndKw,
        LogosToken::OrKw => SyntaxKind::OrKw,
        LogosToken::NotKw => SyntaxKind::NotKw,
        LogosToken::XorKw => SyntaxKind::XorKw,
        LogosToken::ImpliesKw => SyntaxKind::ImpliesKw,
        LogosToken::HasTypeKw => SyntaxKind::HasTypeKw,
        LogosToken::IsTypeKw => SyntaxKind::IsTypeKw,
        LogosToken::AsKw => SyntaxKind::AsKw,
        LogosToken::MetaKw => SyntaxKind::MetaKw,
        LogosToken::StructKw => SyntaxKind::StructKw,
        LogosToken::ClassKw => SyntaxKind::ClassKw,
        LogosToken::DataTypeKw => SyntaxKind::DataTypeKw,
        LogosToken::AssocKw => SyntaxKind::AssocKw,
        LogosToken::BehaviorKw => SyntaxKind::BehaviorKw,
        LogosToken::FunctionKw => SyntaxKind::FunctionKw,
        LogosToken::TypeKw => SyntaxKind::TypeKw,
        LogosToken::FeatureKw => SyntaxKind::FeatureKw,
        LogosToken::StepKw => SyntaxKind::StepKw,
        LogosToken::ExprKw => SyntaxKind::ExprKw,
        LogosToken::BindingKw => SyntaxKind::BindingKw,
        LogosToken::SuccessionKw => SyntaxKind::SuccessionKw,
        LogosToken::ConnectorKw => SyntaxKind::ConnectorKw,
        LogosToken::InvKw => SyntaxKind::InvKw,
        LogosToken::NonuniqueKw => SyntaxKind::NonuniqueKw,
        LogosToken::OrderedKw => SyntaxKind::OrderedKw,
        LogosToken::UnorderedKw => SyntaxKind::UnorderedKw,
        LogosToken::LBrace => SyntaxKind::LBrace,
        LogosToken::RBrace => SyntaxKind::RBrace,
        LogosToken::LBracket => SyntaxKind::LBracket,
        LogosToken::RBracket => SyntaxKind::RBracket,
        LogosToken::LParen => SyntaxKind::LParen,
        LogosToken::RParen => SyntaxKind::RParen,
        LogosToken::Semicolon => SyntaxKind::Semicolon,
        LogosToken::Colon => SyntaxKind::Colon,
        LogosToken::ColonColon => SyntaxKind::ColonColon,
        LogosToken::Dot => SyntaxKind::Dot,
        LogosToken::DotDot => SyntaxKind::Dot, // Map to regular dot for now
        LogosToken::Comma => SyntaxKind::Comma,
        LogosToken::Eq => SyntaxKind::Eq,
        LogosToken::EqEq => SyntaxKind::EqEq,
        LogosToken::NotEq => SyntaxKind::NotEq,
        LogosToken::Lt => SyntaxKind::Lt,
        LogosToken::Gt => SyntaxKind::Gt,
        LogosToken::LtEq => SyntaxKind::LtEq,
        LogosToken::GtEq => SyntaxKind::GtEq,
        LogosToken::Arrow => SyntaxKind::Arrow,
        LogosToken::SubsetsToken => SyntaxKind::Gt, // :>
        LogosToken::RedefinesToken => SyntaxKind::Gt, // :>>
        LogosToken::At => SyntaxKind::At,
        LogosToken::Star => SyntaxKind::Star,
        LogosToken::StarStar => SyntaxKind::Star,
        LogosToken::Plus => SyntaxKind::Plus,
        LogosToken::Minus => SyntaxKind::Minus,
        LogosToken::Slash => SyntaxKind::Slash,
        LogosToken::Percent => SyntaxKind::Percent,
        LogosToken::Caret => SyntaxKind::Caret,
        LogosToken::Tilde => SyntaxKind::Tilde,
        LogosToken::Question => SyntaxKind::Question,
        LogosToken::QuestionQuestion => SyntaxKind::Question,
        LogosToken::Bang => SyntaxKind::Bang,
        LogosToken::Pipe => SyntaxKind::Pipe,
        LogosToken::PipePipe => SyntaxKind::Pipe,
        LogosToken::Ampersand => SyntaxKind::Ampersand,
        LogosToken::AmpAmp => SyntaxKind::Ampersand,
        LogosToken::Hash => SyntaxKind::Hash,
        LogosToken::Number => SyntaxKind::Number,
        LogosToken::String => SyntaxKind::String,
        LogosToken::Identifier => SyntaxKind::Identifier,
    }
}

/// Tokenize source code into a vector of tokens with their text
pub fn tokenize(source: &str) -> Vec<Token<'_>> {
    let mut tokens = Vec::new();
    let mut lexer = LogosToken::lexer(source);

    while let Some(result) = lexer.next() {
        let text = lexer.slice();
        let kind = match result {
            Ok(token) => to_syntax_kind(token),
            Err(()) => SyntaxKind::Error,
        };
        tokens.push(Token { kind, text });
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_simple() {
        let tokens = tokenize("package Test { }");
        assert!(tokens.iter().any(|t| t.kind == SyntaxKind::PackageKw));
        assert!(tokens.iter().any(|t| t.kind == SyntaxKind::Identifier));
        assert!(tokens.iter().any(|t| t.kind == SyntaxKind::LBrace));
        assert!(tokens.iter().any(|t| t.kind == SyntaxKind::RBrace));
    }

    #[test]
    fn test_tokenize_with_comments() {
        let tokens = tokenize("// comment\npackage Test;");
        assert!(tokens.iter().any(|t| t.kind == SyntaxKind::LineComment));
        assert!(tokens.iter().any(|t| t.kind == SyntaxKind::PackageKw));
    }

    #[test]
    fn test_tokenize_block_comment() {
        let tokens = tokenize("/* block */ package Test;");
        assert!(tokens.iter().any(|t| t.kind == SyntaxKind::BlockComment));
    }

    #[test]
    fn test_whitespace_preserved() {
        let tokens = tokenize("package   Test");
        assert!(tokens.iter().any(|t| t.kind == SyntaxKind::Whitespace));
    }
}
