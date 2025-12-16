/// Central registry of all named elements in a SysML/KerML model
mod lookup;
mod scope;
mod symbol;
mod table;

pub use scope::Import;
pub use symbol::{Symbol, SymbolReference};
pub use table::SymbolTable;

#[cfg(test)]
#[path = "symbol_table/tests.rs"]
mod tests;
