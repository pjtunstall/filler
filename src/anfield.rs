use std::fmt;
use std::io;

use crate::bimap::BiMap;
use crate::cell::Cell;
use crate::parse;
use crate::piece::{Piece, PossiblePlacement};
use crate::symbols::{self, CellRole, Chars};

#[derive(Debug)]
pub struct Anfield {
    pub width: usize,
    pub height: usize,
    cells: Vec<CellRole>, // Row-major order, cells[y * width + x].
    own_char: char,
    opponent_char: char,
    own_latest_char: char,
    opponent_latest_char: char,
    char_to_role: BiMap<char, CellRole>,
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
                    self.char_to_role
                        .get_by_value(&cell)
                        .expect("Invalid cell found in BiMap of Anfield")
                )?;
            }
            write!(f, "\n")?;
        }

        write!(
            f,
            "own_char: {}, opponent_char: {}, own_latest_char: {}, opponent_latest_char: {}",
            self.own_char, self.opponent_char, self.own_latest_char, self.opponent_latest_char
        )
    }
}

impl Anfield {
    pub fn new(width: usize, height: usize, own_id: u8) -> Self {
        let mut char_to_role = BiMap::new();
        let Chars {
            own_char,
            opponent_char,
            own_latest_char,
            opponent_latest_char,
        } = symbols::populate_char_to_role(&mut char_to_role, own_id);

        Self {
            width,
            height,
            cells: vec![CellRole::Empty; width * height],
            own_char,
            opponent_char,
            own_latest_char,
            opponent_latest_char,
            char_to_role,
        }
    }

    pub fn place(&self, piece: &Piece) -> [usize; 2] {
        let possible_placements = self.get_possible_placements(&piece);
        let mut chosen_possible_placement = possible_placements[0];
        for possible_placement in possible_placements.iter().skip(1) {
            if possible_placement.distance_to_opponent
                > chosen_possible_placement.distance_to_opponent
            {
                chosen_possible_placement = *possible_placement;
            }
        }
        let x = chosen_possible_placement.x;
        let y = chosen_possible_placement.y;
        [x, y]
    }

    fn get_possible_placements(&self, piece: &Piece) -> Vec<PossiblePlacement> {
        let mut possible_placements = Vec::new();
        for x in 0..(self.width - piece.width) {
            for y in 0..(self.height - piece.height) {
                if let Some(mut possible_placement) = self.try_fit(&piece, x, y) {
                    possible_placement.distance_to_opponent =
                        self.get_distance_to_opponent(possible_placement.x, possible_placement.y);
                    possible_placements.push(possible_placement);
                }
            }
        }
        if possible_placements.len() == 0 {
            possible_placements.push(PossiblePlacement::default());
        }
        possible_placements
    }

    fn try_fit(&self, _piece: &Piece, _x: usize, _y: usize) -> Option<PossiblePlacement> {
        // TODO: iterate over all possible coordinates and check,
        // for each cell in the shape, if it's out of bounds. If not,
        // check if any of its cells overlap with our own territory.
        // If so, iterate over all cells in the shape and
        // check if they would overlap with the opponent's territory
        // or if they'd be a second cell overlapping with our own.
        // If all checks pass, return Some(PossiblePlacement).
        None
    }

    fn get_distance_to_opponent(&self, x: usize, y: usize) -> usize {
        let mut distance = 0;
        let mut q = Vec::new();
        let mut visited = vec![false; self.cells.len()];
        visited[y * self.width + x] = true;
        q.push(Cell { x, y });

        while !q.is_empty() {
            let p = q.pop().unwrap();
            match self.get_cell(p.x, p.y) {
                Some(CellRole::OpponentSymbol) | Some(CellRole::OpponentLatestMove) => break,
                _ => (),
            }
            distance += 1;

            if p.x > 0 {
                self.try_visit(&mut q, &mut visited, p.x - 1, p.y);
            }
            if p.x < self.width - 1 {
                self.try_visit(&mut q, &mut visited, p.x + 1, p.y);
            }
            if p.y > 0 {
                self.try_visit(&mut q, &mut visited, p.x, p.y - 1);
            }
            if p.y < self.height - 1 {
                self.try_visit(&mut q, &mut visited, p.x, p.y + 1);
            }
        }

        distance
    }

    fn try_visit(&self, q: &mut Vec<Cell>, visited: &mut Vec<bool>, s: usize, t: usize) {
        if visited[t * self.width + s] {
            return;
        }
        match self.get_cell(s, t) {
            Some(CellRole::OwnSymbol) | Some(CellRole::OwnLatestMove) => return,
            _ => (),
        }
        visited[t * self.width + s] = true;
        q.push(Cell { x: s, y: t });
    }

    fn get_cell(&self, x: usize, y: usize) -> Option<CellRole> {
        if x < self.width && y < self.height {
            Some(self.cells[y * self.width + x])
        } else {
            None
        }
    }

    fn set_cell(&mut self, x: usize, y: usize, cell: CellRole) {
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

    // fn is_own_char(&self, x: usize, y: usize) -> bool {
    //     self.get_cell(x, y) == Some(Cell::OwnCellRole)
    // }

    // fn is_opponent_char(&self, x: usize, y: usize) -> bool {
    //     self.get_cell(x, y) == Some(Cell::OpponentCellRole)
    // }

    // fn is_own_latest_char(&self, x: usize, y: usize) -> bool {
    //     self.get_cell(x, y) == Some(Cell::OwnLatestMove)
    // }

    // fn is_opponent_latest_char(&self, x: usize, y: usize) -> bool {
    //     self.get_cell(x, y) == Some(Cell::OpponentLatestMove)
    // }

    // fn set_own_latest_char(&mut self, x: usize, y: usize) {
    //     self.set_cell(x, y, Cell::OwnLatestMove);
    // }

    // fn set_opponent_latest_char(&mut self, x: usize, y: usize) {
    //     self.set_cell(x, y, Cell::OpponentLatestMove);
    // }

    // fn set_own_char(&mut self, x: usize, y: usize) {
    //     self.set_cell(x, y, Cell::OwnCellRole);
    // }

    // fn set_opponent_char(&mut self, x: usize, y: usize) {
    //     self.set_cell(x, y, Cell::OpponentCellRole);
    // }

    fn parse_cell(&mut self, x: usize, y: usize, c: char) {
        let cell = self
            .char_to_role
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
