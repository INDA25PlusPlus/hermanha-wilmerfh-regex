#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CodePoint {
    bytes: Vec<u8>,
    size: usize,
}

impl CodePoint {
    pub fn pipe() -> Self {
        Self {
            bytes: vec![b'|'],
            size: 1,
        }
    }
}

fn utf8_size(b0: u8) -> Option<usize> {
    if (b0 & 0b1000_0000) == 0b0000_0000 {
        Some(1)
    } else if (b0 & 0b1110_0000) == 0b1100_0000 {
        Some(2)
    } else if (b0 & 0b1111_0000) == 0b1110_0000 {
        Some(3)
    } else if (b0 & 0b1111_1000) == 0b1111_0000 {
        Some(4)
    } else {
        None
    }
}

pub struct Parser {
    code_points: Vec<CodePoint>,
    index: usize,
}

impl Parser {
    pub fn new(code_points: Vec<CodePoint>) -> Self {
        Self {
            code_points,
            index: 0,
        }
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        let code_points = bytes_to_codepoints(bytes);
        return Self::new(code_points);
    }

    pub fn peek(&self) -> Option<CodePoint> {
        if self.index >= self.code_points.len() {
            None
        } else {
            Some(self.code_points[self.index].clone())
        }
    }

    pub fn consume(&mut self) -> Option<CodePoint> {
        if self.index >= self.code_points.len() {
            None
        } else {
            let cp = self.code_points[self.index].clone();
            self.index += 1;
            Some(cp)
        }
    }
}

pub fn bytes_to_codepoints(bytes: Vec<u8>) -> Vec<CodePoint> {
    let mut iter = bytes.into_iter();
    let mut code_points = Vec::new();

    while let Some(b0) = iter.next() {
        let size = utf8_size(b0).expect("wrong utf8 format");
        let mut cp_bytes = Vec::with_capacity(size);
        cp_bytes.push(b0);

        for _ in 1..size {
            let b = iter.next().expect("size incorrect");

            if (b & 0b1100_0000) != 0b1000_0000 {
                panic!("wrong utf8 format")
            }

            cp_bytes.push(b)
        }

        let cp = CodePoint {
            bytes: cp_bytes,
            size,
        };
        code_points.push(cp);
    }

    code_points
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn ascii_string() {
//         let code_points = bytes_to_codepoints("hello".as_bytes().to_vec()).code_points;
//         assert_eq!(code_points.len(), 5);
//     }

//     #[test]
//     fn mixed_multibyte() {
//         let mut parser = bytes_to_codepoints("héllo, théré!".as_bytes().to_vec());
//         assert_eq!(parser.consume().unwrap().bytes.len(), 1);
//         assert_eq!(parser.consume().unwrap().bytes.len(), 2);
//         assert_eq!(parser.consume().unwrap().bytes.len(), 1);
//         assert_eq!(parser.consume().unwrap().bytes.len(), 1);
//     }
// }
