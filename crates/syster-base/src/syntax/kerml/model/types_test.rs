#[cfg(test)]
mod tests {
    use crate::syntax::kerml::model::types::*;

    fn create_literal_expression() -> LiteralExpression {
        LiteralExpression {
            expression: Expression {
                step: Step {
                    feature: Feature {
                        type_: Type {
                            namespace: Namespace {
                                element: Element {
                                    declared_name: None,
                                    declared_short_name: None,
                                },
                                prefixes: vec![],
                                children: vec![],
                            },
                            is_sufficient: false,
                            is_abstract: None,
                            heritage: vec![],
                            type_relationships: vec![],
                            multiplicity: None,
                        },
                        is_nonunique: false,
                        is_ordered: false,
                        direction: None,
                        is_composite: None,
                        is_derived: None,
                        is_end: None,
                        is_portion: None,
                        is_readonly: None,
                        value: None,
                        write: None,
                        crossing_feature: None,
                    },
                },
                result: None,
            },
        }
    }

    fn create_literal_expression_with_name(name: &str) -> LiteralExpression {
        LiteralExpression {
            expression: Expression {
                step: Step {
                    feature: Feature {
                        type_: Type {
                            namespace: Namespace {
                                element: Element {
                                    declared_name: Some(name.to_string()),
                                    declared_short_name: None,
                                },
                                prefixes: vec![],
                                children: vec![],
                            },
                            is_sufficient: false,
                            is_abstract: None,
                            heritage: vec![],
                            type_relationships: vec![],
                            multiplicity: None,
                        },
                        is_nonunique: false,
                        is_ordered: false,
                        direction: None,
                        is_composite: None,
                        is_derived: None,
                        is_end: None,
                        is_portion: None,
                        is_readonly: None,
                        value: None,
                        write: None,
                        crossing_feature: None,
                    },
                },
                result: None,
            },
        }
    }

    #[test]
    fn test_literalnumber_eq_identical() {
        let val1 = LiteralNumber {
            literal_expression: create_literal_expression(),
            literal: 42.0,
        };
        let val2 = LiteralNumber {
            literal_expression: create_literal_expression(),
            literal: 42.0,
        };
        assert_eq!(val1, val2, "Identical instances should be equal");
    }

    #[test]
    fn test_literalnumber_ne_diff_literal_expression() {
        let val1 = LiteralNumber {
            literal_expression: create_literal_expression(),
            literal: 42.0,
        };
        let val2 = LiteralNumber {
            literal_expression: create_literal_expression_with_name("different"),
            literal: 42.0,
        };
        assert_ne!(
            val1, val2,
            "Instances with different literal_expression should not be equal"
        );
    }

    #[test]
    fn test_literalnumber_ne_diff_literal() {
        let val1 = LiteralNumber {
            literal_expression: create_literal_expression(),
            literal: 42.0,
        };
        let val2 = LiteralNumber {
            literal_expression: create_literal_expression(),
            literal: 43.0,
        };
        assert_ne!(
            val1, val2,
            "Instances with different literal should not be equal"
        );
    }

    #[test]
    fn test_literalnumber_eq_negative_numbers() {
        let val1 = LiteralNumber {
            literal_expression: create_literal_expression(),
            literal: -99.5,
        };
        let val2 = LiteralNumber {
            literal_expression: create_literal_expression(),
            literal: -99.5,
        };
        assert_eq!(val1, val2, "Negative numbers should be equal");
    }

    #[test]
    fn test_literalnumber_eq_zero() {
        let val1 = LiteralNumber {
            literal_expression: create_literal_expression(),
            literal: 0.0,
        };
        let val2 = LiteralNumber {
            literal_expression: create_literal_expression(),
            literal: 0.0,
        };
        assert_eq!(val1, val2, "Zero should equal zero");
    }
}
