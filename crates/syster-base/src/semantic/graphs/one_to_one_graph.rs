//! One-to-one directed graph (e.g., typing relationship)

use crate::core::Span;
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct OneToOneGraph {
    relationships: HashMap<String, (String, Option<Span>)>,
}

impl OneToOneGraph {
    pub fn new() -> Self {
        Self {
            relationships: HashMap::new(),
        }
    }

    pub fn add(&mut self, source: String, target: String, span: Option<Span>) {
        self.relationships.insert(source, (target, span));
    }

    pub fn get_target(&self, source: &str) -> Option<&String> {
        self.relationships.get(source).map(|(target, _)| target)
    }

    pub fn get_target_with_span(&self, source: &str) -> Option<(&String, Option<&Span>)> {
        self.relationships
            .get(source)
            .map(|(target, span)| (target, span.as_ref()))
    }

    pub fn has_relationship(&self, source: &str) -> bool {
        self.relationships.contains_key(source)
    }

    pub fn get_sources(&self, target: &str) -> Vec<&String> {
        self.relationships
            .iter()
            .filter(|(_, (t, _))| t.as_str() == target)
            .map(|(s, _)| s)
            .collect()
    }
}
