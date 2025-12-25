//! One-to-many directed graph (e.g., specialization, subsetting)

use crate::core::Span;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Default)]
pub struct OneToManyGraph {
    relationships: HashMap<String, Vec<(String, Option<Span>)>>,
}

impl OneToManyGraph {
    pub fn new() -> Self {
        Self {
            relationships: HashMap::new(),
        }
    }

    pub fn add(&mut self, source: String, target: String, span: Option<Span>) {
        self.relationships
            .entry(source)
            .or_default()
            .push((target, span));
    }

    pub fn get_targets(&self, source: &str) -> Option<Vec<&String>> {
        self.relationships
            .get(source)
            .map(|v| v.iter().map(|(target, _)| target).collect())
    }

    pub fn get_targets_with_spans(&self, source: &str) -> Option<Vec<(&String, Option<&Span>)>> {
        self.relationships.get(source).map(|v| {
            v.iter()
                .map(|(target, span)| (target, span.as_ref()))
                .collect()
        })
    }

    pub fn get_sources(&self, target: &str) -> Vec<&String> {
        self.relationships
            .iter()
            .filter(|(_, targets)| targets.iter().any(|(t, _)| t == target))
            .map(|(source, _)| source)
            .collect()
    }

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
