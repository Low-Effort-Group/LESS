use crate::types::ContentType;
use crate::video::graphics::*;
use image::{Rgba, RgbaImage};

use crate::video::{HEIGHT, WIDTH};

// Get rid of these println! macros. They are only here for base implementation structure.
/// Bouncing ball
pub fn run(time: f32) -> RgbaImage {
    // <add ball bounce related stuff here>
    let mut img = RgbaImage::new(WIDTH, HEIGHT);

    // Fill background (black)
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            img.put_pixel(x, y, Rgba([0, 0, 0, 255]));
        }
    }

    // Draw moving circle
    let circle_x = (WIDTH as f32 / 2.0 + (time * 2.0).sin() * 200.0) as i32;
    let circle_y = (HEIGHT as f32 / 2.0 + (time * 1.5).cos() * 200.0) as i32;
    draw_circle(&mut img, circle_x, circle_y, 50, Rgba([255, 0, 0, 255]));

    // Draw rotating rectangle
    draw_rect(&mut img, 400, 800, 200, 100, Rgba([0, 255, 0, 255]));

    // Draw frame rate
    // Draw text here
    img
}
