use std::collections::HashSet;

use crate::nfa::{Edge, EdgeType, NFA};
use crate::utf_parser::{CodePoint, Parser};

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
    left: Box<Expression>,
    right: Box<Expression>,
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

struct Sequence {
    items: Vec<Expression>,
}

impl Sequence {
    fn build(&self, adjecents: &mut Vec<Vec<Edge>>) -> (usize, usize) {
        let (first_start, mut prev_end) = self.items[0].build(adjecents);
        for item in &self.items[1..] {
            let (start, end) = item.build(adjecents);
            adjecents[prev_end].push(Edge {
                to: start,
                r#type: EdgeType::Epsilon,
            });
            prev_end = end;
        }
        (first_start, prev_end)
    }
}

pub enum Expression {
    Literal(Literal),
    Alternation(Alternation),
    Sequence(Sequence),
}

impl Expression {
    pub fn parse(parser: &mut Parser) -> Self {
        let mut items = vec![];
        while parser.peek().is_some() && parser.peek() != Some(CodePoint::close_paren()) {
            items.push(Self::parse_unit(parser));
        }
        if items.len() == 1 {
            items.pop().unwrap()
        } else {
            Expression::Sequence(Sequence { items })
        }
    }

    fn parse_unit(parser: &mut Parser) -> Self {
        let left = if parser.peek() == Some(CodePoint::open_paren()) {
            parser.consume();
            let expr = Self::parse(parser);
            parser.consume();
            expr
        } else {
            let codepoint = parser.consume().expect("unexpected end of input");
            Expression::Literal(Literal { value: codepoint })
        };
        let next = parser.peek();
        if next == Some(CodePoint::pipe()) {
            parser.consume();
            let right = Self::parse_unit(parser);
            Expression::Alternation(Alternation {
                left: Box::new(left),
                right: Box::new(right),
            })
        } else {
            left
        }
    }

    fn build(&self, adjecents: &mut Vec<Vec<Edge>>) -> (usize, usize) {
        match self {
            Expression::Literal(l) => l.build(adjecents),
            Expression::Alternation(a) => a.build(adjecents),
            Expression::Sequence(s) => s.build(adjecents),
        }
    }

    pub fn nfa(&self) -> NFA {
        let mut adjecents = Vec::new();
        let (start, end) = self.build(&mut adjecents);
        NFA {
            adjecents,
            start,
            accepting: HashSet::from([end]),
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::utf_parser::bytes_to_codepoints;

//     #[test]
//     fn parse_single_literal() {
//         let mut parser = bytes_to_codepoints("a".as_bytes().to_vec());
//         let expression = Expression::parse(&mut parser);
//         assert!(matches!(expression, Expression::Literal(_)));
//     }

//     #[test]
//     fn parse_alternation() {
//         let mut parser = bytes_to_codepoints("a|b".as_bytes().to_vec());
//         let expression = Expression::parse(&mut parser);
//         assert!(matches!(expression, Expression::Alternation(_)));
//     }
// }
