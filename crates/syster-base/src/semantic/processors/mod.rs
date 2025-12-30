pub mod reference_collector;
pub mod semantic_token_collector;

pub use reference_collector::ReferenceCollector;
pub use semantic_token_collector::{SemanticToken, SemanticTokenCollector, TokenType};

#[cfg(test)]
mod tests;

#[cfg(test)]
mod semantic_token_text_extraction_test;

#[cfg(test)]
mod semantic_token_collector_semantictokencollector_test;

#[cfg(test)]
mod semantic_token_collector_normalize_path_test;
