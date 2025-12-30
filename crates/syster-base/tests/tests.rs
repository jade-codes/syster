#[path = "parser/mod.rs"]
mod parser;

#[path = "semantic/mod.rs"]
mod semantic;

#[path = "core/mod.rs"]
mod core;

#[path = "syntax/mod.rs"]
mod syntax;

#[cfg(test)]
mod core_parse_result_test;
