use crate::nfa::{Edge, EdgeType, NFA};
use crate::utf_parser::CodePoint;

struct Literal {
    value: CodePoint,
}

impl Literal {
    fn build(&self, adjecents: &mut Vec<Vec<Edge>>) -> (usize, usize) {
        let start = adjecents.len();
        adjecents.push(vec![]);
        let end = adjecents.len();
        adjecents.push(vec![]);
        adjecents[start].push(Edge {
            to: end,
            r#type: EdgeType::Regular(self.value.clone()),
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
