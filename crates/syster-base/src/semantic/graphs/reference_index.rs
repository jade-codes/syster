//! Bidirectional index for references with span information.
//!
//! Stores qualified names along with their reference spans and file paths.
//! Enables both:
//! - "Find References": given a target, find all sources that reference it
//! - "Find Specializations": given a source, find all targets it references

use crate::core::Span;
use crate::semantic::types::TokenType;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

/// A single reference from a source symbol to a target
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReferenceInfo {
    /// Qualified name of the symbol that contains this reference
    pub source_qname: String,
    /// File containing the reference
    pub file: PathBuf,
    /// Span of the reference (where the target name appears)
    pub span: Span,
    /// Optional token type for semantic highlighting.
    /// If None, defaults to TokenType::Type when generating semantic tokens.
    pub token_type: Option<TokenType>,
}

/// Entry in the reverse index: all references to a target
#[derive(Debug, Clone, Default)]
struct ReferenceEntry {
    /// All references to this target (deduplicated via HashSet)
    references: HashSet<ReferenceInfo>,
}

/// Bidirectional index for references.
///
/// Stores references with their spans for accurate "Find References" results.
/// Also supports forward lookups (source → targets) for hover and documentation.
#[derive(Debug, Clone, Default)]
pub struct ReferenceIndex {
    /// Reverse index: target_name → references to it
    reverse: HashMap<String, ReferenceEntry>,

    /// Forward index: source_qname → targets it references
    forward: HashMap<String, HashSet<String>>,

    /// Track which sources came from which file (for cleanup on file change)
    source_to_file: HashMap<String, PathBuf>,
}

impl ReferenceIndex {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a reference from source to target with span information.
    ///
    /// # Arguments
    /// * `source_qname` - Qualified name of the symbol that has the reference
    /// * `target_name` - Name of the target (may be simple or qualified)
    /// * `source_file` - File containing the reference
    /// * `span` - Location of the reference in source code
    pub fn add_reference(
        &mut self,
        source_qname: &str,
        target_name: &str,
        source_file: Option<&PathBuf>,
        span: Option<Span>,
    ) {
        self.add_reference_with_type(source_qname, target_name, source_file, span, None);
    }

    /// Add a reference with an explicit token type for semantic highlighting.
    ///
    /// Use this when the reference target's token type differs from the default (Type).
    /// For example, redefinition and subsetting targets should use Property.
    ///
    /// # Arguments
    /// * `source_qname` - Qualified name of the symbol that has the reference
    /// * `target_name` - Name of the target (may be simple or qualified)
    /// * `source_file` - File containing the reference
    /// * `span` - Location of the reference in source code
    /// * `token_type` - Token type for semantic highlighting (None defaults to Type)
    pub fn add_reference_with_type(
        &mut self,
        source_qname: &str,
        target_name: &str,
        source_file: Option<&PathBuf>,
        span: Option<Span>,
        token_type: Option<TokenType>,
    ) {
        // Only add if we have both file and span
        if let (Some(file), Some(span)) = (source_file, span) {
            let info = ReferenceInfo {
                source_qname: source_qname.to_string(),
                file: file.clone(),
                span,
                token_type,
            };

            // Add to reverse index (target → sources)
            self.reverse
                .entry(target_name.to_string())
                .or_default()
                .references
                .insert(info);

            // Add to forward index (source → targets)
            self.forward
                .entry(source_qname.to_string())
                .or_default()
                .insert(target_name.to_string());

            // Track file for cleanup
            self.source_to_file
                .insert(source_qname.to_string(), file.clone());
        }
    }

    /// Get all references to a target with their span information.
    ///
    /// Returns references with file paths and spans for accurate location reporting.
    pub fn get_references(&self, target: &str) -> Vec<&ReferenceInfo> {
        self.reverse
            .get(target)
            .map(|entry| entry.references.iter().collect())
            .unwrap_or_default()
    }

    /// Get all targets that a source references (forward lookup).
    ///
    /// Returns the qualified names of all symbols that this source references.
    /// Useful for showing specializations in hover.
    pub fn get_targets(&self, source_qname: &str) -> Vec<&str> {
        self.forward
            .get(source_qname)
            .map(|targets| targets.iter().map(|s| s.as_str()).collect())
            .unwrap_or_default()
    }

    /// Get all sources that reference a target (qualified names only).
    ///
    /// Returns the qualified names of all symbols that reference this target.
    pub fn get_sources(&self, target: &str) -> Vec<&str> {
        self.reverse
            .get(target)
            .map(|entry| {
                entry
                    .references
                    .iter()
                    .map(|r| r.source_qname.as_str())
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Check if a target has any references.
    pub fn has_references(&self, target: &str) -> bool {
        self.reverse
            .get(target)
            .map(|entry| !entry.references.is_empty())
            .unwrap_or(false)
    }

    /// Get all targets that have references.
    ///
    /// Useful for debugging and testing.
    pub fn targets(&self) -> Vec<&str> {
        self.reverse.keys().map(|s| s.as_str()).collect()
    }

    /// Remove all references from symbols in the given file.
    ///
    /// Called when a file is modified or deleted to invalidate stale references.
    pub fn remove_references_from_file(&mut self, file_path: &str) {
        let path = PathBuf::from(file_path);

        // Remove references that came from this file
        for entry in self.reverse.values_mut() {
            entry.references.retain(|r| r.file != path);
        }

        // Find all sources from this file and remove from tracking
        let sources_to_remove: Vec<String> = self
            .source_to_file
            .iter()
            .filter(|(_, f)| *f == &path)
            .map(|(s, _)| s.clone())
            .collect();

        for source in &sources_to_remove {
            self.source_to_file.remove(source);
            self.forward.remove(source);
        }

        // Clean up empty entries
        self.reverse.retain(|_, entry| !entry.references.is_empty());
    }

    /// Remove all references where the given qualified name is the source.
    pub fn remove_source(&mut self, source_qname: &str) {
        // Remove from source_to_file
        self.source_to_file.remove(source_qname);

        // Remove from forward index
        self.forward.remove(source_qname);

        // Remove references from this source
        for entry in self.reverse.values_mut() {
            entry.references.retain(|r| r.source_qname != source_qname);
        }

        // Clean up empty entries
        self.reverse.retain(|_, entry| !entry.references.is_empty());
    }

    /// Clear all entries.
    pub fn clear(&mut self) {
        self.reverse.clear();
        self.forward.clear();
        self.source_to_file.clear();
    }

    /// Get the number of unique targets.
    pub fn target_count(&self) -> usize {
        self.reverse.len()
    }

    /// Get the total number of references.
    pub fn reference_count(&self) -> usize {
        self.reverse.values().map(|e| e.references.len()).sum()
    }

    /// Get all references that occur in a specific file.
    ///
    /// Returns references with their spans for semantic token highlighting.
    pub fn get_references_in_file(&self, file_path: &str) -> Vec<&ReferenceInfo> {
        let path = PathBuf::from(file_path);
        self.reverse
            .values()
            .flat_map(|entry| entry.references.iter())
            .filter(|r| r.file == path)
            .collect()
    }

    /// Re-resolve simple reference targets to qualified names using the provided resolver.
    ///
    /// Called after import resolution to update references that were stored with simple names
    /// during population (before imports were resolved).
    ///
    /// # Arguments
    /// * `resolve_fn` - A function that takes (simple_name, source_file) and returns Option<qualified_name>
    pub fn resolve_targets<F>(&mut self, mut resolve_fn: F)
    where
        F: FnMut(&str, &PathBuf) -> Option<String>,
    {
        // Collect references that need to be re-indexed under a new target name
        let mut updates: Vec<(String, String, ReferenceInfo)> = Vec::new();

        for (target_name, entry) in &self.reverse {
            // Only process simple names (no ::)
            if target_name.contains("::") {
                continue;
            }

            for ref_info in &entry.references {
                if let Some(qualified_name) = resolve_fn(target_name, &ref_info.file) {
                    // Only update if the resolved name is different
                    if &qualified_name != target_name {
                        updates.push((target_name.clone(), qualified_name, ref_info.clone()));
                    }
                }
            }
        }

        // Apply the updates
        for (old_target, new_target, ref_info) in updates {
            // Remove from old target
            if let Some(entry) = self.reverse.get_mut(&old_target) {
                entry.references.remove(&ref_info);
            }

            // Add to new target
            self.reverse
                .entry(new_target.clone())
                .or_default()
                .references
                .insert(ref_info.clone());

            // Update forward index
            if let Some(targets) = self.forward.get_mut(&ref_info.source_qname) {
                targets.remove(&old_target);
                targets.insert(new_target);
            }
        }

        // Clean up empty entries
        self.reverse.retain(|_, entry| !entry.references.is_empty());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Position;

    fn test_span() -> Span {
        Span::new(Position::new(0, 0), Position::new(0, 10))
    }

    #[test]
    fn test_add_and_get_references() {
        let mut index = ReferenceIndex::new();
        let file = PathBuf::from("test.sysml");

        index.add_reference("Car", "Vehicle", Some(&file), Some(test_span()));
        index.add_reference("Truck", "Vehicle", Some(&file), Some(test_span()));

        let refs = index.get_references("Vehicle");
        assert_eq!(refs.len(), 2);

        let sources: Vec<&str> = refs.iter().map(|r| r.source_qname.as_str()).collect();
        assert!(sources.contains(&"Car"));
        assert!(sources.contains(&"Truck"));
    }

    #[test]
    fn test_get_sources() {
        let mut index = ReferenceIndex::new();
        let file = PathBuf::from("test.sysml");

        index.add_reference("Car", "Vehicle", Some(&file), Some(test_span()));
        index.add_reference("Truck", "Vehicle", Some(&file), Some(test_span()));

        let sources = index.get_sources("Vehicle");
        assert_eq!(sources.len(), 2);
        assert!(sources.contains(&"Car"));
        assert!(sources.contains(&"Truck"));
    }

    #[test]
    fn test_get_sources_empty() {
        let index = ReferenceIndex::new();
        let sources = index.get_sources("NonExistent");
        assert!(sources.is_empty());
    }

    #[test]
    fn test_remove_references_from_file() {
        let mut index = ReferenceIndex::new();
        let file_a = PathBuf::from("a.sysml");
        let file_b = PathBuf::from("b.sysml");

        index.add_reference("Car", "Vehicle", Some(&file_a), Some(test_span()));
        index.add_reference("Truck", "Vehicle", Some(&file_b), Some(test_span()));

        index.remove_references_from_file(file_a.to_str().unwrap());

        let sources = index.get_sources("Vehicle");
        assert_eq!(sources.len(), 1);
        assert!(sources.contains(&"Truck"));
    }

    #[test]
    fn test_remove_source() {
        let mut index = ReferenceIndex::new();
        let file = PathBuf::from("test.sysml");

        index.add_reference("Car", "Vehicle", Some(&file), Some(test_span()));
        index.add_reference("Car", "Engine", Some(&file), Some(test_span()));

        index.remove_source("Car");

        assert!(!index.has_references("Vehicle"));
        assert!(!index.has_references("Engine"));
    }

    #[test]
    fn test_reference_count() {
        let mut index = ReferenceIndex::new();
        let file = PathBuf::from("test.sysml");

        index.add_reference("Car", "Vehicle", Some(&file), Some(test_span()));
        index.add_reference("Car", "Engine", Some(&file), Some(test_span()));
        index.add_reference("Truck", "Vehicle", Some(&file), Some(test_span()));

        assert_eq!(index.target_count(), 2); // Vehicle, Engine
        assert_eq!(index.reference_count(), 3); // Car→Vehicle, Car→Engine, Truck→Vehicle
    }
}
