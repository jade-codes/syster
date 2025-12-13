use std::collections::{HashMap, HashSet};

// ============================================================================
// Low-Level Graph Primitives
// ============================================================================

/// Generic one-to-many relationship graph
/// Maps from source element to multiple target elements
#[derive(Debug, Clone, Default)]
pub struct OneToManyGraph {
    relationships: HashMap<String, Vec<String>>,
}

impl OneToManyGraph {
    pub fn new() -> Self {
        Self {
            relationships: HashMap::new(),
        }
    }

    /// Add a relationship from source to target
    pub fn add(&mut self, source: String, target: String) {
        self.relationships.entry(source).or_default().push(target);
    }

    /// Get all targets for a source
    pub fn get_targets(&self, source: &str) -> Option<&[String]> {
        self.relationships.get(source).map(|v| v.as_slice())
    }

    /// Get all sources that point to a given target (reverse lookup)
    pub fn get_sources(&self, target: &str) -> Vec<&String> {
        self.relationships
            .iter()
            .filter(|(_, targets)| targets.iter().any(|t| t == target))
            .map(|(source, _)| source)
            .collect()
    }

    /// Check if there's a path from source to target (transitive)
    pub fn has_path(&self, from: &str, to: &str) -> bool {
        if from == to {
            return true;
        }

        let mut visited = HashSet::new();
        let mut stack = vec![from];

        while let Some(current) = stack.pop() {
            if current == to {
                return true;
            }

            if !visited.insert(current) {
                continue;
            }

            if let Some(targets) = self.get_targets(current) {
                for target in targets {
                    stack.push(target);
                }
            }
        }

        false
    }

    /// Find all cycles in the graph
    pub fn find_cycles(&self) -> Vec<Vec<String>> {
        let mut cycles = Vec::new();
        let mut visited = HashSet::new();
        let mut path = Vec::new();

        for start in self.relationships.keys() {
            if !visited.contains(start.as_str()) {
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

    /// Check if there's a circular dependency starting from a given node
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
                if dep == target {
                    return true;
                }
                if self.dfs_circular(dep, target, visited) {
                    return true;
                }
            }
        }

        false
    }
}

/// Generic one-to-one relationship graph
/// Maps from source element to a single target element
#[derive(Debug, Clone, Default)]
pub struct OneToOneGraph {
    relationships: HashMap<String, String>,
}

impl OneToOneGraph {
    pub fn new() -> Self {
        Self {
            relationships: HashMap::new(),
        }
    }

    /// Add or update a relationship
    pub fn add(&mut self, source: String, target: String) {
        self.relationships.insert(source, target);
    }

    /// Get the target for a source
    pub fn get_target(&self, source: &str) -> Option<&String> {
        self.relationships.get(source)
    }

    /// Check if a source has a relationship
    pub fn has_relationship(&self, source: &str) -> bool {
        self.relationships.contains_key(source)
    }

    /// Get all sources that point to a given target (reverse lookup)
    pub fn get_sources(&self, target: &str) -> Vec<&String> {
        self.relationships
            .iter()
            .filter(|(_, t)| t.as_str() == target)
            .map(|(s, _)| s)
            .collect()
    }
}

/// Generic symmetric relationship graph (bidirectional)
/// Used for relationships where if A relates to B, then B relates to A
#[derive(Debug, Clone, Default)]
pub struct SymmetricGraph {
    relationships: HashMap<String, Vec<String>>,
}

impl SymmetricGraph {
    pub fn new() -> Self {
        Self {
            relationships: HashMap::new(),
        }
    }

    /// Add a symmetric relationship (adds both directions)
    pub fn add(&mut self, element1: String, element2: String) {
        self.relationships
            .entry(element1.clone())
            .or_default()
            .push(element2.clone());
        self.relationships
            .entry(element2)
            .or_default()
            .push(element1);
    }

    /// Get all elements related to the given element
    pub fn get_related(&self, element: &str) -> Option<&[String]> {
        self.relationships.get(element).map(|v| v.as_slice())
    }

    /// Check if two elements are related
    pub fn are_related(&self, element1: &str, element2: &str) -> bool {
        self.relationships
            .get(element1)
            .is_some_and(|related| related.iter().any(|e| e == element2))
    }
}

// ============================================================================
// Generic Relationship Graph
// ============================================================================

/// Generic relationship graph that stores named relationships between symbols
/// Each relationship type is identified by a string key
#[derive(Debug, Clone, Default)]
pub struct RelationshipGraph {
    /// One-to-many relationships (e.g., specialization, subsetting, etc.)
    one_to_many: HashMap<String, OneToManyGraph>,

    /// One-to-one relationships (e.g., typing, conjugation, etc.)
    one_to_one: HashMap<String, OneToOneGraph>,

    /// Symmetric relationships (e.g., disjointing)
    symmetric: HashMap<String, SymmetricGraph>,
}
impl RelationshipGraph {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a one-to-many relationship
    pub fn add_one_to_many(&mut self, relationship_type: &str, source: String, target: String) {
        self.one_to_many
            .entry(relationship_type.to_string())
            .or_insert_with(OneToManyGraph::new)
            .add(source, target);
    }

    /// Get targets for a one-to-many relationship
    pub fn get_one_to_many(&self, relationship_type: &str, source: &str) -> Option<&[String]> {
        self.one_to_many
            .get(relationship_type)
            .and_then(|g| g.get_targets(source))
    }

    /// Get sources for a one-to-many relationship (reverse lookup)
    pub fn get_one_to_many_sources(&self, relationship_type: &str, target: &str) -> Vec<&String> {
        self.one_to_many
            .get(relationship_type)
            .map(|g| g.get_sources(target))
            .unwrap_or_default()
    }

    /// Add a one-to-one relationship
    pub fn add_one_to_one(&mut self, relationship_type: &str, source: String, target: String) {
        self.one_to_one
            .entry(relationship_type.to_string())
            .or_insert_with(OneToOneGraph::new)
            .add(source, target);
    }

    /// Get target for a one-to-one relationship
    pub fn get_one_to_one(&self, relationship_type: &str, source: &str) -> Option<&String> {
        self.one_to_one
            .get(relationship_type)
            .and_then(|g| g.get_target(source))
    }

    /// Add a symmetric relationship
    pub fn add_symmetric(&mut self, relationship_type: &str, element1: String, element2: String) {
        self.symmetric
            .entry(relationship_type.to_string())
            .or_insert_with(SymmetricGraph::new)
            .add(element1, element2);
    }

    /// Get related elements for a symmetric relationship
    pub fn get_symmetric(&self, relationship_type: &str, element: &str) -> Option<&[String]> {
        self.symmetric
            .get(relationship_type)
            .and_then(|g| g.get_related(element))
    }

    /// Check if there's a transitive path in a one-to-many relationship
    pub fn has_transitive_path(&self, relationship_type: &str, from: &str, to: &str) -> bool {
        self.one_to_many
            .get(relationship_type)
            .map(|g| g.has_path(from, to))
            .unwrap_or(false)
    }

    /// Get all relationship types stored in this graph
    pub fn relationship_types(&self) -> Vec<String> {
        let mut types = Vec::new();
        types.extend(self.one_to_many.keys().cloned());
        types.extend(self.one_to_one.keys().cloned());
        types.extend(self.symmetric.keys().cloned());
        types.sort();
        types.dedup();
        types
    }
}

#[cfg(test)]
#[path = "graph/tests.rs"]
mod tests;
