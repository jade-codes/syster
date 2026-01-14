use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parser/kerml_expressions.pest"]
#[grammar = "parser/sysml.pest"]
pub struct SysMLParser;

#[cfg(test)]
mod tests {
    use super::*;
    use pest::Parser;

    #[test]
    fn test_parser_compiles() {
        let result = SysMLParser::parse(Rule::file, "");
        assert!(result.is_ok());
    }
}
