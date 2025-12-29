//! SysML v2 Language Server Library
//!
//! Provides LSP functionality for SysML v2 that can be used by the binary server
//! or tested independently.

pub mod server;

pub use server::LspServer;
pub use server::formatting;
