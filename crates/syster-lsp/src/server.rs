mod completion;
mod core;
mod definition;
mod diagnostics;
mod document;
mod document_symbols;
mod folding_range;
mod helpers;
mod hover;
mod position;
mod references;
mod rename;
mod selection_range;
mod semantic_tokens;

pub use core::LspServer;

#[cfg(test)]
#[path = "server/tests.rs"]
mod tests;
