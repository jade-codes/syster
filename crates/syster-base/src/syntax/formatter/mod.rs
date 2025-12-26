//! Rowan-based formatter for SysML/KerML
//!
//! This module provides lossless formatting that preserves comments and trivia.
//! It uses Rowan for CST (Concrete Syntax Tree) representation and Logos for lexing.

mod lexer;
mod options;
mod syntax_kind;

#[cfg(test)]
mod tests;

pub use options::FormatOptions;
use rowan::GreenNodeBuilder;
use syntax_kind::{SyntaxKind, SyntaxNode};

/// Format SysML/KerML source code with the given options
pub fn format(source: &str, options: &FormatOptions) -> String {
    let tokens = lexer::tokenize(source);
    let cst = parse_to_cst(&tokens);
    render(&cst, options)
}

/// Token with text and kind
pub(crate) struct Token<'a> {
    pub kind: SyntaxKind,
    pub text: &'a str,
}

/// Parse tokens into a CST (Concrete Syntax Tree)
fn parse_to_cst(tokens: &[Token]) -> SyntaxNode {
    let mut builder = GreenNodeBuilder::new();

    builder.start_node(SyntaxKind::SourceFile.into());

    let mut pos = 0;
    while pos < tokens.len() {
        pos = parse_element(tokens, pos, &mut builder);
    }

    builder.finish_node();

    SyntaxNode::new_root(builder.finish())
}

/// Parse a single element (package, definition, usage, import, comment, etc.)
fn parse_element(tokens: &[Token], mut pos: usize, builder: &mut GreenNodeBuilder) -> usize {
    // Consume leading trivia
    pos = consume_trivia(tokens, pos, builder);

    if pos >= tokens.len() {
        return pos;
    }

    let token = &tokens[pos];

    match token.kind {
        SyntaxKind::PackageKw => parse_package(tokens, pos, builder),
        SyntaxKind::PartKw
        | SyntaxKind::AttributeKw
        | SyntaxKind::PortKw
        | SyntaxKind::ItemKw
        | SyntaxKind::ActionKw
        | SyntaxKind::StateKw
        | SyntaxKind::RequirementKw
        | SyntaxKind::ConstraintKw
        | SyntaxKind::ConnectionKw
        | SyntaxKind::AllocationKw
        | SyntaxKind::InterfaceKw
        | SyntaxKind::FlowKw
        | SyntaxKind::UseCaseKw
        | SyntaxKind::ViewKw
        | SyntaxKind::ViewpointKw
        | SyntaxKind::RenderingKw
        | SyntaxKind::MetadataKw
        | SyntaxKind::OccurrenceKw
        | SyntaxKind::AnalysisKw
        | SyntaxKind::VerificationKw
        | SyntaxKind::ConcernKw
        | SyntaxKind::EnumKw
        | SyntaxKind::CalcKw
        | SyntaxKind::CaseKw
        | SyntaxKind::IndividualKw => parse_definition_or_usage(tokens, pos, builder),
        SyntaxKind::AbstractKw | SyntaxKind::RefKw | SyntaxKind::ReadonlyKw => {
            parse_definition_or_usage(tokens, pos, builder)
        }
        SyntaxKind::ImportKw => parse_import(tokens, pos, builder),
        SyntaxKind::AliasKw => parse_alias(tokens, pos, builder),
        SyntaxKind::DocKw | SyntaxKind::CommentKw => parse_annotation(tokens, pos, builder),
        _ => {
            // Unknown token, just add it and move on
            builder.token(token.kind.into(), token.text);
            pos + 1
        }
    }
}

/// Consume trivia (whitespace, comments) and add to tree
fn consume_trivia(tokens: &[Token], mut pos: usize, builder: &mut GreenNodeBuilder) -> usize {
    while pos < tokens.len() {
        let token = &tokens[pos];
        match token.kind {
            SyntaxKind::Whitespace | SyntaxKind::LineComment | SyntaxKind::BlockComment => {
                builder.token(token.kind.into(), token.text);
                pos += 1;
            }
            _ => break,
        }
    }
    pos
}

/// Parse a package declaration
fn parse_package(tokens: &[Token], mut pos: usize, builder: &mut GreenNodeBuilder) -> usize {
    builder.start_node(SyntaxKind::Package.into());

    // 'package' keyword
    builder.token(tokens[pos].kind.into(), tokens[pos].text);
    pos += 1;

    pos = consume_trivia(tokens, pos, builder);

    // Optional name
    if pos < tokens.len() && tokens[pos].kind == SyntaxKind::Identifier {
        builder.start_node(SyntaxKind::Name.into());
        builder.token(tokens[pos].kind.into(), tokens[pos].text);
        builder.finish_node();
        pos += 1;
    }

    pos = consume_trivia(tokens, pos, builder);

    // Body or semicolon
    if pos < tokens.len() {
        if tokens[pos].kind == SyntaxKind::LBrace {
            pos = parse_body(tokens, pos, builder);
        } else if tokens[pos].kind == SyntaxKind::Semicolon {
            builder.token(tokens[pos].kind.into(), tokens[pos].text);
            pos += 1;
        }
    }

    builder.finish_node();
    pos
}

/// Parse a block body { ... }
fn parse_body(tokens: &[Token], mut pos: usize, builder: &mut GreenNodeBuilder) -> usize {
    builder.start_node(SyntaxKind::Body.into());

    // Opening brace
    builder.token(tokens[pos].kind.into(), tokens[pos].text);
    pos += 1;

    // Parse elements until closing brace
    while pos < tokens.len() && tokens[pos].kind != SyntaxKind::RBrace {
        let prev_pos = pos;
        pos = parse_element(tokens, pos, builder);
        if pos == prev_pos {
            // Avoid infinite loop - consume unknown token
            builder.token(tokens[pos].kind.into(), tokens[pos].text);
            pos += 1;
        }
    }

    // Closing brace
    if pos < tokens.len() && tokens[pos].kind == SyntaxKind::RBrace {
        pos = consume_trivia(tokens, pos, builder);
        if pos < tokens.len() && tokens[pos].kind == SyntaxKind::RBrace {
            builder.token(tokens[pos].kind.into(), tokens[pos].text);
            pos += 1;
        }
    }

    builder.finish_node();
    pos
}

/// Parse a definition or usage (part def, part, attribute, etc.)
fn parse_definition_or_usage(
    tokens: &[Token],
    mut pos: usize,
    builder: &mut GreenNodeBuilder,
) -> usize {
    // Determine if this is a definition (has 'def' keyword) or usage
    let is_definition = has_def_keyword(tokens, pos);

    if is_definition {
        builder.start_node(SyntaxKind::Definition.into());
    } else {
        builder.start_node(SyntaxKind::Usage.into());
    }

    // Consume modifiers (abstract, ref, readonly)
    while pos < tokens.len() {
        match tokens[pos].kind {
            SyntaxKind::AbstractKw | SyntaxKind::RefKw | SyntaxKind::ReadonlyKw => {
                builder.token(tokens[pos].kind.into(), tokens[pos].text);
                pos += 1;
                pos = consume_trivia(tokens, pos, builder);
            }
            _ => break,
        }
    }

    // Keyword (part, attribute, etc.)
    if pos < tokens.len() && is_element_keyword(&tokens[pos].kind) {
        builder.token(tokens[pos].kind.into(), tokens[pos].text);
        pos += 1;
        pos = consume_trivia(tokens, pos, builder);
    }

    // 'def' keyword if definition
    if pos < tokens.len() && tokens[pos].kind == SyntaxKind::DefKw {
        builder.token(tokens[pos].kind.into(), tokens[pos].text);
        pos += 1;
        pos = consume_trivia(tokens, pos, builder);
    }

    // Name
    if pos < tokens.len() && tokens[pos].kind == SyntaxKind::Identifier {
        builder.start_node(SyntaxKind::Name.into());
        builder.token(tokens[pos].kind.into(), tokens[pos].text);
        builder.finish_node();
        pos += 1;
    }

    pos = consume_trivia(tokens, pos, builder);

    // Relationships and type annotations (consume until { or ;)
    while pos < tokens.len() {
        match tokens[pos].kind {
            SyntaxKind::LBrace | SyntaxKind::Semicolon => break,
            _ => {
                builder.token(tokens[pos].kind.into(), tokens[pos].text);
                pos += 1;
            }
        }
    }

    // Body or semicolon
    if pos < tokens.len() {
        if tokens[pos].kind == SyntaxKind::LBrace {
            pos = parse_body(tokens, pos, builder);
        } else if tokens[pos].kind == SyntaxKind::Semicolon {
            builder.token(tokens[pos].kind.into(), tokens[pos].text);
            pos += 1;
        }
    }

    builder.finish_node();
    pos
}

/// Check if a sequence starting at pos has a 'def' keyword before { or ;
fn has_def_keyword(tokens: &[Token], mut pos: usize) -> bool {
    while pos < tokens.len() {
        match tokens[pos].kind {
            SyntaxKind::DefKw => return true,
            SyntaxKind::LBrace | SyntaxKind::Semicolon => return false,
            _ => pos += 1,
        }
    }
    false
}

/// Check if a kind is an element keyword
fn is_element_keyword(kind: &SyntaxKind) -> bool {
    matches!(
        kind,
        SyntaxKind::PartKw
            | SyntaxKind::AttributeKw
            | SyntaxKind::PortKw
            | SyntaxKind::ItemKw
            | SyntaxKind::ActionKw
            | SyntaxKind::StateKw
            | SyntaxKind::RequirementKw
            | SyntaxKind::ConstraintKw
            | SyntaxKind::ConnectionKw
            | SyntaxKind::AllocationKw
            | SyntaxKind::InterfaceKw
            | SyntaxKind::FlowKw
            | SyntaxKind::UseCaseKw
            | SyntaxKind::ViewKw
            | SyntaxKind::ViewpointKw
            | SyntaxKind::RenderingKw
            | SyntaxKind::MetadataKw
            | SyntaxKind::OccurrenceKw
            | SyntaxKind::AnalysisKw
            | SyntaxKind::VerificationKw
            | SyntaxKind::ConcernKw
            | SyntaxKind::EnumKw
            | SyntaxKind::CalcKw
            | SyntaxKind::CaseKw
            | SyntaxKind::IndividualKw
    )
}

/// Parse an import statement
fn parse_import(tokens: &[Token], mut pos: usize, builder: &mut GreenNodeBuilder) -> usize {
    builder.start_node(SyntaxKind::Import.into());

    // 'import' keyword
    builder.token(tokens[pos].kind.into(), tokens[pos].text);
    pos += 1;

    // Consume until semicolon
    while pos < tokens.len() && tokens[pos].kind != SyntaxKind::Semicolon {
        builder.token(tokens[pos].kind.into(), tokens[pos].text);
        pos += 1;
    }

    // Semicolon
    if pos < tokens.len() && tokens[pos].kind == SyntaxKind::Semicolon {
        builder.token(tokens[pos].kind.into(), tokens[pos].text);
        pos += 1;
    }

    builder.finish_node();
    pos
}

/// Parse an alias declaration
fn parse_alias(tokens: &[Token], mut pos: usize, builder: &mut GreenNodeBuilder) -> usize {
    builder.start_node(SyntaxKind::Alias.into());

    // 'alias' keyword
    builder.token(tokens[pos].kind.into(), tokens[pos].text);
    pos += 1;

    // Consume until semicolon
    while pos < tokens.len() && tokens[pos].kind != SyntaxKind::Semicolon {
        builder.token(tokens[pos].kind.into(), tokens[pos].text);
        pos += 1;
    }

    // Semicolon
    if pos < tokens.len() && tokens[pos].kind == SyntaxKind::Semicolon {
        builder.token(tokens[pos].kind.into(), tokens[pos].text);
        pos += 1;
    }

    builder.finish_node();
    pos
}

/// Parse a doc or comment annotation
fn parse_annotation(tokens: &[Token], mut pos: usize, builder: &mut GreenNodeBuilder) -> usize {
    builder.start_node(SyntaxKind::Annotation.into());

    // 'doc' or 'comment' keyword
    builder.token(tokens[pos].kind.into(), tokens[pos].text);
    pos += 1;

    // Consume until end of annotation (block comment or semicolon)
    while pos < tokens.len() {
        let kind = tokens[pos].kind;
        builder.token(tokens[pos].kind.into(), tokens[pos].text);
        pos += 1;

        if kind == SyntaxKind::BlockComment || kind == SyntaxKind::Semicolon {
            break;
        }
    }

    builder.finish_node();
    pos
}

/// Render the CST back to formatted source code
fn render(node: &SyntaxNode, options: &FormatOptions) -> String {
    let mut output = String::new();
    let mut indent_level: usize = 0;
    let mut at_line_start = true;

    render_node(
        node,
        options,
        &mut output,
        &mut indent_level,
        &mut at_line_start,
    );

    output
}

fn render_node(
    node: &SyntaxNode,
    options: &FormatOptions,
    output: &mut String,
    indent_level: &mut usize,
    at_line_start: &mut bool,
) {
    for child in node.children_with_tokens() {
        match child {
            rowan::NodeOrToken::Token(token) => {
                let kind: SyntaxKind = token.kind();
                let text = token.text();

                match kind {
                    SyntaxKind::Whitespace => {
                        // Normalize whitespace
                        if text.contains('\n') {
                            // Preserve newlines
                            let newline_count = text.matches('\n').count();
                            for _ in 0..newline_count.min(2) {
                                output.push('\n');
                            }
                            *at_line_start = true;
                        } else if !*at_line_start && !output.ends_with(' ') && !output.is_empty() {
                            // Single space between tokens
                            output.push(' ');
                        }
                    }
                    SyntaxKind::LineComment => {
                        if *at_line_start {
                            output.push_str(&options.indent(*indent_level));
                            *at_line_start = false;
                        }
                        output.push_str(text);
                    }
                    SyntaxKind::BlockComment => {
                        if *at_line_start {
                            output.push_str(&options.indent(*indent_level));
                            *at_line_start = false;
                        }
                        output.push_str(text);
                    }
                    SyntaxKind::LBrace => {
                        if !output.ends_with(' ') && !output.ends_with('\n') {
                            output.push(' ');
                        }
                        output.push('{');
                        *indent_level += 1;
                    }
                    SyntaxKind::RBrace => {
                        *indent_level = indent_level.saturating_sub(1);
                        if *at_line_start {
                            output.push_str(&options.indent(*indent_level));
                        }
                        output.push('}');
                        *at_line_start = false;
                    }
                    SyntaxKind::Semicolon => {
                        output.push(';');
                        *at_line_start = false;
                    }
                    _ => {
                        if *at_line_start {
                            output.push_str(&options.indent(*indent_level));
                            *at_line_start = false;
                        } else if !output.ends_with(' ')
                            && !output.ends_with('\n')
                            && !output.ends_with('{')
                            && !output.is_empty()
                        {
                            output.push(' ');
                        }
                        output.push_str(text);
                    }
                }
            }
            rowan::NodeOrToken::Node(child_node) => {
                render_node(&child_node, options, output, indent_level, at_line_start);
            }
        }
    }
}
