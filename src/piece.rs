use crate::symbols;

pub struct Piece {
    width: usize,
    height: usize,
    shape: Vec<Cell>,
}

impl Piece {
    pub fn new(width: usize, height: usize, raw: Vec<String>) -> Self {
        let shape = parse(raw);
        Piece {
            width,
            height,
            shape,
        }
    }
}

struct Cell {
    x: usize,
    y: usize,
}

fn parse(raw: Vec<String>) -> Vec<Cell> {
    let mut shape = Vec::new();
    for (x, line) in raw.iter().enumerate() {
        for (y, c) in line.chars().enumerate() {
            if c == symbols::NEW_PIECE {
                shape.push(Cell { y, x });
            }
        }
    }
    shape
}
