pub mod semantic_token_collector;

pub use semantic_token_collector::{SemanticToken, SemanticTokenCollector, TokenType};

#[cfg(test)]
mod tests;
