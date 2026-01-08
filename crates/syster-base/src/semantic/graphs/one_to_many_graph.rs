//! One-to-many directed graph (e.g., specialization, subsetting)
//!
//! Stores file paths with spans for O(1) reference lookups without symbol table queries.

use crate::core::{IStr, Span};
use std::collections::{HashMap, HashSet};

/// Reference location: file path + span
#[derive(Debug, Clone, PartialEq)]
pub struct RefLocation {
    pub file: IStr,
    pub span: Span,
}

#[derive(Debug, Clone, Default)]
pub struct OneToManyGraph {
    /// Forward relationships: source → [(target, location)]
    relationships: HashMap<IStr, Vec<(IStr, Option<RefLocation>)>>,
    /// Reverse index: target → [(source, location)] for O(1) reverse lookups
    reverse_index: HashMap<IStr, Vec<(IStr, Option<RefLocation>)>>,
    /// File index: file → [(source, target)] for O(1) file cleanup
    entries_by_file: HashMap<IStr, Vec<(IStr, IStr)>>,
}

impl OneToManyGraph {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a relationship from source to target with optional location info.
    pub fn add(&mut self, source: IStr, target: IStr, file: Option<IStr>, span: Option<Span>) {
        let location = match (&file, span) {
            (Some(f), Some(s)) => Some(RefLocation {
                file: f.clone(),
                span: s,
            }),
            _ => None,
        };

        let targets = self.relationships.entry(source.clone()).or_default();
        if !targets.iter().any(|(t, _)| t == &target) {
            targets.push((target.clone(), location.clone()));
            self.reverse_index
                .entry(target.clone())
                .or_default()
                .push((source.clone(), location));

            // Add to file index
            if let Some(f) = file {
                self.entries_by_file
                    .entry(f)
                    .or_default()
                    .push((source, target));
            }
        }
    }

    pub fn get_targets(&self, source: &str) -> Option<Vec<&IStr>> {
        self.relationships
            .get(source)
            .map(|v| v.iter().map(|(target, _)| target).collect())
    }

    pub fn get_targets_with_locations(
        &self,
        source: &str,
    ) -> Option<Vec<(&IStr, Option<&RefLocation>)>> {
        self.relationships.get(source).map(|v| {
            v.iter()
                .map(|(target, loc)| (target, loc.as_ref()))
                .collect()
        })
    }

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

    /// Count how many sources reference the given target.
    pub fn count_sources(&self, target: &str) -> usize {
        self.reverse_index.get(target).map(|v| v.len()).unwrap_or(0)
    }

    /// Remove all relationships where the given source is the origin
    pub fn remove_source(&mut self, source: &str) {
        if let Some(targets) = self.relationships.get(source) {
            for (target, _) in targets {
                if let Some(sources) = self.reverse_index.get_mut(target.as_ref()) {
                    sources.retain(|(s, _)| s.as_ref() != source);
                    if sources.is_empty() {
                        self.reverse_index.remove(target.as_ref());
                    }
                }
            }
        }
        self.relationships.remove(source);
    }

    /// Remove all relationships that have RefLocation pointing to the given file.
    /// O(1) lookup using file index.
    pub fn remove_by_file(&mut self, file_path: &str) {
        // Get entries from file index (O(1) lookup)
        let Some(entries) = self.entries_by_file.remove(file_path) else {
            return;
        };

        // Remove each (source, target) pair
        for (source, target) in entries {
            // Remove from forward index
            if let Some(targets) = self.relationships.get_mut(&source) {
                targets.retain(|(t, _)| t.as_ref() != target.as_ref());
                if targets.is_empty() {
                    self.relationships.remove(&source);
                }
            }

            // Remove from reverse index
            if let Some(sources) = self.reverse_index.get_mut(target.as_ref()) {
                sources.retain(|(s, _)| s.as_ref() != source.as_ref());
                if sources.is_empty() {
                    self.reverse_index.remove(target.as_ref());
                }
            }
        }
    }

    pub fn has_path(&self, from: &str, to: &str) -> bool {
        if from == to {
            return true;
        }

        let mut visited = HashSet::new();
        let mut stack = vec![from.to_string()];

        while let Some(current) = stack.pop() {
            if current == to {
                return true;
            }

            if !visited.insert(current.clone()) {
                continue;
            }

            if let Some(targets) = self.get_targets(&current) {
                for target in targets {
                    stack.push(target.to_string());
                }
            }
        }

        false
    }

    pub fn find_cycles(&self) -> Vec<Vec<String>> {
        let mut cycles = Vec::new();
        let mut visited = HashSet::new();
        let mut path = Vec::new();

        for start in self.relationships.keys() {
            if !visited.contains(start.as_ref()) {
                self.dfs_cycles(start, &mut visited, &mut path, &mut cycles);
            }
        }

        cycles
    }

    fn dfs_cycles(
        &self,
        node: &str,
        visited: &mut HashSet<String>,
        path: &mut Vec<String>,
        cycles: &mut Vec<Vec<String>>,
    ) {
        if path.contains(&node.to_string()) {
            if let Some(cycle_start) = path.iter().position(|n| n == node) {
                cycles.push(path[cycle_start..].to_vec());
            }
            return;
        }

        if visited.contains(node) {
            return;
        }

        path.push(node.to_string());

        if let Some(targets) = self.get_targets(node) {
            for target in targets {
                self.dfs_cycles(target, visited, path, cycles);
            }
        }

        visited.insert(node.to_string());
        path.pop();
    }

    pub fn has_circular_dependency(&self, element: &str) -> bool {
        let mut visited = HashSet::new();
        self.dfs_circular(element, element, &mut visited)
    }

    fn dfs_circular(&self, current: &str, target: &str, visited: &mut HashSet<String>) -> bool {
        if !visited.insert(current.to_string()) {
            return false;
        }

        if let Some(deps) = self.get_targets(current) {
            for dep in deps {
                if dep.as_ref() == target {
                    return true;
                }
                if self.dfs_circular(dep, target, visited) {
                    return true;
                }
            }
        }

        false
    }

    /// Iterate all entries: (source, target, optional location)
    pub fn all_entries(&self) -> impl Iterator<Item = (&IStr, &IStr, Option<&RefLocation>)> {
        self.relationships.iter().flat_map(|(source, targets)| {
            targets
                .iter()
                .map(move |(target, loc)| (source, target, loc.as_ref()))
        })
    }

    /// Update all targets using a resolver function.
    /// The resolver takes (source, old_target) and returns Some(new_target) if it should be updated.
    pub fn resolve_targets<F>(&mut self, mut resolver: F)
    where
        F: FnMut(&str, &str) -> Option<IStr>,
    {
        // Collect all updates first to avoid borrowing issues
        let mut updates: Vec<(IStr, IStr, IStr, Option<RefLocation>)> = Vec::new();

        for (source, targets) in &self.relationships {
            for (old_target, loc) in targets {
                if let Some(new_target) = resolver(source.as_ref(), old_target.as_ref()) {
                    updates.push((source.clone(), old_target.clone(), new_target, loc.clone()));
                }
            }
        }

        // Apply updates
        for (source, old_target, new_target, location) in updates {
            // Update forward index
            if let Some(targets) = self.relationships.get_mut(&source) {
                for (target, _loc) in targets.iter_mut() {
                    if target == &old_target {
                        *target = new_target.clone();
                    }
                }
            }

            // Update reverse index: remove from old, add to new
            if let Some(sources) = self.reverse_index.get_mut(&old_target) {
                sources.retain(|(s, _)| s != &source);
                if sources.is_empty() {
                    self.reverse_index.remove(&old_target);
                }
            }
            self.reverse_index
                .entry(new_target)
                .or_default()
                .push((source, location));
        }
    }
}
