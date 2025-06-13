use std::io::{self, BufRead};

use crate::{anfield::Anfield, errors::GameError, game::Game, parse, strategy::Strategy};

pub fn run(strategy: impl Strategy) -> Result<(), GameError> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = parse::read_line(&mut lines, "first")?;
    let second_line = parse::read_line(&mut lines, "second")?;

    let [own_id, _opponent_id] = parse::get_ids(first_line)?;
    let [width, height] = parse::get_width_and_height(second_line.clone())
        .map_err(|_| GameError::ParseAnfieldDimensions(second_line))?;
    let anfield = Anfield::new(width, height, own_id);

    let game = Game {
        anfield,
        strategy,
        lines,
    };

    for turn in game {
        turn?;
    }

    Ok(())
}
