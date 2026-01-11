pub mod semantic_token_collector;

pub use crate::semantic::types::TokenType;
pub use semantic_token_collector::{SemanticToken, SemanticTokenCollector};

#[cfg(test)]
mod tests;
