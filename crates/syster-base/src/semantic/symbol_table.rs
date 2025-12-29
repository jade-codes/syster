/// Central registry of all named elements in a SysML/KerML model
mod lookup;
mod scope;
mod symbol;
mod table;

pub use scope::Import;
pub use symbol::{Symbol, SymbolReference};
pub use table::SymbolTable;

#[path = "symbol_table/build_scope_chain_test.rs"]
mod build_scope_chain_test;
#[path = "symbol_table/find_in_scope_chain_test.rs"]
mod find_in_scope_chain_test;
#[path = "symbol_table/lookup_global_mut_test.rs"]
mod lookup_global_mut_test;
#[cfg(test)]
#[path = "symbol_table/tests.rs"]
mod tests;
