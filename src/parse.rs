use std::io;

// use std::io::{self, Write};
// use std::fs::OpenOptions;

// fn log(msg: &str) {
//     if let Ok(mut file) = OpenOptions::new()
//         .create(true)
//         .append(true)
//         .open("/tmp/filler_log.txt")
//     {
//         let _ = writeln!(file, "{}", msg);
//     }
// }

pub fn read_line<I>(lines: &mut I, what: &str) -> Result<String, io::Error>
where
    I: Iterator<Item = Result<String, io::Error>>,
{
    match lines.next() {
        Some(Ok(line)) => {
            // log(&format!("Read line ({}): {}", what, line));
            Ok(line)
        }
        Some(Err(e)) => {
            // log(&format!("Error reading line ({}): {}", what, e));
            Err(e)
        }
        None => {
            // log(&format!("EOF reached unexpectedly while reading {}", what));
            Err(io::Error::new(io::ErrorKind::UnexpectedEof, format!("Missing {}", what)))
        }
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
