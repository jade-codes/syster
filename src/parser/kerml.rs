use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parser/kerml.pest"]
pub struct KerMLParser;
