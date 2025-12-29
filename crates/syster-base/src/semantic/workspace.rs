//! # Workspace
//!
//! Manages multi-file SysML/KerML projects with shared symbol table and relationship graphs.
//!
//! Coordinates multiple source files, cross-file symbol resolution, and incremental updates
//! with automatic dependency invalidation for LSP implementations.

mod accessors;
mod core;
mod events;
mod file;
mod file_manager;
mod parsed_file;
mod population;
mod populator;

pub use core::Workspace;
pub use file::WorkspaceFile;
pub use parsed_file::ParsedFile;

#[cfg(test)]
#[path = "workspace/tests.rs"]
mod tests;

#[cfg(test)]
mod workspace_file_test;
