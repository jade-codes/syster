#![allow(clippy::unwrap_used)]
#![allow(clippy::panic)]

use super::super::*;
use crate::core::Span;

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

    let debug_str = format!("{comment:?}");
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
