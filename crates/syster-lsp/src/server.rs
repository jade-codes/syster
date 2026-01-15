mod code_lens;
mod completion;
mod core;
mod definition;
mod diagnostics;
pub mod diagram;
mod document;
mod document_links;
mod document_symbols;
mod folding_ranges;
pub mod formatting;
pub mod helpers;
mod hover;
mod inlay_hints;
mod position;
mod references;
mod rename;
mod selection_range;
mod semantic_tokens;
mod workspace_symbols;

pub mod background_tasks;

pub use core::LspServer;

#[cfg(test)]
mod tests;
