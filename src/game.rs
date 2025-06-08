use std::io::{self, Write};

use crate::anfield::Anfield;
use crate::piece::Piece;
use crate::strategy::Strategy;

pub struct Game<S: Strategy> {
    pub anfield: Anfield,
    pub strategy: S,
}

impl<S: Strategy> Game<S> {
    pub fn play(&mut self, piece: &Piece) {
        let [x, y] = self.strategy.choose_move(&self.anfield, piece);
        let s = format!("{} {}\n", x, y);
        io::stdout()
            .write_all(s.as_bytes())
            .expect("Failed to write move to stdout (broken pipe or closed output?)");
        io::stdout()
            .flush()
            .expect("Failed to flush stdout (broken pipe or output stream error?)");
    }
}
