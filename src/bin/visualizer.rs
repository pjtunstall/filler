use std::{
    io::{self, BufRead},
    sync::mpsc,
    thread,
    time::Duration,
};

use minifb::{Key, Window, WindowOptions};

mod visualizer {
    pub mod parser;
    pub mod renderer;
}

use visualizer::parser::GridParser;
use visualizer::renderer::{create_buffer_and_window, render_grid_to_buffer};

fn main() {
    let (tx, rx) = mpsc::channel();
    let (scale, duration) = get_args();

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
        thread::sleep(Duration::from_millis(duration));
    }
}

fn get_args() -> (usize, u64) {
    let mut scale = 20;
    let mut duration = 16;

    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--scale" | "-s" => {
                if let Some(val) = args.next().and_then(|s| s.parse().ok()) {
                    scale = val;
                }
            }
            "--duration" | "-d" => {
                if let Some(val) = args.next().and_then(|s| s.parse().ok()) {
                    duration = val;
                }
            }
            _ => {}
        }
    }

    (scale, duration)
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
