use crate::server::formatting::*;
use async_lsp::lsp_types::FormattingOptions;
use tokio_util::sync::CancellationToken;

#[test]
fn test_format_basic_kerml() {
    let input = "package Test {
feature x;
}";
    let format_options = syster::syntax::formatter::FormatOptions {
        tab_size: 4,
        insert_spaces: true,
        print_width: 80,
    };

    let result =
        syster::syntax::formatter::format_async(input, &format_options, &CancellationToken::new())
            .unwrap();

    // Rowan formatter preserves structure with proper indentation
    assert!(result.contains("package Test"));
    assert!(result.contains("feature x"));
}

#[test]
fn test_format_nested_kerml() {
    let input = "package Test {
struct Vehicle {
feature wheels;
}
}";
    let format_options = syster::syntax::formatter::FormatOptions {
        tab_size: 2,
        insert_spaces: true,
        print_width: 80,
    };

    let result =
        syster::syntax::formatter::format_async(input, &format_options, &CancellationToken::new())
            .unwrap();

    // Verify structure is preserved
    assert!(result.contains("package Test"));
    assert!(result.contains("struct Vehicle"));
    assert!(result.contains("feature wheels"));
}

#[test]
fn test_format_with_tabs() {
    let input = "package Test {
feature x;
}";
    let format_options = syster::syntax::formatter::FormatOptions {
        tab_size: 1,
        insert_spaces: false,
        print_width: 80,
    };

    let result =
        syster::syntax::formatter::format_async(input, &format_options, &CancellationToken::new())
            .unwrap();

    // Verify tabs are used for indentation
    assert!(result.contains("package Test"));
    assert!(result.contains("\t")); // Should have tab indentation
}

#[test]
fn test_format_preserves_comments() {
    let input = "// This is a comment
package Test {
/* block comment */
feature x;
}";
    let format_options = syster::syntax::formatter::FormatOptions {
        tab_size: 4,
        insert_spaces: true,
        print_width: 80,
    };

    let result =
        syster::syntax::formatter::format_async(input, &format_options, &CancellationToken::new())
            .unwrap();

    // Rowan formatter preserves comments
    assert!(result.contains("// This is a comment"));
    assert!(result.contains("/* block comment */"));
}

#[test]
fn test_format_normalizes_excessive_whitespace() {
    let input = "metadata def              ToolVariable";
    let format_options = syster::syntax::formatter::FormatOptions {
        tab_size: 4,
        insert_spaces: true,
        print_width: 80,
    };

    let result =
        syster::syntax::formatter::format_async(input, &format_options, &CancellationToken::new())
            .unwrap();

    // Multiple spaces should be normalized to single space
    assert_eq!(
        result.trim(),
        "metadata def ToolVariable",
        "Should normalize multiple spaces. Got: |{result}|"
    );
}

#[test]
fn test_lsp_format_normalizes_whitespace() {
    let source = "metadata def              ToolVariable  ";
    let options = FormattingOptions {
        tab_size: 4,
        insert_spaces: true,
        ..Default::default()
    };

    let result = format_text(source, options, &CancellationToken::new());

    assert!(result.is_some(), "format should return Some edits");
    let edits = result.unwrap();
    assert_eq!(edits.len(), 1, "Should have one edit");

    let new_text = &edits[0].new_text;
    assert!(
        new_text.contains("metadata def ToolVariable"),
        "Formatted text should normalize whitespace. Got: |{new_text}|"
    );
    assert!(
        !new_text.contains("def              "),
        "Should not have multiple spaces. Got: |{new_text}|"
    );
}
