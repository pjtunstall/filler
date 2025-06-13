use std::io::{Lines, StdinLock, Write};

use crate::{anfield::Anfield, errors::GameError, parse, piece::Piece, strategy::Strategy};

pub struct Game<S: Strategy> {
    pub anfield: Anfield,
    pub strategy: S,
    pub lines: Lines<StdinLock<'static>>,
}

impl<S: Strategy> Iterator for Game<S> {
    type Item = Result<(), GameError>;

    fn next(&mut self) -> Option<Self::Item> {
        let result = (|| {
            self.anfield
                .parse(&mut self.lines)
                .map_err(|e| GameError::ParseAnfieldBody(e.to_string()))?;

            let next_line = parse::read_line(&mut self.lines, "piece header")
                .map_err(|e| map_io_error(e, "piece header"))?;

            let [width, height] = parse::get_width_and_height(next_line.clone())
                .map_err(|_| GameError::ParsePieceDimensions(next_line))?;

            let piece = Piece::new(&mut self.lines, width, height)
                .map_err(|e| GameError::ParsePieceBody(e.to_string()))?;

            self.play(&piece);

            parse::read_line(&mut self.lines, "anfield header")
                .map_err(|e| map_io_error(e, "anfield header"))?;

            Ok(())
        })();

        Some(result)
    }
}

impl<S: Strategy> Game<S> {
    pub fn play(&mut self, piece: &Piece) {
        let [x, y] = self.strategy.choose_move(&self.anfield, piece);
        println!("{} {}", x, y);
        std::io::stdout().flush().expect("flush failed");
    }
}

fn map_io_error(err: std::io::Error, context: &'static str) -> GameError {
    if err.kind() == std::io::ErrorKind::UnexpectedEof {
        GameError::UnexpectedEof(context)
    } else {
        GameError::Io(err)
    }
}
