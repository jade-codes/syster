use super::LspServer;
use super::helpers::{span_to_lsp_range, uri_to_path};
use async_lsp::lsp_types::{
    CallHierarchyIncomingCall, CallHierarchyIncomingCallsParams, CallHierarchyItem,
    CallHierarchyOutgoingCall, CallHierarchyOutgoingCallsParams, CallHierarchyPrepareParams,
    SymbolKind, Url,
};
use syster::core::constants::REL_PERFORM;

impl LspServer {
    /// Prepare call hierarchy for a symbol at the given position
    ///
    /// This finds the callable symbol (action definition or usage) at the cursor position
    /// and returns a CallHierarchyItem that can be used for finding callers/callees.
    pub fn prepare_call_hierarchy(
        &self,
        params: &CallHierarchyPrepareParams,
    ) -> Option<Vec<CallHierarchyItem>> {
        let uri = &params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;

        let path = uri_to_path(uri)?;
        let (element_qname, _) = self.find_symbol_at_position(&path, position)?;

        // Look up symbol in workspace
        let symbol = self.workspace.symbol_table().resolve(&element_qname)?;

        // Only actions and functions are callable
        let is_callable = matches!(
            symbol,
            syster::semantic::symbol_table::Symbol::Definition { kind, .. }
                if kind == "Action" || kind == "function"
        ) || matches!(
            symbol,
            syster::semantic::symbol_table::Symbol::Usage { kind, .. }
                if kind == "Action"
        );

        if !is_callable {
            return None;
        }

        // Get definition location
        let source_file = symbol.source_file()?;
        let span = symbol.span()?;
        let def_uri = Url::from_file_path(source_file).ok()?;

        let item = CallHierarchyItem {
            name: symbol.name().to_string(),
            kind: SymbolKind::FUNCTION,
            tags: None,
            detail: Some(element_qname.clone()),
            uri: def_uri,
            range: span_to_lsp_range(&span),
            selection_range: span_to_lsp_range(&span),
            data: None,
        };

        Some(vec![item])
    }

    /// Find incoming calls to the given callable
    ///
    /// Returns all places where this action/function is performed/called.
    pub fn incoming_calls(
        &self,
        params: &CallHierarchyIncomingCallsParams,
    ) -> Option<Vec<CallHierarchyIncomingCall>> {
        let item = &params.item;
        let target_qname = item.detail.as_ref()?;

        // Find all sources that perform this target
        let callers = self
            .workspace
            .relationship_graph()
            .get_one_to_many_sources(REL_PERFORM, target_qname);

        if callers.is_empty() {
            return Some(Vec::new());
        }

        let mut incoming_calls = Vec::new();

        for caller_qname in callers {
            // Look up the caller symbol
            let Some(caller_symbol) = self.workspace.symbol_table().lookup_qualified(caller_qname)
            else {
                continue;
            };

            // Get caller's location
            let Some(source_file) = caller_symbol.source_file() else {
                continue;
            };
            let Some(span) = caller_symbol.span() else {
                continue;
            };
            let Ok(caller_uri) = Url::from_file_path(source_file) else {
                continue;
            };

            // Create CallHierarchyItem for the caller
            let from_item = CallHierarchyItem {
                name: caller_symbol.name().to_string(),
                kind: SymbolKind::FUNCTION,
                tags: None,
                detail: Some(caller_qname.to_string()),
                uri: caller_uri,
                range: span_to_lsp_range(&span),
                selection_range: span_to_lsp_range(&span),
                data: None,
            };

            // Get the ranges where this caller performs the target
            let from_ranges = self
                .workspace
                .relationship_graph()
                .get_one_to_many_with_spans(REL_PERFORM, caller_qname)
                .map(|targets| {
                    targets
                        .into_iter()
                        .filter(|(target, _)| target == &target_qname)
                        .filter_map(|(_, span)| span.map(span_to_lsp_range))
                        .collect()
                })
                .unwrap_or_default();

            incoming_calls.push(CallHierarchyIncomingCall {
                from: from_item,
                from_ranges,
            });
        }

        Some(incoming_calls)
    }

    /// Find outgoing calls from the given callable
    ///
    /// Returns all actions/functions that are performed/called by this callable.
    pub fn outgoing_calls(
        &self,
        params: &CallHierarchyOutgoingCallsParams,
    ) -> Option<Vec<CallHierarchyOutgoingCall>> {
        let item = &params.item;
        let source_qname = item.detail.as_ref()?;

        // Find all targets that this source performs
        let callees = self
            .workspace
            .relationship_graph()
            .get_one_to_many_with_spans(REL_PERFORM, source_qname);

        // If no callees, return empty vec (not None)
        let callees = match callees {
            Some(c) => c,
            None => return Some(Vec::new()),
        };

        let mut outgoing_calls = Vec::new();

        for (callee_qname, call_span) in callees {
            // Look up the callee symbol
            let Some(callee_symbol) = self.workspace.symbol_table().lookup_qualified(callee_qname)
            else {
                continue;
            };

            // Get callee's location
            let Some(source_file) = callee_symbol.source_file() else {
                continue;
            };
            let Some(span) = callee_symbol.span() else {
                continue;
            };
            let Ok(callee_uri) = Url::from_file_path(source_file) else {
                continue;
            };

            // Create CallHierarchyItem for the callee
            let to_item = CallHierarchyItem {
                name: callee_symbol.name().to_string(),
                kind: SymbolKind::FUNCTION,
                tags: None,
                detail: Some(callee_qname.to_string()),
                uri: callee_uri,
                range: span_to_lsp_range(&span),
                selection_range: span_to_lsp_range(&span),
                data: None,
            };

            // The range where this call happens
            let from_ranges = if let Some(span) = call_span {
                vec![span_to_lsp_range(span)]
            } else {
                vec![]
            };

            outgoing_calls.push(CallHierarchyOutgoingCall {
                to: to_item,
                from_ranges,
            });
        }

        Some(outgoing_calls)
    }
}
