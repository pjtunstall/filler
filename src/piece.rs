use std::io;

use crate::parse;
use crate::symbols;

#[derive(Debug)]
pub struct Piece {
    width: usize,
    height: usize,
    shape: Vec<Cell>,
}

impl Piece {
    pub fn new(
        lines: &mut impl Iterator<Item = Result<String, io::Error>>,
        width: usize,
        height: usize,
    ) -> Self {
        let mut raw_piece = Vec::new();
        for _ in 0..height {
            raw_piece.push(parse::read_line(lines, "piece data"));
        }
        let shape = parse(raw_piece);
        Piece {
            width,
            height,
            shape,
        }
    }
}

#[derive(Debug)]
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
