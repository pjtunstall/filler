use std::io;

use crate::cell::Cell;
use crate::parse;
use crate::symbols;

#[derive(Debug)]
pub struct Piece {
    pub width: usize,
    pub height: usize,
    pub shape: Vec<Cell>,
}

impl Piece {
    pub fn new(
        lines: &mut impl Iterator<Item = Result<String, io::Error>>,
        width: usize,
        height: usize,
    ) -> Result<Self, io::Error> {
        let mut raw_piece = Vec::with_capacity(height);
        for _ in 0..height {
            raw_piece.push(parse::read_line(lines, "piece data")?);
        }
        let shape = parse(raw_piece);
        Ok(Piece {
            width,
            height,
            shape,
        })
    }
}


#[derive(Default, Debug, Clone, Copy)]
pub struct PossiblePlacement {
    pub x: usize,
    pub y: usize,
    pub distance_to_opponent: usize,
}

fn parse(raw: Vec<String>) -> Vec<Cell> {
    let mut shape = Vec::new();

    for (y, line) in raw.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != symbols::EMPTY_CHAR {
                shape.push(Cell { y, x });
            }
        }
    }
    shape
}
