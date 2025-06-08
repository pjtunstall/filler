use std::fmt;
use std::io;

use crate::bimap::BiMap;
use crate::parse;
use crate::symbols::{self, CellRole, Chars};

#[derive(Debug)]
pub struct Anfield {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<CellRole>, // Row-major order, cells[y * width + x].
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

    pub fn get_cell_role(&self, x: usize, y: usize) -> Option<CellRole> {
        if x < self.width && y < self.height {
            Some(self.cells[y * self.width + x])
        } else {
            None
        }
    }

    fn set_cell(&mut self, x: usize, y: usize, cell_role: CellRole) {
        if x < self.width && y < self.height {
            self.cells[y * self.width + x] = cell_role;
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

    pub fn parse(
    &mut self,
    lines: &mut impl Iterator<Item = Result<String, io::Error>>,
) -> Result<(), io::Error> {
    let _ = parse::read_line(lines, "column numbers")?; // Skip column numbers

    let mut field = Vec::with_capacity(self.height);
    for _ in 0..self.height {
        let line = parse::read_line(lines, "field")?;
        field.push(line[4..].to_string()); // Skip row number
    }

    for (y, line) in field.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            self.parse_cell(x, y, c);
        }
    }

    Ok(())
}


}
