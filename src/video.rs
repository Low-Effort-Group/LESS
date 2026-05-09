mod encoder;
pub mod graphics;
use crate::video::encoder::*;
use crate::types::ball::Ball;
use crate::simulation;

pub const WIDTH: u32 = 1080;
pub const HEIGHT: u32 = 1920;
pub const FPS: u32 = 60;

pub fn setup_encoder() {
    let duration_secs = 5;

    let mut encoder =
        VideoEncoder::new(WIDTH, HEIGHT, FPS, "output.mp4").expect("Failed to start ffmpeg");

    let total_frames = FPS * duration_secs;

    println!("Recording {} frames at {}x{}", total_frames, WIDTH, HEIGHT);

    // let content: ContentType = TYPES[0];

    // println!("simulation name: {}", content.name);
    // println!("simulation descripticon: {}", content.description);

    //setup simulation
    let balls = crate::simulation::setupSimulation();

    for frame_num in 0..total_frames {
        let frame_start = std::time::Instant::now();
        // This draws the ball (ball.rs)
        let mut img = simulation::newFrame(balls.clone());

        
        encoder.write_frame(&img);

        //print wich frame is done and frame time
        let time = frame_start.elapsed().as_secs_f32();
        println!("Frame {}/{}, in {} seconds", frame_num, total_frames, time);
        
    }

    encoder.finish();
    println!("Video saved to output.mp4.");
}
