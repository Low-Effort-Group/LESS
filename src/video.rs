mod encoder;
mod graphics;
use crate::video::encoder::*;
use crate::video::graphics::*;

use image::{ImageBuffer, Rgba};

pub fn setup_encoder() {
    let width = 1080u32;
    let height = 1920u32;
    let fps = 60u32;
    let duration_secs = 5;

    let mut encoder =
        VideoEncoder::new(width, height, fps, "output.mp4").expect("Failed to start ffmpeg");

    let total_frames = fps * duration_secs;

    println!("Recording {} frames at {}x{}", total_frames, width, height);

    for frame_num in 0..total_frames {
        let mut img = ImageBuffer::new(width, height);

        // Fill background (black)
        for y in 0..height {
            for x in 0..width {
                img.put_pixel(x, y, Rgba([0, 0, 0, 255]));
            }
        }

        let time = frame_num as f32 / fps as f32;

        // Draw moving circle
        let circle_x = (width as f32 / 2.0 + (time * 2.0).sin() * 200.0) as i32;
        let circle_y = (height as f32 / 2.0 + (time * 1.5).cos() * 200.0) as i32;
        draw_circle(&mut img, circle_x, circle_y, 50, Rgba([255, 0, 0, 255]));

        // Draw rotating rectangle
        draw_rect(&mut img, 400, 800, 200, 100, Rgba([0, 255, 0, 255]));

        encoder.write_frame(&img);

        if frame_num % 60 == 0 {
            println!("Frame {}/{}", frame_num, total_frames);
        }
    }

    encoder.finish();
    println!("Video saved to output.mp4.");
}
