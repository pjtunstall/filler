use std::{io, num::ParseIntError};

use crate::errors::GameError;

pub fn read_line<I>(lines: &mut I, what: &str) -> Result<String, io::Error>
where
    I: Iterator<Item = Result<String, io::Error>>,
{
    match lines.next() {
        Some(Ok(line)) => Ok(line),
        Some(Err(e)) => Err(e),
        None => Err(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            format!("Missing {}", what),
        )),
    }
}

pub fn get_ids(line: String) -> Result<u8, GameError> {
    let c = line
        .chars()
        .nth(10)
        .ok_or_else(|| GameError::ParsePlayerIds("line too short".to_string()))?;

    let digit = c.to_digit(10).ok_or_else(|| {
        GameError::ParsePlayerIds(
            "invalid first line: character at 11th position is not a digit".to_string(),
        )
    })?;

    if digit == 1 {
        Ok(1)
    } else if digit == 2 {
        Ok(2)
    } else {
        Err(GameError::ParsePlayerIds("wrong digit".to_string()))
    }
}

pub fn get_width_and_height(line: String) -> Result<[usize; 2], GameError> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    let first: usize = parts[1]
        .parse()
        .map_err(|e: ParseIntError| GameError::ParseDimensions(e.to_string()))?;
    let second: usize = parts[2]
        .trim_end_matches(':')
        .parse()
        .map_err(|e: ParseIntError| GameError::ParseDimensions(e.to_string()))?;
    Ok([first, second])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_width_and_height() {
        let line = "Anfield 20 15:";
        let msg = format!("Failed to get width and height: {:?}", line);
        let dims = get_width_and_height(line.to_string()).expect(&msg);
        assert_eq!(dims, [20, 15]);
    }

    #[test]
    fn test_get_width_and_height_fail() {
        let line = "Anfield xx 15:";
        let err = get_width_and_height(line.to_string()).unwrap_err();
        match err {
            GameError::ParseDimensions(_) => {}
            _ => panic!("Unexpected error variant"),
        }
    }

    #[test]
    fn test_get_ids_success() {
        let p1 = "$$$ exec p1 : [robots/bender]";
        let ids1 = get_ids(p1.to_string()).expect("Failed to get ID for player 1");
        assert_eq!(ids1, 1);

        let p2 = "$$$ exec p2 : [robots/bender]";
        let ids2 = get_ids(p2.to_string()).expect("Failed to get ID for player 2");
        assert_eq!(ids2, 2);
    }

    #[test]
    fn test_get_ids_fail_short_line() {
        let p = "short line";
        let err = get_ids(p.to_string()).unwrap_err();
        match err {
            GameError::ParsePlayerIds(msg) => assert_eq!(msg, "line too short"),
            _ => panic!("Expected ParsePlayerIds error"),
        }
    }

    #[test]
    fn test_get_ids_fail_invalid_digit() {
        let p = "$$$ exec px : [robots/bender]";
        let err = get_ids(p.to_string()).unwrap_err();
        match err {
            GameError::ParsePlayerIds(msg) => assert!(
                msg.contains("character at 11th position is not a digit"),
                "Unexpected message: {}",
                msg
            ),
            _ => panic!("Expected ParsePlayerIds error"),
        }
    }

    #[test]
    fn test_get_ids_fail_wrong_digit() {
        let p = "$$$ exec p3 : [robots/bender]";
        let err = get_ids(p.to_string()).unwrap_err();
        match err {
            GameError::ParsePlayerIds(msg) => assert_eq!(msg, "wrong digit"),
            _ => panic!("Expected ParsePlayerIds error"),
        }
    }
}
