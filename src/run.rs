use std::io::{self, BufRead};

use crate::{anfield::Anfield, game::Game, parse, piece::Piece, strategy::Strategy};

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
        game.anfield
            .parse(&mut lines)
            .expect("Failed to parse Anfield");

        let next_line = parse::read_line(&mut lines, "piece header")?;
        let [width, height] = parse::get_width_and_height(next_line);
        let piece = Piece::new(&mut lines, width, height)?;
        game.play(&piece);

        let _ = parse::read_line(&mut lines, "anfield header")?;
    }
}
