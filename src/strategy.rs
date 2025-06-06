pub mod attack;

use crate::anfield::Anfield;
use crate::piece::Piece;

pub trait Strategy {
    fn choose_move(&self, anfield: &Anfield, piece: &Piece) -> Option<(i32, i32)>;
}
