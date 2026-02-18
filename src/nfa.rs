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
    pub accepting: HashSet<usize>,
}

impl NFA {
    pub fn epsilon_closure(&self, node: usize) -> HashSet<usize> {
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

    pub fn epsilon_closures(&self) -> impl Iterator<Item = HashSet<usize>> + '_ {
        (0..self.adjecents.len()).map(|node| self.epsilon_closure(node))
    }

    fn accepting_nodes(&self) -> HashSet<usize> {
        self.epsilon_closures()
            .enumerate()
            .filter(|(_, closure)| closure.iter().any(|n| self.accepting.contains(n)))
            .map(|(node, _)| node)
            .collect()
    }

    fn lift_edges(&mut self) {
        let closures: Vec<HashSet<usize>> = self.epsilon_closures().collect();
        let mut adjecents: Vec<Vec<Edge>> = vec![vec![]; closures.len()];
        for (n, closure) in closures.iter().enumerate() {
            for &m in closure {
                for edge in &self.adjecents[m] {
                    if matches!(edge.r#type, EdgeType::Regular(_)) {
                        adjecents[n].push(edge.clone());
                    }
                }
            }
        }
        self.adjecents = adjecents;
    }

    pub fn collapse_epsilons(&mut self) {
        self.accepting = self.accepting_nodes();
        self.lift_edges();
    }
}
