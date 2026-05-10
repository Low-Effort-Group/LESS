mod encoder;
pub mod graphics;
use crate::video::encoder::*;
use crate::types::ball::Ball;
use crate::simulation;

use log::*;
use std::fs::create_dir_all;
use chrono::Local;

pub const WIDTH: u32 = 1080;
pub const HEIGHT: u32 = 1920;
pub const FPS: u32 = 60;

pub fn setup_encoder() {
    //start timer
    let timer = std::time::Instant::now();

    let duration_secs = 20;
    
    create_dir_all("output").expect("Failed to create output directory");
    let time = Local::now();
    let time = time.format("%Y-%m-%d_%H-%M-%S").to_string();
    let mut encoder =
        VideoEncoder::new(WIDTH, HEIGHT, FPS, format!("output/slop_{}.mp4", time).as_str()).unwrap();

    let total_frames = FPS * duration_secs;

    info!("Recording {} frames at {}x{}", total_frames, WIDTH, HEIGHT);

    // let content: ContentType = TYPES[0];

    // println!("simulation name: {}", content.name);
    // println!("simulation descripticon: {}", content.description);

    //setup simulation
    let (mut balls, mut colliders) = crate::simulation::setupSimulation();

    let mut time = std::time::Instant::now();
    for frame_num in 0..total_frames {
        let frame_start = std::time::Instant::now();
        // This draws the ball (ball.rs)
        let mut img = simulation::newFrame(&mut balls, &mut colliders);
        
        encoder.write_frame(&img);
        
        if frame_num % 50 == 0 {
            info!("Frame {}/{}, in {} seconds", frame_num, total_frames, time.elapsed().as_secs_f32());
            time = std::time::Instant::now();
        } else {
            //print which frame is done and frame time
            let time = frame_start.elapsed().as_secs_f32();
            trace!("Frame {}/{}, in {} seconds", frame_num, total_frames, time);        
        }
    }

    encoder.finish();
    info!("{} Frames recorded in {} seconds", total_frames, timer.elapsed().as_secs_f32());
    info!("Video saved to output.mp4.");
}
