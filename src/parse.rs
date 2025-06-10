use std::io;

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

pub fn get_ids(line: String) -> [u8; 2] {
    let digit = line
        .chars()
        .nth(10)
        .expect("Line too short")
        .to_digit(10)
        .expect("Invalid first line: character at 11th position is not a digit");

    if digit == 1 { [1, 2] } else { [2, 1] }
}

pub fn get_width_and_height(line: String) -> [usize; 2] {
    let parts: Vec<&str> = line.split_whitespace().collect();
    let first: usize = parts[1].parse().unwrap();
    let second: usize = parts[2].trim_end_matches(':').parse().unwrap();
    [first, second]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_width_and_height() {
        let line = "Anfield 20 15:";
        let dims = get_width_and_height(line.to_string());
        assert_eq!(dims, [20, 15]);
    }

    #[test]
    fn test_get_ids() {
        let line1 = "$$$ exec p1 : [robots/bender]";
        let ids1 = get_ids(line1.to_string());
        assert_eq!(ids1, [1, 2]);

        let line2 = "$$$ exec p2 : [robots/bender]";
        let ids2 = get_ids(line2.to_string());
        assert_eq!(ids2, [2, 1]);
    }
}
