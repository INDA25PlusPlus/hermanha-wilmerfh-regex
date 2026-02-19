use std::collections::HashMap;

use crate::expression::Expression;
use crate::nfa::{Edge, EdgeType, NFA};
use crate::utf_parser::{CodePoint, Parser};

#[derive(Clone)]
pub struct Matrix {
    matrix: Vec<Vec<bool>>,
}

impl Matrix {
    pub fn new(size: usize) -> Self {
        Self {
            matrix: vec![vec![false; size]; size],
        }
    }

    fn bool_mul(&self, input: &[bool]) -> Vec<bool> {
        let n = self.matrix.len();
        let mut reach = vec![false; n];

        for (idx, in_state) in input.iter().enumerate() {
            if !in_state {
                continue;
            }
            let row = &self.matrix[idx];
            let indicies = row
                .iter()
                .enumerate()
                .filter(|(_idx, b)| **b)
                .map(|(idx, _b)| idx);
            for index in indicies {
                reach[index] = true;
            }
        }

        reach
    }
}

pub struct Regex {
    NFA: NFA,
}

impl Regex {
    fn new(pattern: Vec<u8>) -> Self {
        let mut parser = Parser::from_bytes(pattern);
        let expression = Expression::parse(&mut parser);
        let mut nfa = expression.nfa();
        nfa.collapse_epsilons();
        Self { NFA: nfa }
    }

    fn accepts(&self, input: Vec<CodePoint>) -> bool {
        let adjacents = self.NFA.adjecents.clone();
        let n = adjacents.len();

        let neighbour_map = self.neighbour_list_to_neighbour_matrix(adjacents);
        let mut state = vec![false; n];
        state[self.NFA.start] = true;
        for code_point in input {
            let Some(matrix) = neighbour_map.get(&code_point) else {
                return false;
            };
            state = matrix.bool_mul(&state);
            if !state.iter().any(|x| *x) {
                return false;
            }
        }

        for &accepting in &self.NFA.accepting {
            if state[accepting] {
                return true;
            }
        }

        false
    }

    fn neighbour_list_to_neighbour_matrix(
        &self,
        neighbour_list: Vec<Vec<Edge>>,
    ) -> HashMap<CodePoint, Matrix> {
        let size = neighbour_list.len();
        let mut map: HashMap<CodePoint, Matrix> = HashMap::new();

        for (i, neighbors) in neighbour_list.into_iter().enumerate() {
            for edge in neighbors {
                let code_point = match edge.r#type {
                    EdgeType::Regular(code_point) => code_point,
                    EdgeType::Epsilon => panic!("not possible"),
                };
                let j = edge.to;

                let entry = map.entry(code_point).or_insert_with(|| Matrix::new(size));

                entry.matrix[i][j] = true;
            }
        }

        map
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utf_parser::bytes_to_codepoints;

    #[test]
    fn test_concatenation() {
        let regex = Regex::new("abc".as_bytes().to_vec());

        let abc = bytes_to_codepoints("abc".as_bytes().to_vec());
        assert!(regex.accepts(abc));
        let ab = bytes_to_codepoints("ab".as_bytes().to_vec());
        assert!(!regex.accepts(ab));
        let d = bytes_to_codepoints("d".as_bytes().to_vec());
        assert!(!regex.accepts(d));
    }

    #[test]
    fn test_concatenation_with_alternation() {
        let regex = Regex::new("ab|cd".as_bytes().to_vec());

        let ab = bytes_to_codepoints("ab".as_bytes().to_vec());
        assert!(!regex.accepts(ab));
        let cd = bytes_to_codepoints("cd".as_bytes().to_vec());
        assert!(!regex.accepts(cd));
        let ac = bytes_to_codepoints("ac".as_bytes().to_vec());
        assert!(!regex.accepts(ac));
        let abd = bytes_to_codepoints("abd".as_bytes().to_vec());
        assert!(regex.accepts(abd));
        let acd = bytes_to_codepoints("acd".as_bytes().to_vec());
        assert!(regex.accepts(acd));
    }

    #[test]
    fn test_grouped_alternation() {
        let regex = Regex::new("(ab)|(cd)".as_bytes().to_vec());

        let ab = bytes_to_codepoints("ab".as_bytes().to_vec());
        assert!(regex.accepts(ab));
        let cd = bytes_to_codepoints("cd".as_bytes().to_vec());
        assert!(regex.accepts(cd));
        let abd = bytes_to_codepoints("abd".as_bytes().to_vec());
        assert!(!regex.accepts(abd));
        let acd = bytes_to_codepoints("acd".as_bytes().to_vec());
        assert!(!regex.accepts(acd));
        let ac = bytes_to_codepoints("ac".as_bytes().to_vec());
        assert!(!regex.accepts(ac));
    }

    #[test]
    fn test_parenthesized_alternation_in_sequence() {
        let regex = Regex::new("a(b|c)d".as_bytes().to_vec());

        let abd = bytes_to_codepoints("abd".as_bytes().to_vec());
        assert!(regex.accepts(abd));
        let acd = bytes_to_codepoints("acd".as_bytes().to_vec());
        assert!(regex.accepts(acd));
        let ad = bytes_to_codepoints("ad".as_bytes().to_vec());
        assert!(!regex.accepts(ad));
        let abcd = bytes_to_codepoints("abcd".as_bytes().to_vec());
        assert!(!regex.accepts(abcd));
    }

    #[test]
    fn test_star_single_char() {
        let regex = Regex::new("a*".as_bytes().to_vec());

        let empty = bytes_to_codepoints("".as_bytes().to_vec());
        assert!(regex.accepts(empty));
        let a = bytes_to_codepoints("a".as_bytes().to_vec());
        assert!(regex.accepts(a));
        let aaa = bytes_to_codepoints("aaa".as_bytes().to_vec());
        assert!(regex.accepts(aaa));
        let b = bytes_to_codepoints("b".as_bytes().to_vec());
        assert!(!regex.accepts(b));
    }

    #[test]
    fn test_star_then_literal() {
        let regex = Regex::new("a*b".as_bytes().to_vec());

        let b = bytes_to_codepoints("b".as_bytes().to_vec());
        assert!(regex.accepts(b));
        let ab = bytes_to_codepoints("ab".as_bytes().to_vec());
        assert!(regex.accepts(ab));
        let aab = bytes_to_codepoints("aab".as_bytes().to_vec());
        assert!(regex.accepts(aab));
        let a = bytes_to_codepoints("a".as_bytes().to_vec());
        assert!(!regex.accepts(a));
        let ba = bytes_to_codepoints("ba".as_bytes().to_vec());
        assert!(!regex.accepts(ba));
    }

    #[test]
    fn test_star_group() {
        let regex = Regex::new("(ab)*".as_bytes().to_vec());

        let empty = bytes_to_codepoints("".as_bytes().to_vec());
        assert!(regex.accepts(empty));
        let ab = bytes_to_codepoints("ab".as_bytes().to_vec());
        assert!(regex.accepts(ab));
        let abab = bytes_to_codepoints("abab".as_bytes().to_vec());
        assert!(regex.accepts(abab));
        let a = bytes_to_codepoints("a".as_bytes().to_vec());
        assert!(!regex.accepts(a));
        let aba = bytes_to_codepoints("aba".as_bytes().to_vec());
        assert!(!regex.accepts(aba));
    }

    #[test]
    fn test_chained_alternation() {
        let regex_pattern = "a|b|c".to_string();
        let regex = Regex::new(regex_pattern.as_bytes().to_vec());

        let a = bytes_to_codepoints("a".to_string().as_bytes().to_vec());
        assert!(regex.accepts(a));
        let b = bytes_to_codepoints("b".to_string().as_bytes().to_vec());
        assert!(regex.accepts(b));
        let c = bytes_to_codepoints("c".to_string().as_bytes().to_vec());
        assert!(regex.accepts(c));
        let d = bytes_to_codepoints("d".to_string().as_bytes().to_vec());
        assert!(!regex.accepts(d));
    }
}
