use std::io::{self, BufRead, Write};

use crate::{anfield::Anfield, errors::GameError, game::Game, parse, strategy::Strategy};

pub fn run(strategy: impl Strategy) -> Result<(), GameError> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = parse::read_line(&mut lines, "first")?;
    let own_id = parse::get_ids(first_line)?;
    let anfield = Anfield::new(own_id);
    let game = Game::new(anfield, strategy, lines);

    for turn in game {
        match turn {
            Ok(s) => {
                println!("{s}");
                io::stdout().flush().expect("flush failed");
            }
            Err(e) => return Err(e),
        }
    }

    Ok(())
}
