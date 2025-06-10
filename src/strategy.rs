pub mod attack;

use crate::{anfield::Anfield, piece::Piece};

pub trait Strategy {
    fn choose_move(&self, anfield: &Anfield, piece: &Piece) -> [i32; 2];
}
