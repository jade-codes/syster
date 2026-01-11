use crate::core::Span;
use crate::semantic::graphs::ReferenceIndex;
use crate::semantic::symbol_table::SymbolTable;
use crate::semantic::types::SemanticError;

pub struct KermlAdapter<'a> {
    pub(super) symbol_table: &'a mut SymbolTable,
    pub(super) reference_index: Option<&'a mut ReferenceIndex>,
    pub(super) current_namespace: Vec<String>,
    pub(super) errors: Vec<SemanticError>,
}

impl<'a> KermlAdapter<'a> {
    pub fn new(symbol_table: &'a mut SymbolTable) -> Self {
        Self {
            symbol_table,
            reference_index: None,
            current_namespace: Vec::new(),
            errors: Vec::new(),
        }
    }

    pub fn with_index(
        symbol_table: &'a mut SymbolTable,
        reference_index: &'a mut ReferenceIndex,
    ) -> Self {
        Self {
            symbol_table,
            reference_index: Some(reference_index),
            current_namespace: Vec::new(),
            errors: Vec::new(),
        }
    }

    /// Index a reference from source to target for reverse lookups
    pub(super) fn index_reference(&mut self, source_qname: &str, target: &str, span: Option<Span>) {
        if let Some(index) = &mut self.reference_index {
            let file = self.symbol_table.current_file().map(PathBuf::from);
            index.add_reference(source_qname, target, file.as_ref(), span);
        }
    }
}

use std::path::PathBuf;
