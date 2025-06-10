pub struct GridParser {
    width: usize,
    height: usize,
    grid: Vec<Vec<char>>,
}

impl GridParser {
    pub fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            grid: Vec::new(),
        }
    }

    pub fn process_line(&mut self, line: &str) -> Option<(usize, usize, Vec<Vec<char>>)> {
        if line.starts_with("Anfield ") {
            self.parse_header(line);
        } else if self.width > 0 && self.height > 0 {
            if let Some(grid_data) = self.try_parse_grid_row(line) {
                return Some(grid_data);
            }
        }
        None
    }

    fn parse_header(&mut self, line: &str) {
        let parts: Vec<&str> = line.trim_end_matches(':').split_whitespace().collect();
        if parts.len() >= 3 {
            self.width = parts[1].parse().unwrap_or(0);
            self.height = parts[2].parse().unwrap_or(0);
            self.grid.clear();
        }
    }

    fn try_parse_grid_row(&mut self, line: &str) -> Option<(usize, usize, Vec<Vec<char>>)> {
        if line.len() <= 4 {
            return None;
        }

        // Only process lines that start with a 3-digit row number followed by space.
        if !self.is_grid_data_line(line) {
            return None;
        }

        let row_str = &line[4..];
        if row_str.len() >= self.width {
            self.grid.push(row_str.chars().take(self.width).collect());
            if self.grid.len() == self.height {
                let result = (self.width, self.height, self.grid.clone());
                self.grid.clear();
                return Some(result);
            }
        }
        None
    }

    fn is_grid_data_line(&self, line: &str) -> bool {
        line.len() >= 4
            && line.chars().nth(0).map_or(false, |c| c.is_ascii_digit())
            && line.chars().nth(1).map_or(false, |c| c.is_ascii_digit())
            && line.chars().nth(2).map_or(false, |c| c.is_ascii_digit())
            && line.chars().nth(3) == Some(' ')
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_anfield_grid_correctly() {
        let input_lines = [
            "Anfield 20 15:",
            "    01234567890123456789",
            "000 ....................",
            "001 ....................",
            "002 .........@..........",
            "003 ....................",
            "004 ....................",
            "005 ....................",
            "006 ....................",
            "007 ....................",
            "008 ....................",
            "009 ....................",
            "010 ....................",
            "011 ....................",
            "012 .........$..........",
            "013 ....................",
            "014 ....................",
        ];

        let mut parser = GridParser::new();

        let mut parsed_grid = None;
        for line in input_lines.iter() {
            if let Some((width, height, grid)) = parser.process_line(line) {
                parsed_grid = Some((width, height, grid));
                break;
            }
        }

        let (width, height, grid) = parsed_grid.expect("Grid was not parsed");

        assert_eq!(width, 20);
        assert_eq!(height, 15);
        assert_eq!(grid.len(), height);

        assert_eq!(grid[2][9], '@');
        assert_eq!(grid[12][9], '$');
        assert_eq!(grid[0][0], '.');
    }
}
