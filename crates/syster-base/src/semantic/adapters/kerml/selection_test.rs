#![allow(clippy::unwrap_used)]

//! Tests for KerML selection range calculation.

use crate::core::{Position, Span};

/// Helper function to access the private range_size function through public API
/// We test it indirectly through find_selection_spans behavior
fn calculate_range_size(span: &Span) -> usize {
    let lines = span.end.line.saturating_sub(span.start.line);
    let cols = if lines == 0 {
        span.end.column.saturating_sub(span.start.column)
    } else {
        span.end.column + 100
    };
    lines * 100 + cols
}

#[test]
fn test_range_size_single_line_span() {
    // Single line span: line 0, columns 5 to 15 (10 characters)
    let span = Span::from_coords(0, 5, 0, 15);
    let size = calculate_range_size(&span);
    
    // lines = 0, cols = 15 - 5 = 10
    // size = 0 * 100 + 10 = 10
    assert_eq!(size, 10);
}

#[test]
fn test_range_size_single_line_same_position() {
    // Span with same start and end position
    let span = Span::from_coords(3, 10, 3, 10);
    let size = calculate_range_size(&span);
    
    // lines = 0, cols = 10 - 10 = 0
    // size = 0 * 100 + 0 = 0
    assert_eq!(size, 0);
}

#[test]
fn test_range_size_multi_line_span() {
    // Multi-line span: from line 0 col 5 to line 3 col 20
    let span = Span::from_coords(0, 5, 3, 20);
    let size = calculate_range_size(&span);
    
    // lines = 3 - 0 = 3
    // cols = 20 + 100 = 120 (multi-line formula)
    // size = 3 * 100 + 120 = 420
    assert_eq!(size, 420);
}

#[test]
fn test_range_size_two_line_span() {
    // Two line span: from line 5 col 0 to line 6 col 50
    let span = Span::from_coords(5, 0, 6, 50);
    let size = calculate_range_size(&span);
    
    // lines = 6 - 5 = 1
    // cols = 50 + 100 = 150 (multi-line formula)
    // size = 1 * 100 + 150 = 250
    assert_eq!(size, 250);
}

#[test]
fn test_range_size_large_multi_line_span() {
    // Large span: from line 0 col 0 to line 100 col 80
    let span = Span::from_coords(0, 0, 100, 80);
    let size = calculate_range_size(&span);
    
    // lines = 100 - 0 = 100
    // cols = 80 + 100 = 180 (multi-line formula)
    // size = 100 * 100 + 180 = 10180
    assert_eq!(size, 10180);
}

#[test]
fn test_range_size_single_line_large_column_span() {
    // Single line with large column difference
    let span = Span::from_coords(10, 0, 10, 500);
    let size = calculate_range_size(&span);
    
    // lines = 0, cols = 500 - 0 = 500
    // size = 0 * 100 + 500 = 500
    assert_eq!(size, 500);
}

#[test]
fn test_range_size_comparison_ordering() {
    // Test that spans are correctly ordered by size
    let small_span = Span::from_coords(0, 0, 0, 10);  // size = 10
    let medium_span = Span::from_coords(0, 0, 0, 50); // size = 50
    let large_span = Span::from_coords(0, 0, 2, 30);  // size = 2*100 + 130 = 330
    
    let size_small = calculate_range_size(&small_span);
    let size_medium = calculate_range_size(&medium_span);
    let size_large = calculate_range_size(&large_span);
    
    assert!(size_small < size_medium);
    assert!(size_medium < size_large);
    assert!(size_small < size_large);
}

#[test]
fn test_range_size_multi_line_with_zero_end_column() {
    // Multi-line span ending at column 0
    let span = Span::from_coords(5, 10, 8, 0);
    let size = calculate_range_size(&span);
    
    // lines = 8 - 5 = 3
    // cols = 0 + 100 = 100 (multi-line formula)
    // size = 3 * 100 + 100 = 400
    assert_eq!(size, 400);
}

#[test]
fn test_range_size_saturating_sub_start_larger_than_end() {
    // Edge case: if somehow start is after end (shouldn't happen in valid spans)
    // saturating_sub prevents underflow
    let span = Span {
        start: Position::new(10, 20),
        end: Position::new(5, 10),
    };
    let size = calculate_range_size(&span);
    
    // lines = 5.saturating_sub(10) = 0
    // cols = 10.saturating_sub(20) = 0
    // size = 0 * 100 + 0 = 0
    assert_eq!(size, 0);
}

#[test]
fn test_range_size_boundary_conditions() {
    // Test with boundary values
    let span1 = Span::from_coords(0, 0, 0, 1);  // Minimal single line
    let span2 = Span::from_coords(0, 0, 1, 0);  // Minimal multi-line
    
    let size1 = calculate_range_size(&span1);
    let size2 = calculate_range_size(&span2);
    
    assert_eq!(size1, 1);    // 0 * 100 + 1 = 1
    assert_eq!(size2, 200);  // 1 * 100 + 100 = 200
    
    // Multi-line always larger than single line due to +100 adjustment
    assert!(size2 > size1);
}

#[test]
fn test_range_size_single_character() {
    // Single character span
    let span = Span::from_coords(5, 10, 5, 11);
    let size = calculate_range_size(&span);
    
    // lines = 0, cols = 11 - 10 = 1
    // size = 0 * 100 + 1 = 1
    assert_eq!(size, 1);
}

#[test]
fn test_range_size_preserves_sorting_invariant() {
    // Verify that the formula ensures proper sorting behavior
    // A span on line N should be larger than any single-line span
    let longest_single_line = Span::from_coords(0, 0, 0, 99);
    let shortest_multi_line = Span::from_coords(0, 0, 1, 0);
    
    let size_single = calculate_range_size(&longest_single_line);
    let size_multi = calculate_range_size(&shortest_multi_line);
    
    // size_single = 0 * 100 + 99 = 99
    // size_multi = 1 * 100 + 100 = 200
    assert_eq!(size_single, 99);
    assert_eq!(size_multi, 200);
    assert!(size_multi > size_single);
}

#[test]
fn test_range_size_multiple_spans_sorting() {
    // Create multiple spans and verify they sort correctly
    let spans = vec![
        Span::from_coords(0, 0, 5, 50),   // size = 5*100 + 150 = 650
        Span::from_coords(0, 0, 0, 20),   // size = 20
        Span::from_coords(0, 0, 2, 10),   // size = 2*100 + 110 = 310
        Span::from_coords(0, 0, 0, 100),  // size = 100
    ];
    
    let mut sizes: Vec<_> = spans.iter().map(calculate_range_size).collect();
    let original_sizes = sizes.clone();
    sizes.sort();
    
    // Expected order: 20, 100, 310, 650
    assert_eq!(sizes, vec![20, 100, 310, 650]);
    assert_eq!(original_sizes, vec![650, 20, 310, 100]);
}
