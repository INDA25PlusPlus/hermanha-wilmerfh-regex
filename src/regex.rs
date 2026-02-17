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

enum Expression {
    Literal(Literal),
    Alternation(Alternation),
}

impl Expression {
    fn parse(parser: &mut Parser) -> Self {
        let first_codepoint = parser.consume().expect("unexpected end of input");

        let Some(next) = parser.peek() else {
            return Expression::Literal(Literal {
                value: first_codepoint,
            });
        };

        if next == CodePoint::pipe() {
            parser.consume();
            let rest = Self::parse(parser);
            Expression::Alternation(Alternation {
                left: Box::new(Expression::Literal(Literal {
                    value: first_codepoint,
                })),
                right: Box::new(rest),
            })
        } else {
            Expression::Literal(Literal {
                value: first_codepoint,
            })
        }
    }

    fn build(&self, adjecents: &mut Vec<Vec<Edge>>) -> (usize, usize) {
        match self {
            Expression::Literal(l) => l.build(adjecents),
            Expression::Alternation(a) => a.build(adjecents),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utf_parser::bytes_to_codepoints;

    #[test]
    fn parse_single_literal() {
        let mut parser = bytes_to_codepoints("a".as_bytes().to_vec());
        let expression = Expression::parse(&mut parser);
        assert!(matches!(expression, Expression::Literal(_)));
    }

    #[test]
    fn parse_alternation() {
        let mut parser = bytes_to_codepoints("a|b".as_bytes().to_vec());
        let expression = Expression::parse(&mut parser);
        assert!(matches!(expression, Expression::Alternation(_)));
    }
}
