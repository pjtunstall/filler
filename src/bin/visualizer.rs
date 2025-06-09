use minifb::{Key, Window, WindowOptions};
use std::io::{self, BufRead};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

struct GridParser {
    width: usize,
    height: usize,
    grid: Vec<Vec<char>>,
}

impl GridParser {
    fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            grid: Vec::new(),
        }
    }

    fn process_line(&mut self, line: &str) -> Option<(usize, usize, Vec<Vec<char>>)> {
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

fn main() {
    let (tx, rx) = mpsc::channel();
    let scale = get_scale_from_args();

    spawn_input_thread(tx);

    let mut width = 1;
    let mut height = 1;
    let mut buffer = vec![0u32; scale * scale];
    let mut window = Window::new("Visualizer", scale, scale, WindowOptions::default())
        .expect("Failed to create window");

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if let Ok((new_width, new_height, new_grid)) = rx.try_recv() {
            width = new_width;
            height = new_height;

            let (new_buffer, new_window) =
                create_buffer_and_window(width, height, scale, Some(window));
            buffer = new_buffer;
            window = new_window;

            render_grid_to_buffer(&mut buffer, &new_grid, width, height, scale);
        }

        window
            .update_with_buffer(&buffer, width * scale, height * scale)
            .unwrap();
        thread::sleep(Duration::from_millis(16));
    }
}

fn spawn_input_thread(tx: mpsc::Sender<(usize, usize, Vec<Vec<char>>)>) {
    thread::spawn(move || {
        let stdin = io::stdin();
        let mut parser = GridParser::new();

        for line in stdin.lock().lines().filter_map(Result::ok) {
            println!("{}", line);
            if let Some(grid_data) = parser.process_line(&line) {
                let _ = tx.send(grid_data);
            }
        }
    });
}

fn create_buffer_and_window(
    width: usize,
    height: usize,
    scale: usize,
    current_window: Option<Window>,
) -> (Vec<u32>, Window) {
    let buf_width = width * scale;
    let buf_height = height * scale;
    let buffer = vec![0u32; buf_width * buf_height];

    let window = if let Some(win) = current_window {
        if buf_width != win.get_size().0 || buf_height != win.get_size().1 {
            Window::new(
                "Visualizer",
                buf_width,
                buf_height,
                WindowOptions::default(),
            )
            .unwrap_or_else(|e| panic!("Failed to create window: {}", e))
        } else {
            win
        }
    } else {
        Window::new(
            "Visualizer",
            buf_width,
            buf_height,
            WindowOptions::default(),
        )
        .expect("Failed to create window")
    };

    (buffer, window)
}

fn render_grid_to_buffer(
    buffer: &mut [u32],
    grid: &[Vec<char>],
    width: usize,
    height: usize,
    scale: usize,
) {
    let buf_width = width * scale;

    for y in 0..height {
        for x in 0..width {
            let color = match grid[y][x] {
                '@' => 0xFF0000,
                '$' => 0x00FF00,
                'a' => 0xFFAAAA,
                's' => 0xAAFFAA,
                _ => 0x123456,
            };
            for dy in 0..scale {
                for dx in 0..scale {
                    let px = x * scale + dx;
                    let py = y * scale + dy;
                    buffer[py * buf_width + px] = color;
                }
            }
        }
    }
}

fn get_scale_from_args() -> usize {
    std::env::args()
        .nth(1)
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(20)
}
