use std::io;

use crate::parse;
use crate::point::Point;
use crate::symbols;

#[derive(Debug)]
pub struct Piece {
    pub width: usize,
    pub height: usize,
    shape: Vec<Point>,
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

#[derive(Default, Debug, Clone, Copy)]
pub struct PossiblePlacement {
    pub x: usize,
    pub y: usize,
    pub distance_to_opponent: usize,
}

fn parse(raw: Vec<String>) -> Vec<Point> {
    let mut shape = Vec::new();

    for (x, line) in raw.iter().enumerate() {
        for (y, c) in line.chars().enumerate() {
            if c == symbols::NEW_PIECE_CHAR {
                shape.push(Point { y, x });
            }
        }
    }
    shape
}
