#[path = "parser/mod.rs"]
mod parser;

#[path = "semantic/mod.rs"]
mod semantic;

#[cfg(test)]
mod batch_1_test;

#[cfg(test)]
mod core_parse_result_test;
