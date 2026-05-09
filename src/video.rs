mod encoder;
pub mod graphics;
use crate::video::encoder::*;
use crate::types::TYPE_BALL;

pub const WIDTH: u32 = 1080;
pub const HEIGHT: u32 = 1920;
pub const FPS: u32 = 60;

pub fn setup_encoder() {
    let duration_secs = 5;

    let mut encoder =
        VideoEncoder::new(WIDTH, HEIGHT, FPS, "output.mp4").expect("Failed to start ffmpeg");

    let total_frames = FPS * duration_secs;

    println!("Recording {} frames at {}x{}", total_frames, WIDTH, HEIGHT);

    for frame_num in 0..total_frames {
        let time = frame_num as f32 / FPS as f32;
        // This draws the ball (ball.rs)
        let img = TYPE_BALL.invoke(time);
        encoder.write_frame(&img);

        if frame_num % 60 == 0 {
            println!("Frame {}/{}", frame_num, total_frames);
        }
    }

    encoder.finish();
    println!("Video saved to output.mp4.");
}
