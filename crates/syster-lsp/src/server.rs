mod completion;
mod core;
mod definition;
mod diagnostics;
mod document;
mod document_symbols;
mod folding_range;
pub mod formatting;
mod helpers;
mod hover;
mod inlay_hints;
mod position;
mod references;
mod rename;
mod selection_range;
mod semantic_tokens;

pub mod background_tasks;

pub use core::LspServer;

#[cfg(test)]
#[path = "server/tests.rs"]
mod tests;

#[cfg(test)]
#[path = "server/lsp_server_state_test.rs"]
mod lsp_server_state_test;

#[cfg(test)]
#[path = "server/helpers_format_symbol_declaration_test.rs"]
mod helpers_format_symbol_declaration_test;
