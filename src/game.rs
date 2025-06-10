use std::io::{self, Write};

use crate::{anfield::Anfield, piece::Piece, strategy::Strategy};

pub struct Game<S: Strategy> {
    pub anfield: Anfield,
    pub strategy: S,
}

impl<S: Strategy> Game<S> {
    pub fn play(&mut self, piece: &Piece) {
        let [x, y] = self.strategy.choose_move(&self.anfield, piece);
        println!("{} {}", x, y);
        io::stdout().flush().expect("flush failed");
    }
}
