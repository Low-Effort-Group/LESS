mod encoder;
pub mod graphics;
use crate::video::encoder::*;
use crate::simulation;

use log::*;
use std::fs::create_dir_all;
use chrono::Local;

use crate::Args;

pub const WIDTH: u32 = 1080;
pub const HEIGHT: u32 = 1920;

pub fn start(args: Args) {
    //start timer
    let timer = std::time::Instant::now();
    
    create_dir_all("output").expect("Failed to create output directory");
    let time = Local::now();
    let filename = format!("slop_{}.mp4", time.format("%Y-%m-%d_%H-%M-%S"));
    let mut encoder =
        VideoEncoder::new(WIDTH, HEIGHT, args.fps, format!("output/{}", filename).as_str()).unwrap();

    let total_frames = args.fps * args.duration;

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
        let img = simulation::newFrame(&mut balls, &mut colliders);
        
        encoder.write_frame(&img);
        
        if frame_num % args.fps == 0 {
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
    info!("Video saved to {}.", filename);
}
