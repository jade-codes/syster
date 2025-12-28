#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_literalnumber_eq_identical() {
        let val1 = LiteralNumber { literal_expression: LiteralExpression::default(), literal: 2.0 };
        let val2 = LiteralNumber { literal_expression: LiteralExpression::default(), literal: 2.0 };
        assert_eq!(val1, val2, "Identical instances should be equal");
    }

    #[test]
    fn test_literalnumber_ne_diff_literal_expression() {
        let val1 = LiteralNumber { literal_expression: LiteralExpression::default(), literal: 2.0 };
        let val2 = LiteralNumber { literal_expression: LiteralExpression::default(), literal: 2.0 };
        assert_ne!(val1, val2, "Instances with different literal_expression should not be equal");
    }

    #[test]
    fn test_literalnumber_ne_diff_literal() {
        let val1 = LiteralNumber { literal_expression: LiteralExpression::default(), literal: 2.0 };
        let val2 = LiteralNumber { literal_expression: LiteralExpression::default(), literal: 3.0 };
        assert_ne!(val1, val2, "Instances with different literal should not be equal");
    }
}
