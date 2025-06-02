use std::fmt;
use std::io;

use crate::bimap::BiMap;
use crate::parse;
use crate::piece::{Piece, Possibility};
use crate::symbols;

struct Node {
    x: usize,
    y: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum Cell {
    Empty,
    OwnSymbol,
    OpponentSymbol,
    OwnLatestMove,
    OpponentLatestMove,
}

#[derive(Debug)]
pub struct Anfield {
    pub width: usize,
    pub height: usize,
    cells: Vec<Cell>, // Row-major order, cells[y * width + x].
    own_symbol: char,
    opponent_symbol: char,
    own_latest_move: char,
    opponent_latest_move: char,
    symbols: BiMap<char, Cell>,
}

impl fmt::Display for Anfield {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Anfield {} {}\n", self.width, self.height)?;

        write!(f, "    ")?;
        for x in 0..self.width {
            write!(f, "{}", x % 10)?;
        }
        write!(f, "\n")?;

        for y in 0..self.height {
            write!(f, "{:03} ", y)?;
            for x in 0..self.width {
                let cell = self.cells[y * self.width + x];
                write!(
                    f,
                    "{}",
                    self.symbols
                        .get_by_value(&cell)
                        .expect("Invalid cell found in BiMap of Anfield")
                )?;
            }
            write!(f, "\n")?;
        }

        write!(
            f,
            "own_symbol: {}, opponent_symbol: {}, own_latest_move: {}, opponent_latest_move: {}",
            self.own_symbol, self.opponent_symbol, self.own_latest_move, self.opponent_latest_move
        )
    }
}

impl Anfield {
    pub fn new(width: usize, height: usize, own_id: u8) -> Self {
        let [own_symbol, opponent_symbol] = Self::assign_symbols(own_id);
        let [own_latest_move, opponent_latest_move] = Self::assign_latest_move_symbols(own_id);

        let mut symbols = BiMap::new();
        symbols.insert(symbols::EMPTY, Cell::Empty);
        symbols.insert(own_symbol, Cell::OwnSymbol);
        symbols.insert(opponent_symbol, Cell::OpponentSymbol);
        symbols.insert(own_latest_move, Cell::OwnLatestMove);
        symbols.insert(opponent_latest_move, Cell::OpponentLatestMove);

        Self {
            width,
            height,
            cells: vec![Cell::Empty; width * height],
            own_symbol,
            opponent_symbol,
            own_latest_move,
            opponent_latest_move,
            symbols,
        }
    }

    pub fn assign_symbols(own_id: u8) -> [char; 2] {
        if own_id == 1 {
            [symbols::P1, symbols::P2]
        } else {
            [symbols::P2, symbols::P1]
        }
    }

    pub fn assign_latest_move_symbols(own_id: u8) -> [char; 2] {
        if own_id == 1 {
            [symbols::P1_LATEST_MOVE, symbols::P2_LATEST_MOVE]
        } else {
            [symbols::P2_LATEST_MOVE, symbols::P1_LATEST_MOVE]
        }
    }

    pub fn place(&self, piece: &Piece) -> [usize; 2] {
        let possibilities = self.get_possibilities(&piece);
        let mut chosen_possibility = possibilities[0];
        for possibility in possibilities.iter().skip(1) {
            if possibility.distance > chosen_possibility.distance {
                chosen_possibility = *possibility;
            }
        }
        let x = chosen_possibility.x;
        let y = chosen_possibility.y;
        [x, y]
    }

    fn get_possibilities(&self, piece: &Piece) -> Vec<Possibility> {
        let mut possibilities = Vec::new();
        for x in 0..(self.width - piece.width) {
            for y in 0..(self.height - piece.height) {
                if let Some(mut possibility) = self.try_fit(&piece, x, y) {
                    possibility.distance =
                        self.get_distance_to_opponent(possibility.x, possibility.y);
                    possibilities.push(possibility);
                }
            }
        }
        if possibilities.len() == 0 {
            possibilities.push(Possibility::default());
        }
        possibilities
    }

    fn try_fit(&self, _piece: &Piece, _x: usize, _y: usize) -> Option<Possibility> {
        // TODO: iterate over all possible coordinates and check,
        // for each cell in the shape, if it's out of bounds. If not,
        // check if any of its cells overlap with our own territory.
        // If so, iterate over all cells in the shape and
        // check if they would overlap with the opponent's territory
        // of if they'd be a second cell overlapping with our own.
        // If all checks pass, return Some(Possibility).
        None
    }

    fn get_distance_to_opponent(&self, x: usize, y: usize) -> usize {
        let mut distance = 0;
        let mut q = Vec::new();
        let mut visited = vec![false; self.cells.len()];
        visited[y * self.width + x] = true;
        q.push(Node { x, y });

        while !q.is_empty() {
            let n = q.pop().unwrap();
            match self.get_cell(n.x, n.y) {
                Some(Cell::OpponentSymbol) | Some(Cell::OpponentLatestMove) => break,
                _ => (),
            }
            distance += 1;

            if n.x > 0 {
                self.try_visit(&mut q, &mut visited, n.x - 1, n.y);
            }
            if n.x < self.width - 1 {
                self.try_visit(&mut q, &mut visited, n.x + 1, n.y);
            }
            if n.y > 0 {
                self.try_visit(&mut q, &mut visited, n.x, n.y - 1);
            }
            if n.y < self.height - 1 {
                self.try_visit(&mut q, &mut visited, n.x, n.y + 1);
            }
        }

        distance
    }

    fn try_visit(&self, q: &mut Vec<Node>, visited: &mut Vec<bool>, s: usize, t: usize) {
        if visited[t * self.width + s] {
            return;
        }
        match self.get_cell(s, t) {
            Some(Cell::OwnSymbol) | Some(Cell::OwnLatestMove) => return,
            _ => (),
        }
        visited[t * self.width + s] = true;
        q.push(Node { x: s, y: t });
    }

    fn get_cell(&self, x: usize, y: usize) -> Option<Cell> {
        if x < self.width && y < self.height {
            Some(self.cells[y * self.width + x])
        } else {
            None
        }
    }

    fn set_cell(&mut self, x: usize, y: usize, cell: Cell) {
        if x < self.width && y < self.height {
            self.cells[y * self.width + x] = cell;
        }
    }

    // fn is_empty(&self, x: usize, y: usize) -> bool {
    //     self.get_cell(x, y) == Some(Cell::Empty)
    // }

    // fn is_occupied(&self, x: usize, y: usize) -> bool {
    //     self.get_cell(x, y) != Some(Cell::Empty)
    // }

    // fn is_own_symbol(&self, x: usize, y: usize) -> bool {
    //     self.get_cell(x, y) == Some(Cell::OwnSymbol)
    // }

    // fn is_opponent_symbol(&self, x: usize, y: usize) -> bool {
    //     self.get_cell(x, y) == Some(Cell::OpponentSymbol)
    // }

    // fn is_own_latest_move(&self, x: usize, y: usize) -> bool {
    //     self.get_cell(x, y) == Some(Cell::OwnLatestMove)
    // }

    // fn is_opponent_latest_move(&self, x: usize, y: usize) -> bool {
    //     self.get_cell(x, y) == Some(Cell::OpponentLatestMove)
    // }

    // fn set_own_latest_move(&mut self, x: usize, y: usize) {
    //     self.set_cell(x, y, Cell::OwnLatestMove);
    // }

    // fn set_opponent_latest_move(&mut self, x: usize, y: usize) {
    //     self.set_cell(x, y, Cell::OpponentLatestMove);
    // }

    // fn set_own_symbol(&mut self, x: usize, y: usize) {
    //     self.set_cell(x, y, Cell::OwnSymbol);
    // }

    // fn set_opponent_symbol(&mut self, x: usize, y: usize) {
    //     self.set_cell(x, y, Cell::OpponentSymbol);
    // }

    fn parse_cell(&mut self, x: usize, y: usize, c: char) {
        let cell = self
            .symbols
            .get_by_key(&c)
            .expect("Invalid character received in Anfield");
        self.set_cell(x, y, *cell);
    }

    pub fn parse(&mut self, lines: &mut impl Iterator<Item = Result<String, io::Error>>) {
        let _ = parse::read_line(lines, "column numbers"); // Ignore line with column numbers.
        let mut field = Vec::with_capacity(self.height);
        for _ in 0..self.height {
            field.push(parse::read_line(lines, "field")[4..].to_string()); // Ignore row numbers and space.
        }
        let mut y = 0;
        for line in field {
            let mut x = 0;
            for c in line.chars() {
                self.parse_cell(x, y, c);
                x += 1;
            }
            y += 1;
        }
    }
}
