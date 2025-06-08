use minifb::{Key, Window, WindowOptions};
use std::io::{self, BufRead};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

const SCALE: usize = 6;

fn main() {
    let (tx, rx) = mpsc::channel();

    // Spawn thread to read input
    thread::spawn(move || {
        let stdin = io::stdin();
        let mut width = 0;
        let mut height = 0;
        let mut grid: Vec<Vec<char>> = Vec::new();

        for line in stdin.lock().lines().filter_map(Result::ok) {
            if line.starts_with("Anfield ") {
                let parts: Vec<&str> = line.trim_end_matches(':').split_whitespace().collect();
                if parts.len() >= 3 {
                    width = parts[1].parse().unwrap_or(0);
                    height = parts[2].parse().unwrap_or(0);
                    grid.clear();
                }
            } else if width > 0 && height > 0 {
                if line.len() > 4 {
                    let row_str = &line[4..];
                    if row_str.len() >= width {
                        grid.push(row_str.chars().take(width).collect());
                        if grid.len() == height {
                            eprintln!("Parsed grid {}x{}", width, height);
                            let _ = tx.send((width, height, grid.clone()));
                            grid.clear();
                        }
                    }
                }
            }
        }
    });

    let mut width = 1;
    let mut height = 1;
    let mut buffer = vec![0u32; SCALE * SCALE];
    let mut window = Window::new("Visualizer", SCALE, SCALE, WindowOptions::default())
        .expect("Failed to create window");

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if let Ok((new_width, new_height, new_grid)) = rx.try_recv() {
            width = new_width;
            height = new_height;

            let buf_width = width * SCALE;
            let buf_height = height * SCALE;
            buffer = vec![0u32; buf_width * buf_height];

            window = Window::new(
                "Visualizer",
                buf_width,
                buf_height,
                WindowOptions::default(),
            )
            .unwrap_or_else(|e| panic!("Failed to create window: {}", e));

            for y in 0..height {
                for x in 0..width {
                    let color = match new_grid[y][x] {
                        '@' => 0xFF0000,
                        '$' => 0x00FF00,
                        'a' => 0xFFAAAA,
                        's' => 0xAAFFAA,
                        _ => 0x000000,
                    };
                    for dy in 0..SCALE {
                        for dx in 0..SCALE {
                            let px = x * SCALE + dx;
                            let py = y * SCALE + dy;
                            buffer[py * buf_width + px] = color;
                        }
                    }
                }
            }
        }

        window
            .update_with_buffer(&buffer, width * SCALE, height * SCALE)
            .unwrap();
        thread::sleep(Duration::from_millis(16));
    }
}
