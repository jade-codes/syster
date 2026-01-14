//! Syntax kinds for the Rowan-based CST

/// All syntax kinds (tokens and nodes) in SysML/KerML
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum SyntaxKind {
    // === Trivia ===
    Whitespace = 0,
    LineComment,
    BlockComment,

    // === Literals ===
    Identifier,
    Number,
    String,

    // === Punctuation ===
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    LParen,
    RParen,
    Semicolon,
    Colon,
    ColonColon,
    Dot,
    Comma,
    Eq,
    EqEq,
    NotEq,
    Lt,
    Gt,
    LtEq,
    GtEq,
    Arrow,
    At,
    Star,
    Plus,
    Minus,
    Slash,
    Percent,
    Caret,
    Tilde,
    Question,
    Bang,
    Pipe,
    Ampersand,
    Hash,

    // === Keywords (SysML) ===
    PackageKw,
    PartKw,
    DefKw,
    ImportKw,
    AttributeKw,
    PortKw,
    ItemKw,
    ActionKw,
    StateKw,
    RequirementKw,
    ConstraintKw,
    ConnectionKw,
    AllocationKw,
    InterfaceKw,
    FlowKw,
    UseCaseKw,
    ViewKw,
    ViewpointKw,
    RenderingKw,
    MetadataKw,
    OccurrenceKw,
    AnalysisKw,
    VerificationKw,
    ConcernKw,
    EnumKw,
    CalcKw,
    CaseKw,
    IndividualKw,
    AbstractKw,
    RefKw,
    ConstKw,
    DerivedKw,
    EndKw,
    InKw,
    OutKw,
    InoutKw,
    AliasKw,
    DocKw,
    CommentKw,
    AboutKw,
    RepKw,
    LanguageKw,
    SpecializesKw,
    SubsetsKw,
    RedefinesKw,
    TypedByKw,
    ReferencesKw,
    AssertKw,
    AssumeKw,
    RequireKw,
    PerformKw,
    ExhibitKw,
    IncludeKw,
    SatisfyKw,
    EntryKw,
    ExitKw,
    DoKw,
    IfKw,
    ElseKw,
    ThenKw,
    LoopKw,
    WhileKw,
    UntilKw,
    ForKw,
    ForkKw,
    JoinKw,
    MergeKw,
    DecideKw,
    AcceptKw,
    SendKw,
    ViaKw,
    ToKw,
    FromKw,
    DependencyKw,
    FilterKw,
    ExposeKw,
    AllKw,
    FirstKw,
    ModelKw,
    LibraryKw,
    StandardKw,
    PrivateKw,
    ProtectedKw,
    PublicKw,
    TrueKw,
    FalseKw,
    NullKw,
    AndKw,
    OrKw,
    NotKw,
    XorKw,
    ImpliesKw,
    HasTypeKw,
    IsTypeKw,
    AsKw,
    MetaKw,

    // === Keywords (KerML) ===
    StructKw,
    ClassKw,
    DataTypeKw,
    AssocKw,
    BehaviorKw,
    FunctionKw,
    TypeKw,
    FeatureKw,
    StepKw,
    ExprKw,
    BindingKw,
    SuccessionKw,
    ConnectorKw,
    InvKw,
    NonuniqueKw,
    OrderedKw,
    UnorderedKw,

    // === Composite Nodes ===
    SourceFile,
    Package,
    Definition,
    Usage,
    Import,
    Alias,
    Annotation,
    Name,
    Body,
    Relationship,

    // === Special ===
    Error,
    Eof,
}

impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(kind: SyntaxKind) -> Self {
        Self(kind as u16)
    }
}

impl From<rowan::SyntaxKind> for SyntaxKind {
    fn from(raw: rowan::SyntaxKind) -> Self {
        // Safety: we control all syntax kinds
        unsafe { std::mem::transmute::<u16, SyntaxKind>(raw.0) }
    }
}

/// Language definition for Rowan
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SysMLLanguage {}

impl rowan::Language for SysMLLanguage {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        raw.into()
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        kind.into()
    }
}

/// Type aliases for convenience
pub type SyntaxNode = rowan::SyntaxNode<SysMLLanguage>;
#[allow(dead_code)]
pub type SyntaxToken = rowan::SyntaxToken<SysMLLanguage>;
#[allow(dead_code)]
pub type SyntaxElement = rowan::SyntaxElement<SysMLLanguage>;
