use std::io::{self, BufRead};

use crate::anfield::Anfield;
use crate::game::Game;
use crate::parse;
use crate::piece::Piece;
use crate::strategy::Strategy;

pub fn run(strategy: impl Strategy) -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = parse::read_line(&mut lines, "first")?;
    let second_line = parse::read_line(&mut lines, "second")?;

    let [own_id, _opponent_id] = parse::get_ids(first_line);
    let [width, height] = parse::get_width_and_height(second_line);
    let anfield = Anfield::new(width, height, own_id);

    let mut game = Game { anfield, strategy };

    loop {
        // Since stdout is consumed by the game engine, we can't rely on it for logging errors.
        if let Err(e) = game.anfield.parse(&mut lines) {
            std::fs::write("/tmp/filler_error.log", format!("Parse error: {}", e)).ok();
            return Err(e.into());
        }

        let next_line = parse::read_line(&mut lines, "piece header")?;
        let [width, height] = parse::get_width_and_height(next_line);
        let piece = Piece::new(&mut lines, width, height)?;
        game.play(&piece);

        let _ = parse::read_line(&mut lines, "anfield header")?;
    }
}
