use std::io::{self, BufRead};

use filler::anfield::Anfield;
use filler::parse;
use filler::piece::Piece;
use filler::piece::PossiblePlacement;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = parse::read_line(&mut lines, "first");
    let second_line = parse::read_line(&mut lines, "second");

    let [own_id, _opponent_id] = parse::get_ids(first_line);
    let [width, height] = parse::get_width_and_height(second_line);
    let mut anfield = Anfield::new(width, height, own_id);

    // Every turn:
    anfield.parse(&mut lines);
    let next_line = parse::read_line(&mut lines, "piece header");
    let [width, height] = parse::get_width_and_height(next_line);
    let piece = Piece::new(&mut lines, width, height);
    let [x, y] = anfield.place(&piece);
}
