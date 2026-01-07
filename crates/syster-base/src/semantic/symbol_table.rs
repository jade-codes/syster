/// Central registry of all named elements in a SysML/KerML model
mod lookup;
mod scope;
mod symbol;
mod table;

pub use scope::{Import, Scope};
pub use symbol::{Symbol, SymbolId, SymbolReference};
pub use table::SymbolTable;

#[cfg(test)]
mod tests;
