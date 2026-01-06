//! One-to-one directed graph (e.g., typing relationship)
//!
//! Stores file paths with spans for O(1) reference lookups without symbol table queries.

use crate::core::{IStr, Span};
use std::collections::HashMap;

use super::RefLocation;

#[derive(Debug, Clone, Default)]
pub struct OneToOneGraph {
    /// Forward relationships: source → (target, location)
    relationships: HashMap<IStr, (IStr, Option<RefLocation>)>,
    /// Reverse index: target → [(source, location)] for O(1) reverse lookups
    reverse_index: HashMap<IStr, Vec<(IStr, Option<RefLocation>)>>,
    /// File index: file → [sources] for O(1) file cleanup
    sources_by_file: HashMap<IStr, Vec<IStr>>,
}

impl OneToOneGraph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, source: IStr, target: IStr, file: Option<IStr>, span: Option<Span>) {
        let location = match (&file, span) {
            (Some(f), Some(s)) => Some(RefLocation {
                file: f.clone(),
                span: s,
            }),
            _ => None,
        };

        // Remove old reverse index entry if source already had a target
        if let Some((old_target, _)) = self.relationships.get(&source)
            && let Some(sources) = self.reverse_index.get_mut(old_target)
        {
            sources.retain(|(s, _)| s != &source);
            if sources.is_empty() {
                self.reverse_index.remove(old_target);
            }
        }

        // Add to forward relationships
        self.relationships
            .insert(source.clone(), (target.clone(), location.clone()));

        // Add to reverse index
        self.reverse_index
            .entry(target)
            .or_default()
            .push((source.clone(), location));

        // Add to file index
        if let Some(f) = file {
            self.sources_by_file.entry(f).or_default().push(source);
        }
    }

    pub fn get_target(&self, source: &str) -> Option<&IStr> {
        self.relationships.get(source).map(|(target, _)| target)
    }

    pub fn get_target_with_location(&self, source: &str) -> Option<(&IStr, Option<&RefLocation>)> {
        self.relationships
            .get(source)
            .map(|(target, loc)| (target, loc.as_ref()))
    }

    pub fn has_relationship(&self, source: &str) -> bool {
        self.relationships.contains_key(source)
    }

    /// Get all sources that reference the given target.
    pub fn get_sources(&self, target: &str) -> Vec<&IStr> {
        self.reverse_index
            .get(target)
            .map(|v| v.iter().map(|(source, _)| source).collect())
            .unwrap_or_default()
    }

    /// Get all sources that reference the given target, with their locations.
    /// O(1) lookup using reverse index.
    pub fn get_sources_with_locations(&self, target: &str) -> Vec<(&IStr, Option<&RefLocation>)> {
        self.reverse_index
            .get(target)
            .map(|v| {
                v.iter()
                    .map(|(source, loc)| (source, loc.as_ref()))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Remove the relationship where the given source is the origin
    pub fn remove_source(&mut self, source: &str) {
        if let Some((target, _)) = self.relationships.get(source)
            && let Some(sources) = self.reverse_index.get_mut(target.as_ref())
        {
            sources.retain(|(s, _)| s.as_ref() != source);
            if sources.is_empty() {
                self.reverse_index.remove(target.as_ref());
            }
        }
        self.relationships.remove(source);
    }

    /// Remove all relationships that have RefLocation pointing to the given file.
    /// O(1) lookup using file index.
    pub fn remove_by_file(&mut self, file_path: &str) {
        // Get sources from file index (O(1) lookup)
        let Some(sources) = self.sources_by_file.remove(file_path) else {
            return;
        };

        // Remove each source
        for source in sources {
            if let Some((target, _)) = self.relationships.remove(&source) {
                // Update reverse index
                if let Some(rev_sources) = self.reverse_index.get_mut(target.as_ref()) {
                    rev_sources.retain(|(s, _)| s.as_ref() != source.as_ref());
                    if rev_sources.is_empty() {
                        self.reverse_index.remove(target.as_ref());
                    }
                }
            }
        }
    }

    /// Iterate all (source, target) pairs
    pub fn all_entries(&self) -> impl Iterator<Item = (&IStr, &IStr)> {
        self.relationships
            .iter()
            .map(|(source, (target, _))| (source, target))
    }

    /// Update a target to a new resolved value, fixing the reverse index
    pub fn update_target(&mut self, source: &IStr, new_target: IStr) {
        if let Some((old_target, location)) = self.relationships.get(source).cloned() {
            // Remove from old reverse index
            if let Some(sources) = self.reverse_index.get_mut(&old_target) {
                sources.retain(|(s, _)| s != source);
                if sources.is_empty() {
                    self.reverse_index.remove(&old_target);
                }
            }

            // Update forward relationship
            self.relationships
                .insert(source.clone(), (new_target.clone(), location.clone()));

            // Add to new reverse index
            self.reverse_index
                .entry(new_target)
                .or_default()
                .push((source.clone(), location));
        }
    }
}
