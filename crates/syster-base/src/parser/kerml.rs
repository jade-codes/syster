use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parser/kerml_expressions.pest"]
#[grammar = "parser/kerml.pest"]
pub struct KerMLParser;
