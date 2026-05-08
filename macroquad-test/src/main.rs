use macroquad::prelude::*;
use std::fs;
use std::io::Write;
use std::path::Path;

#[macroquad::main("BasicShapes")]
async fn main() {
    let out_dir = "frames";
    if !Path::new(out_dir).exists() {
        fs::create_dir(out_dir).unwrap();
    };

    if let Err(_) = std::env::var("LESS_RES") {
        request_new_screen_size(1080.0, 1920.0); // Short-form content is 9:16
    }
    std::thread::sleep_ms(1000);
    let mut frame_count: u16 = 0;

    let width = screen_width() as u32;
    let height = screen_height() as u32;
    loop {
        if frame_count == 1400 {
            break
        }
        clear_background(RED);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

        draw_text(format!("IT WORKS: frame{:06}", frame_count).as_str(), 20.0, 20.0, 30.0, DARKGRAY);

        let screenshot = get_screen_data();
        // convert to video while still in memory
        next_frame().await;
        
        frame_count += 1;
    }
}