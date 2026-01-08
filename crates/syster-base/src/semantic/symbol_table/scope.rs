use super::symbol::SymbolId;
use crate::core::Span;
use std::collections::HashMap;

/// Import declaration in a scope
#[derive(Debug, Clone)]
pub struct Import {
    pub path: String,
    pub is_recursive: bool,
    pub is_namespace: bool,
    pub is_public: bool,
    pub span: Option<Span>,
    pub file: Option<String>,
}

/// Represents a lexical scope in the symbol table
#[derive(Debug)]
pub struct Scope {
    pub parent: Option<usize>,
    /// Maps symbol name to SymbolId (symbols are stored in arena)
    pub symbols: HashMap<String, SymbolId>,
    pub children: Vec<usize>,
    pub imports: Vec<Import>,
}

impl Scope {
    pub fn new(parent: Option<usize>) -> Self {
        Self {
            parent,
            symbols: HashMap::new(),
            children: Vec::new(),
            imports: Vec::new(),
        }
    }
}
