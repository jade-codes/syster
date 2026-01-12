use crate::core::Span;
use crate::semantic::graphs::ReferenceIndex;
use crate::semantic::symbol_table::SymbolTable;
use crate::semantic::types::SemanticError;
use std::path::PathBuf;

pub struct SysmlAdapter<'a> {
    pub(super) symbol_table: &'a mut SymbolTable,
    pub(super) reference_index: Option<&'a mut ReferenceIndex>,
    pub(super) current_namespace: Vec<String>,
    pub(super) errors: Vec<SemanticError>,
}

impl<'a> SysmlAdapter<'a> {
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

    /// Index a reference from source to target for reverse lookups.
    ///
    /// The target is resolved to its full qualified name by trying:
    /// 1. The target as-is (already fully qualified)
    /// 2. Prefixed with the current namespace (relative to current scope)
    /// 3. Walking up the namespace chain to find the symbol
    pub(super) fn index_reference(&mut self, source_qname: &str, target: &str, span: Option<Span>) {
        self.index_reference_with_type(source_qname, target, span, None);
    }

    /// Index a reference with an explicit token type for semantic highlighting.
    ///
    /// Use this for references where the default Type token is not appropriate:
    /// - Property for redefines/subsets targets (they reference usages/features)
    /// - Type for typed_by targets (they reference definitions/classifiers)
    pub(super) fn index_reference_with_type(
        &mut self,
        source_qname: &str,
        target: &str,
        span: Option<Span>,
        token_type: Option<crate::semantic::types::TokenType>,
    ) {
        // First resolve the target (needs immutable borrow of self)
        let resolved_target = self.resolve_reference_target(target);
        let file = self.symbol_table.current_file().map(PathBuf::from);

        // Then add to index (needs mutable borrow of reference_index)
        if let Some(index) = &mut self.reference_index {
            index.add_reference_with_type(
                source_qname,
                &resolved_target,
                file.as_ref(),
                span,
                token_type,
            );
        }
    }

    /// Resolve a reference target to its full qualified name.
    ///
    /// Tries multiple resolution strategies:
    /// 1. Check if target already exists as a fully qualified name
    /// 2. Try prefixing with current namespace parts (walking up the chain)
    fn resolve_reference_target(&self, target: &str) -> String {
        // Strategy 1: Check if it's already a valid fully qualified name
        if self.symbol_table.find_by_qualified_name(target).is_some() {
            return target.to_string();
        }

        // Strategy 2: Try prefixing with current namespace, walking up the chain
        // e.g., if we're in Outer and target is Inner::Vehicle,
        // try Outer::Inner::Vehicle
        let mut namespace = self.current_namespace.clone();
        while !namespace.is_empty() {
            let candidate = format!("{}::{}", namespace.join("::"), target);
            if self
                .symbol_table
                .find_by_qualified_name(&candidate)
                .is_some()
            {
                return candidate;
            }
            namespace.pop();
        }

        // Strategy 3: If target contains ::, try resolving just the first segment
        // and build the full path from there
        if let Some(first_segment) = target.split("::").next() {
            let mut ns = self.current_namespace.clone();
            while !ns.is_empty() {
                let candidate_prefix = format!("{}::{}", ns.join("::"), first_segment);
                if self
                    .symbol_table
                    .find_by_qualified_name(&candidate_prefix)
                    .is_some()
                {
                    // Found the first segment, now build the full path
                    let full_target = format!("{}::{}", ns.join("::"), target);
                    if self
                        .symbol_table
                        .find_by_qualified_name(&full_target)
                        .is_some()
                    {
                        return full_target;
                    }
                }
                ns.pop();
            }
        }

        // Fallback: return original target (unresolved references)
        target.to_string()
    }
}
