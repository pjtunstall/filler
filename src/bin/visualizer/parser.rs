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

        // Only process lines that start with a 3-digit row number followed by space
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
