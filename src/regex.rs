use std::collections::HashMap;

use crate::nfa::{Edge, EdgeType, NFA};
use crate::utf_parser::{CodePoint, Parser};
use crate::expression::Expression;


#[derive(Clone)]
pub struct Matrix {
    matrix: Vec<Vec<bool>>
}

impl Matrix {
    pub fn new(size: usize) -> Self {
        Self { matrix: vec![vec![false; size]; size] }
    }

    fn bool_mul(&self, input: &[bool]) -> Vec<bool> {
        let n = self.matrix.len();
        let mut reach = vec![false; n];

        for (idx, in_state) in input.iter().enumerate() {
            if !in_state {
                continue;
            }
            let row = &self.matrix[idx];
            let indicies = row.iter().enumerate().filter(|(_idx, b)| **b).map(|(idx, _b)| idx);
            for index in indicies {
                reach[index] = true;
            }

        }

        reach
    }
}

pub struct Regex {
    NFA: NFA
}

impl Regex {
    fn new(pattern: Vec<u8>) -> Self {
        let mut parser = Parser::from_bytes(pattern);
        let expression = Expression::parse(&mut parser);
        let mut nfa  = expression.nfa();
        nfa.collapse_epsilons();
        Self{NFA: nfa}
    }  

    fn accepts(&self, input: Vec<CodePoint>) -> bool {
        let adjacents = self.NFA.adjecents.clone();
        let n = adjacents.len();

        let neighbour_map = self.neighbour_list_to_neighbour_matrix(adjacents);
        let mut state = vec![false; n];
        state[self.NFA.start] = true;
        for code_point in input {
            state = neighbour_map[&code_point].bool_mul(&state);
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

    fn neighbour_list_to_neighbour_matrix(&self, neighbour_list: Vec<Vec<Edge>>) -> HashMap<CodePoint, Matrix> {
        let size = neighbour_list.len();
        let mut map: HashMap<CodePoint, Matrix> = HashMap::new();

        for (i, neighbors) in neighbour_list.into_iter().enumerate() {
            for edge in neighbors {
                let code_point = match edge.r#type {
                    EdgeType::Regular(code_point) => code_point,
                    EdgeType::Epsilon => panic!("not possible") 
                };
                let j = edge.to;

                let entry = map.entry(code_point).or_insert_with(|| Matrix::new(size));

                entry.matrix[i][j] = true;
            }
        }

        map
    }
}