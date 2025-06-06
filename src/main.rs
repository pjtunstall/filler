use std::io::{self, BufRead, Write};

use filler::anfield::Anfield;
use filler::parse;
use filler::piece::Piece;
use filler::strategy::Strategy;
use filler::strategy::attack::Attack;

pub struct Game<S: Strategy> {
    anfield: Anfield,
    strategy: S,
}

impl<S: Strategy> Game<S> {
    fn play(&mut self, piece: &Piece) {
        let s;
        if let Some((x, y)) = self.strategy.choose_move(&self.anfield, piece) {
            s = format!("{} {}\n", x, y);
        } else {
            s = String::from("0 0\n");
        }
        io::stdout()
            .write_all(s.as_bytes())
            .expect("Failed to write move to stdout (broken pipe or closed output?)");
        io::stdout()
            .flush()
            .expect("Failed to flush stdout (broken pipe or output stream error?)");
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = parse::read_line(&mut lines, "first");
    let second_line = parse::read_line(&mut lines, "second");

    let [own_id, _opponent_id] = parse::get_ids(first_line);
    let [width, height] = parse::get_width_and_height(second_line);
    let anfield = Anfield::new(width, height, own_id);

    let strategy = Attack;
    let mut game = Game { anfield, strategy };

    // Each turn:
    game.anfield.parse(&mut lines);
    let next_line = parse::read_line(&mut lines, "piece header");
    let [width, height] = parse::get_width_and_height(next_line);
    let piece = Piece::new(&mut lines, width, height);
    game.play(&piece);
}
