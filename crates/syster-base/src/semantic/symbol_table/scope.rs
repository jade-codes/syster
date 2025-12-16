use super::symbol::Symbol;
use std::collections::HashMap;

/// Import declaration in a scope
#[derive(Debug, Clone)]
pub struct Import {
    pub path: String,
    pub is_recursive: bool,
    pub is_namespace: bool,
}

/// Represents a lexical scope in the symbol table
#[derive(Debug)]
pub(super) struct Scope {
    pub parent: Option<usize>,
    pub symbols: HashMap<String, Symbol>,
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
