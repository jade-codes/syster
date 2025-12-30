#![allow(clippy::unwrap_used)]
#![allow(clippy::panic)]

use super::*;
use crate::core::Span;
use crate::syntax::sysml::visitor::{AstVisitor, Visitable};

// ============================================================================
// Comment struct tests
// ============================================================================

#[test]
fn test_comment_creation() {
    let comment = Comment {
        content: "This is a test comment".to_string(),
        span: None,
    };

    assert_eq!(comment.content, "This is a test comment");
    assert_eq!(comment.span, None);
}

#[test]
fn test_comment_with_span() {
    let span = Span {
        start: crate::core::span::Position { line: 1, column: 0 },
        end: crate::core::span::Position {
            line: 1,
            column: 22,
        },
    };

    let comment = Comment {
        content: "Comment with span".to_string(),
        span: Some(span),
    };

    assert_eq!(comment.content, "Comment with span");
    assert_eq!(comment.span, Some(span));
}

#[test]
fn test_comment_empty_content() {
    let comment = Comment {
        content: String::new(),
        span: None,
    };

    assert_eq!(comment.content, "");
    assert!(comment.content.is_empty());
    assert_eq!(comment.span, None);
}

#[test]
fn test_comment_multiline_content() {
    let content = "This is a\nmultiline\ncomment".to_string();
    let comment = Comment {
        content: content.clone(),
        span: None,
    };

    assert_eq!(comment.content, content);
    assert!(comment.content.contains('\n'));
}

#[test]
fn test_comment_special_characters() {
    let content = "Comment with special chars: @#$%^&*(){}[]|\\:;\"'<>,.?/~`".to_string();
    let comment = Comment {
        content: content.clone(),
        span: None,
    };

    assert_eq!(comment.content, content);
}

#[test]
fn test_comment_clone() {
    let comment1 = Comment {
        content: "Original comment".to_string(),
        span: None,
    };

    let comment2 = comment1.clone();

    assert_eq!(comment1.content, comment2.content);
    assert_eq!(comment1.span, comment2.span);
}

#[test]
fn test_comment_partial_eq() {
    let comment1 = Comment {
        content: "Same comment".to_string(),
        span: None,
    };

    let comment2 = Comment {
        content: "Same comment".to_string(),
        span: None,
    };

    assert_eq!(comment1, comment2);
}

#[test]
fn test_comment_not_eq_different_content() {
    let comment1 = Comment {
        content: "First comment".to_string(),
        span: None,
    };

    let comment2 = Comment {
        content: "Second comment".to_string(),
        span: None,
    };

    assert_ne!(comment1, comment2);
}

#[test]
fn test_comment_not_eq_different_span() {
    let span1 = Span {
        start: crate::core::span::Position { line: 1, column: 0 },
        end: crate::core::span::Position {
            line: 1,
            column: 10,
        },
    };

    let span2 = Span {
        start: crate::core::span::Position { line: 2, column: 0 },
        end: crate::core::span::Position {
            line: 2,
            column: 10,
        },
    };

    let comment1 = Comment {
        content: "Same comment".to_string(),
        span: Some(span1),
    };

    let comment2 = Comment {
        content: "Same comment".to_string(),
        span: Some(span2),
    };

    assert_ne!(comment1, comment2);
}

#[test]
fn test_comment_debug_trait() {
    let comment = Comment {
        content: "Debug test".to_string(),
        span: None,
    };

    let debug_str = format!("{:?}", comment);
    assert!(debug_str.contains("Comment"));
    assert!(debug_str.contains("Debug test"));
}

// ============================================================================
// Comment as Element tests
// ============================================================================

#[test]
fn test_comment_as_element() {
    let comment = Comment {
        content: "Test comment".to_string(),
        span: None,
    };

    let element = Element::Comment(comment.clone());

    match element {
        Element::Comment(c) => {
            assert_eq!(c.content, "Test comment");
            assert_eq!(c, comment);
        }
        _ => panic!("Expected Element::Comment variant"),
    }
}

#[test]
fn test_comment_element_pattern_matching() {
    let comment = Comment {
        content: "Pattern match test".to_string(),
        span: None,
    };

    let element = Element::Comment(comment);

    if let Element::Comment(c) = element {
        assert_eq!(c.content, "Pattern match test");
    } else {
        panic!("Failed to match Element::Comment");
    }
}

// ============================================================================
// Visitable trait tests (Issue #168)
// ============================================================================

struct GenericTestVisitor {
    comment_visited: bool,
    comment_content: Option<String>,
}

impl AstVisitor for GenericTestVisitor {
    fn visit_comment(&mut self, comment: &Comment) {
        self.comment_visited = true;
        self.comment_content = Some(comment.content.clone());
    }
}

#[test]
fn test_comment_visitable_accept_generic() {
    let comment = Comment {
        content: "Visitor test".to_string(),
        span: None,
    };

    let mut visitor = GenericTestVisitor {
        comment_visited: false,
        comment_content: None,
    };

    comment.accept(&mut visitor);

    assert!(visitor.comment_visited, "Comment should be visited");
    assert_eq!(
        visitor.comment_content,
        Some("Visitor test".to_string()),
        "Visitor should capture comment content"
    );
}

#[test]
fn test_comment_visitable_with_multiple_visitors() {
    let comment = Comment {
        content: "Multiple visitors".to_string(),
        span: None,
    };

    let mut visitor1 = GenericTestVisitor {
        comment_visited: false,
        comment_content: None,
    };

    let mut visitor2 = GenericTestVisitor {
        comment_visited: false,
        comment_content: None,
    };

    comment.accept(&mut visitor1);
    comment.accept(&mut visitor2);

    assert!(visitor1.comment_visited);
    assert!(visitor2.comment_visited);
    assert_eq!(visitor1.comment_content, visitor2.comment_content);
}

#[test]
fn test_comment_visitable_empty_content() {
    let comment = Comment {
        content: String::new(),
        span: None,
    };

    let mut visitor = GenericTestVisitor {
        comment_visited: false,
        comment_content: None,
    };

    comment.accept(&mut visitor);

    assert!(visitor.comment_visited);
    assert_eq!(visitor.comment_content, Some(String::new()));
}

#[test]
fn test_comment_visitable_with_span() {
    let span = Span {
        start: crate::core::span::Position {
            line: 5,
            column: 10,
        },
        end: crate::core::span::Position {
            line: 5,
            column: 30,
        },
    };

    let comment = Comment {
        content: "Comment with span".to_string(),
        span: Some(span),
    };

    let mut visitor = GenericTestVisitor {
        comment_visited: false,
        comment_content: None,
    };

    comment.accept(&mut visitor);

    assert!(visitor.comment_visited);
    assert_eq!(
        visitor.comment_content,
        Some("Comment with span".to_string())
    );
}

// ============================================================================
// CommentCountingVisitor tests (Issue #167)
// ============================================================================

/// A visitor that counts comment visits and total visit calls.
/// This is separate from the CountingVisitor in tests.rs which tracks all element types.
/// We use a focused visitor here to specifically test comment visitor behavior.
struct CommentCountingVisitor {
    comments: usize,
    total_visits: usize,
}

impl AstVisitor for CommentCountingVisitor {
    fn visit_comment(&mut self, _comment: &Comment) {
        self.comments += 1;
        self.total_visits += 1;
    }

    fn visit_element(&mut self, _element: &Element) {
        self.total_visits += 1;
    }
}

#[test]
fn test_comment_visitable_accept_counting_visitor() {
    let comment = Comment {
        content: "Counting test".to_string(),
        span: None,
    };

    let mut visitor = CommentCountingVisitor {
        comments: 0,
        total_visits: 0,
    };

    comment.accept(&mut visitor);

    assert_eq!(visitor.comments, 1, "Should visit exactly one comment");
    assert_eq!(
        visitor.total_visits, 1,
        "Total visits should match comment visits"
    );
}

#[test]
fn test_comment_visitable_counting_multiple_comments() {
    let comment1 = Comment {
        content: "First".to_string(),
        span: None,
    };
    let comment2 = Comment {
        content: "Second".to_string(),
        span: None,
    };
    let comment3 = Comment {
        content: "Third".to_string(),
        span: None,
    };

    let mut visitor = CommentCountingVisitor {
        comments: 0,
        total_visits: 0,
    };

    comment1.accept(&mut visitor);
    comment2.accept(&mut visitor);
    comment3.accept(&mut visitor);

    assert_eq!(visitor.comments, 3, "Should count all three comments");
    assert_eq!(visitor.total_visits, 3);
}

#[test]
fn test_comment_element_with_counting_visitor() {
    let comment = Comment {
        content: "Element test".to_string(),
        span: None,
    };
    let element = Element::Comment(comment);

    let mut visitor = CommentCountingVisitor {
        comments: 0,
        total_visits: 0,
    };

    element.accept(&mut visitor);

    assert_eq!(visitor.comments, 1, "Should count comment through element");
    // Element visitor calls visit_element then visit_comment
    assert_eq!(
        visitor.total_visits, 2,
        "Should count both element and comment visits"
    );
}

#[test]
fn test_comment_counting_visitor_zero_initial() {
    let visitor = CommentCountingVisitor {
        comments: 0,
        total_visits: 0,
    };

    assert_eq!(visitor.comments, 0, "Initial comment count should be zero");
    assert_eq!(
        visitor.total_visits, 0,
        "Initial total visits should be zero"
    );
}

#[test]
fn test_comment_in_file_with_counting_visitor() {
    let comment1 = Comment {
        content: "Comment 1".to_string(),
        span: None,
    };
    let comment2 = Comment {
        content: "Comment 2".to_string(),
        span: None,
    };

    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Comment(comment1), Element::Comment(comment2)],
    };

    let mut visitor = CommentCountingVisitor {
        comments: 0,
        total_visits: 0,
    };

    file.accept(&mut visitor);

    assert_eq!(visitor.comments, 2, "Should count both comments in file");
    assert_eq!(
        visitor.total_visits, 4,
        "Should count 2 element visits + 2 comment visits"
    );
}

// ============================================================================
// Edge case tests
// ============================================================================

#[test]
fn test_comment_very_long_content() {
    let long_content = "x".repeat(10000);
    let comment = Comment {
        content: long_content.clone(),
        span: None,
    };

    assert_eq!(comment.content.len(), 10000);
    assert_eq!(comment.content, long_content);
}

#[test]
fn test_comment_unicode_content() {
    let unicode_content = "Hello 世界 🌍 Привет مرحبا".to_string();
    let comment = Comment {
        content: unicode_content.clone(),
        span: None,
    };

    assert_eq!(comment.content, unicode_content);

    let mut visitor = GenericTestVisitor {
        comment_visited: false,
        comment_content: None,
    };

    comment.accept(&mut visitor);

    assert!(visitor.comment_visited);
    assert_eq!(visitor.comment_content, Some(unicode_content));
}

#[test]
fn test_comment_with_embedded_quotes() {
    let content = r#"Comment with "double quotes" and 'single quotes'"#.to_string();
    let comment = Comment {
        content: content.clone(),
        span: None,
    };

    assert_eq!(comment.content, content);
}

#[test]
fn test_comment_with_escape_sequences() {
    let content = "Comment with\ttabs\nand\nnewlines\rand\\backslashes".to_string();
    let comment = Comment {
        content: content.clone(),
        span: None,
    };

    assert_eq!(comment.content, content);
}
