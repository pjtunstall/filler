use minifb::{Window, WindowOptions};

pub fn create_buffer_and_window(
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

pub fn render_grid_to_buffer(
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
                '@' => 0xFF0000, // red
                '$' => 0x00FF00, // green
                'a' => 0xFFAAAA, // light red
                's' => 0xAAFFAA, // light green
                _ => 0x123456,   // dark blue/gray
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
