use std::fmt;

#[derive(Debug)]
pub enum GameError {
    Io(std::io::Error),
    ParsePlayerIds(String),
    ParseDimensions(String),
    ParseAnfieldDimensions(String),
    ParseAnfieldBody(String),
    ParsePieceHeader(String),
    ParsePieceDimensions(String),
    ParsePieceBody(String),
    UnexpectedEof(&'static str),
}

impl std::error::Error for GameError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            GameError::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for GameError {
    fn from(e: std::io::Error) -> Self {
        GameError::Io(e)
    }
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameError::Io(err) => write!(f, "I/O error: {:?}", err),
            GameError::ParsePlayerIds(s) => write!(f, "Failed to parse player IDs: {:?}", s),
            GameError::ParseDimensions(s) => {
                write!(f, "Failed to parse dimensions: {:?}", s)
            }
            GameError::ParseAnfieldDimensions(s) => {
                write!(f, "Failed to parse Anfield dimensions: {:?}", s)
            }
            GameError::ParseAnfieldBody(s) => write!(f, "Failed to parse Anfield body: {:?}", s),
            GameError::ParsePieceHeader(s) => write!(f, "Failed to parse piece header: {:?}", s),
            GameError::ParsePieceDimensions(s) => {
                write!(f, "Failed to parse piece dimensions: {:?}", s)
            }
            GameError::ParsePieceBody(s) => write!(f, "Failed to parse piece: {:?}", s),
            GameError::UnexpectedEof(context) => {
                write!(f, "Unexpected end of input while parsing {}", context)
            }
        }
    }
}
