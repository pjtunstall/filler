use std::fmt;
use std::io;

use crate::bimap::BiMap;
use crate::parse;
use crate::symbols;

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
    width: usize,
    height: usize,
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

    // fn get_cell(&self, x: usize, y: usize) -> Option<Cell> {
    //     if x < self.width && y < self.height {
    //         Some(self.cells[y * self.width + x])
    //     } else {
    //         None
    //     }
    // }

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
