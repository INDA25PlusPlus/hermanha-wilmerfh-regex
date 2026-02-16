use std::collections::HashSet;

struct Literal {
    value: char,
}

impl Literal {
    fn build(&self, adjecents: &mut Vec<Vec<Edge>>) -> (usize, usize) {
        let start = adjecents.len();
        adjecents.push(vec![]);
        let end = adjecents.len();
        adjecents.push(vec![]);
        adjecents[start].push(Edge {
            to: end,
            r#type: EdgeType::Regular(self.value),
        });
        (start, end)
    }
}

struct Alternation {
    left: Box<Regex>,
    right: Box<Regex>,
}

impl Alternation {
    fn build(&self, adjecents: &mut Vec<Vec<Edge>>) -> (usize, usize) {
        let start = adjecents.len();
        adjecents.push(vec![]);

        let (left_start, left_end) = self.left.build(adjecents);
        let (right_start, right_end) = self.right.build(adjecents);

        let end = adjecents.len();
        adjecents.push(vec![]);

        adjecents[start].push(Edge {
            to: left_start,
            r#type: EdgeType::Epsilon,
        });
        adjecents[start].push(Edge {
            to: right_start,
            r#type: EdgeType::Epsilon,
        });
        adjecents[left_end].push(Edge {
            to: end,
            r#type: EdgeType::Epsilon,
        });
        adjecents[right_end].push(Edge {
            to: end,
            r#type: EdgeType::Epsilon,
        });

        (start, end)
    }
}

enum Regex {
    Literal(Literal),
    Alternation(Alternation),
}

impl Regex {
    fn build(&self, adjecents: &mut Vec<Vec<Edge>>) -> (usize, usize) {
        match self {
            Regex::Literal(l) => l.build(adjecents),
            Regex::Alternation(a) => a.build(adjecents),
        }
    }

    fn nfa(&self) -> NFA {
        let mut adjecents = Vec::new();
        let (start, end) = self.build(&mut adjecents);
        NFA {
            adjecents,
            start,
            end,
        }
    }
}

#[derive(Clone, Copy)]
enum EdgeType {
    Regular(char),
    Epsilon,
}

#[derive(Clone, Copy)]
struct Edge {
    to: usize,
    r#type: EdgeType,
}

struct NFA {
    adjecents: Vec<Vec<Edge>>,
    start: usize,
    end: usize,
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
