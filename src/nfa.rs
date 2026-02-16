use std::collections::HashSet;

use crate::utf_parser::CodePoint;

#[derive(Clone)]
pub enum EdgeType {
    Regular(CodePoint),
    Epsilon,
}

#[derive(Clone)]
pub struct Edge {
    pub to: usize,
    pub r#type: EdgeType,
}

pub struct NFA {
    pub adjecents: Vec<Vec<Edge>>,
    pub start: usize,
    pub end: usize,
}

impl NFA {
    fn epsilon_closure(&self, node: usize) -> HashSet<usize> {
        let mut closure = HashSet::new();
        self.epsilon_closure_inner(node, &mut closure);
        closure
    }

    fn epsilon_closure_inner(&self, node: usize, closure: &mut HashSet<usize>) {
        closure.insert(node);

        let epsilon_targets: Vec<usize> = self.adjecents[node]
            .iter()
            .filter(|edge| matches!(edge.r#type, EdgeType::Epsilon))
            .map(|edge| edge.to)
            .collect();

        for target in epsilon_targets {
            if !closure.contains(&target) {
                self.epsilon_closure_inner(target, closure);
            }
        }
    }
}
