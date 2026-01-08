//! Main relationship graph that aggregates different graph types
//!
//! Uses interned strings (IStr) for efficient storage.
//! Stores file paths with spans for O(1) reference lookups.

use super::{OneToManyGraph, OneToOneGraph, RefLocation, SymmetricGraph};
use crate::core::constants::relationship_label;
use crate::core::{IStr, Interner, Span};
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct RelationshipGraph {
    interner: Interner,
    one_to_many: HashMap<String, OneToManyGraph>,
    one_to_one: HashMap<String, OneToOneGraph>,
    symmetric: HashMap<String, SymmetricGraph>,
}

impl RelationshipGraph {
    pub fn new() -> Self {
        Self::default()
    }

    /// Intern a string, returning a cheap-to-clone reference.
    fn intern(&mut self, s: &str) -> IStr {
        self.interner.intern(s)
    }

    /// Remove all relationships where the given source is the origin.
    pub fn remove_relationships_for_source(&mut self, source: &str) {
        for graph in self.one_to_many.values_mut() {
            graph.remove_source(source);
        }
        for graph in self.one_to_one.values_mut() {
            graph.remove_source(source);
        }
        for graph in self.symmetric.values_mut() {
            graph.remove_element(source);
        }
    }

    /// Remove all relationships that have RefLocation pointing to the given file.
    /// This clears entries from the reverse index that were stored with this file path.
    pub fn remove_relationships_for_file(&mut self, file_path: &str) {
        for graph in self.one_to_many.values_mut() {
            graph.remove_by_file(file_path);
        }
        for graph in self.one_to_one.values_mut() {
            graph.remove_by_file(file_path);
        }
    }

    pub fn add_one_to_many(
        &mut self,
        relationship_type: &str,
        source: &str,
        target: &str,
        file: Option<&str>,
        span: Option<Span>,
    ) {
        let source = self.intern(source);
        let target = self.intern(target);
        let file = file.map(|f| self.intern(f));
        self.one_to_many
            .entry(relationship_type.to_string())
            .or_default()
            .add(source, target, file, span);
    }

    pub fn get_one_to_many(&self, relationship_type: &str, source: &str) -> Option<Vec<&str>> {
        self.one_to_many
            .get(relationship_type)
            .and_then(|g| g.get_targets(source))
            .map(|v| v.into_iter().map(|s| s.as_ref()).collect())
    }

    pub fn get_one_to_many_with_locations(
        &self,
        relationship_type: &str,
        source: &str,
    ) -> Option<Vec<(&str, Option<&RefLocation>)>> {
        self.one_to_many
            .get(relationship_type)
            .and_then(|g| g.get_targets_with_locations(source))
            .map(|v| v.into_iter().map(|(s, loc)| (s.as_ref(), loc)).collect())
    }

    pub fn get_one_to_many_sources(&self, relationship_type: &str, target: &str) -> Vec<&str> {
        self.one_to_many
            .get(relationship_type)
            .map(|g| {
                g.get_sources(target)
                    .into_iter()
                    .map(|s| s.as_ref())
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn add_one_to_one(
        &mut self,
        relationship_type: &str,
        source: &str,
        target: &str,
        file: Option<&str>,
        span: Option<Span>,
    ) {
        let source = self.intern(source);
        let target = self.intern(target);
        let file = file.map(|f| self.intern(f));
        self.one_to_one
            .entry(relationship_type.to_string())
            .or_default()
            .add(source, target, file, span);
    }

    pub fn get_one_to_one(&self, relationship_type: &str, source: &str) -> Option<&str> {
        self.one_to_one
            .get(relationship_type)
            .and_then(|g| g.get_target(source))
            .map(|s| s.as_ref())
    }

    pub fn get_one_to_one_with_location(
        &self,
        relationship_type: &str,
        source: &str,
    ) -> Option<(&str, Option<&RefLocation>)> {
        self.one_to_one
            .get(relationship_type)
            .and_then(|g| g.get_target_with_location(source))
            .map(|(s, loc)| (s.as_ref(), loc))
    }

    pub fn add_symmetric(&mut self, relationship_type: &str, element1: String, element2: String) {
        self.symmetric
            .entry(relationship_type.to_string())
            .or_default()
            .add(element1, element2);
    }

    pub fn get_symmetric(&self, relationship_type: &str, element: &str) -> Option<&[String]> {
        self.symmetric
            .get(relationship_type)
            .and_then(|g| g.get_related(element))
    }

    pub fn has_transitive_path(&self, relationship_type: &str, from: &str, to: &str) -> bool {
        self.one_to_many
            .get(relationship_type)
            .map(|g| g.has_path(from, to))
            .unwrap_or(false)
    }

    pub fn relationship_types(&self) -> Vec<String> {
        let mut types = Vec::new();
        types.extend(self.one_to_many.keys().cloned());
        types.extend(self.one_to_one.keys().cloned());
        types.extend(self.symmetric.keys().cloned());
        types.sort();
        types.dedup();
        types
    }

    /// Get all relationships for a given element.
    pub fn get_all_relationships(&self, element: &str) -> Vec<(String, Vec<String>)> {
        self.one_to_many
            .iter()
            .filter_map(|(rel_type, graph)| {
                graph.get_targets(element).map(|targets| {
                    let targets_vec: Vec<String> = targets.iter().map(|s| s.to_string()).collect();
                    (rel_type.clone(), targets_vec)
                })
            })
            .chain(self.one_to_one.iter().filter_map(|(rel_type, graph)| {
                graph
                    .get_target(element)
                    .map(|target| (rel_type.clone(), vec![target.to_string()]))
            }))
            .chain(self.symmetric.iter().filter_map(|(rel_type, graph)| {
                graph
                    .get_related(element)
                    .map(|related| (rel_type.clone(), related.to_vec()))
            }))
            .collect()
    }

    /// Get all relationships formatted for display.
    pub fn get_formatted_relationships(&self, element: &str) -> Vec<String> {
        self.get_all_relationships(element)
            .into_iter()
            .flat_map(|(rel_type, targets)| {
                let label = relationship_label(&rel_type).to_string();
                targets
                    .into_iter()
                    .map(move |target| format!("{label} `{target}`"))
            })
            .collect()
    }

    /// Get all relationships grouped by type.
    pub fn get_relationships_grouped(&self, element: &str) -> Vec<(String, Vec<String>)> {
        self.get_all_relationships(element)
            .into_iter()
            .map(|(rel_type, targets)| {
                let label = relationship_label(&rel_type).to_string();
                (label, targets)
            })
            .collect()
    }

    /// Get all sources that reference a given target symbol.
    /// Returns (file, span) pairs directly - no symbol table lookup needed.
    /// O(1) lookup using reverse indexes.
    pub fn get_references_to(&self, target: &str) -> Vec<(&str, &Span)> {
        let mut refs = Vec::new();

        for graph in self.one_to_many.values() {
            let sources = graph.get_sources_with_locations(target);
            for (_source, loc) in sources {
                if let Some(location) = loc {
                    refs.push((location.file.as_ref(), &location.span));
                }
            }
        }

        for graph in self.one_to_one.values() {
            let sources = graph.get_sources_with_locations(target);
            for (_source, loc) in sources {
                if let Some(location) = loc {
                    refs.push((location.file.as_ref(), &location.span));
                }
            }
        }

        refs
    }

    /// Iterate all one-to-many entries with their locations.
    /// Returns (relationship_type, source, target, file, span) for each entry with a location.
    pub fn all_one_to_many_entries_with_locations(
        &self,
    ) -> impl Iterator<Item = (&str, &str, &str, &str, &Span)> {
        self.one_to_many.iter().flat_map(|(rel_type, graph)| {
            graph
                .all_entries()
                .filter_map(move |(source, target, loc)| {
                    loc.map(|l| {
                        (
                            rel_type.as_str(),
                            source.as_ref(),
                            target.as_ref(),
                            l.file.as_ref(),
                            &l.span,
                        )
                    })
                })
        })
    }

    /// Iterate all one-to-one entries with their locations.
    /// Returns (relationship_type, source, target, file, span) for each entry with a location.
    pub fn all_one_to_one_entries_with_locations(
        &self,
    ) -> impl Iterator<Item = (&str, &str, &str, &str, &Span)> {
        self.one_to_one.iter().flat_map(|(rel_type, graph)| {
            graph
                .all_entries_with_locations()
                .filter_map(move |(source, target, loc)| {
                    loc.map(|l| {
                        (
                            rel_type.as_str(),
                            source.as_ref(),
                            target.as_ref(),
                            l.file.as_ref(),
                            &l.span,
                        )
                    })
                })
        })
    }

    /// Find what symbol is referenced at a given position in a file.
    /// Returns the resolved qualified name if a reference exists at that position.
    pub fn get_binding_at_position(&self, file: &str, line: usize, column: usize) -> Option<&str> {
        use crate::core::Position;
        let pos = Position::new(line, column);

        // Check one-to-one relationships (e.g., typing)
        for graph in self.one_to_one.values() {
            for (_source, target, loc) in graph.all_entries_with_locations() {
                if let Some(location) = loc {
                    if location.file.as_ref() == file && location.span.contains(pos) {
                        return Some(target.as_ref());
                    }
                }
            }
        }

        // Check one-to-many relationships (e.g., specialization)
        for graph in self.one_to_many.values() {
            for (_source, target, loc) in graph.all_entries() {
                if let Some(location) = loc {
                    if location.file.as_ref() == file && location.span.contains(pos) {
                        return Some(target.as_ref());
                    }
                }
            }
        }

        None
    }

    /// Resolve all targets in a one-to-one relationship using the provided resolver function.
    /// The resolver takes (source_qualified_name, unresolved_target) and returns the resolved qualified name if found.
    pub fn resolve_targets<F>(&mut self, relationship_type: &str, resolver: F)
    where
        F: Fn(&str, &str) -> Option<String>,
    {
        // First pass: collect updates from the graph
        let updates: Vec<_> = {
            let Some(graph) = self.one_to_one.get(relationship_type) else {
                return;
            };
            graph
                .all_entries()
                .filter_map(|(source, target)| {
                    resolver(source.as_ref(), target.as_ref())
                        .map(|resolved| (source.clone(), resolved))
                })
                .collect()
        };

        // Intern all new targets
        let interned_updates: Vec<_> = updates
            .into_iter()
            .map(|(source, new_target)| {
                let interned = self.intern(&new_target);
                (source, interned)
            })
            .collect();

        // Apply updates to the graph
        if let Some(graph) = self.one_to_one.get_mut(relationship_type) {
            for (source, new_target_interned) in interned_updates {
                graph.update_target(&source, new_target_interned);
            }
        }
    }

    /// Resolve all targets in a one-to-many relationship using the provided resolver function.
    /// The resolver takes (source_qualified_name, unresolved_target) and returns the resolved qualified name if found.
    pub fn resolve_one_to_many_targets<F>(&mut self, relationship_type: &str, resolver: F)
    where
        F: Fn(&str, &str) -> Option<String>,
    {
        let interner = &mut self.interner;
        if let Some(graph) = self.one_to_many.get_mut(relationship_type) {
            graph.resolve_targets(|source, target| {
                resolver(source, target).map(|resolved| interner.intern(&resolved))
            });
        }
    }
}
