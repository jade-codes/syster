use syster::core::{Position, Span};

#[test]
fn test_span_contains_position() {
    let span = Span::from_coords(5, 10, 5, 20);

    // Inside
    assert!(span.contains(Position::new(5, 15)));
    assert!(span.contains(Position::new(5, 10))); // Start boundary
    assert!(span.contains(Position::new(5, 20))); // End boundary

    // Outside
    assert!(!span.contains(Position::new(4, 15))); // Before line
    assert!(!span.contains(Position::new(6, 15))); // After line
    assert!(!span.contains(Position::new(5, 9))); // Before column
    assert!(!span.contains(Position::new(5, 21))); // After column
}

#[test]
fn test_span_multiline() {
    let span = Span::from_coords(5, 10, 7, 5);

    assert!(span.contains(Position::new(5, 15))); // First line
    assert!(span.contains(Position::new(6, 0))); // Middle line
    assert!(span.contains(Position::new(7, 3))); // Last line

    assert!(!span.contains(Position::new(5, 9))); // Before start
    assert!(!span.contains(Position::new(7, 6))); // After end
}
