mod encoder;
pub mod graphics;
use crate::video::encoder::*;
use crate::simulation;
use crate::CONFIG;

use log::*;
use std::fs::create_dir_all;
use chrono::Local;

pub fn start() {
    //start timer
    let timer = std::time::Instant::now();
    
    create_dir_all("output").expect("Failed to create output directory");
    let time = Local::now();
    let time = time.format("%Y-%m-%d_%H-%M-%S");
    let filename = format!("output/slop_{time}.mp4");

    let mut encoder =
        VideoEncoder::new(CONFIG.width, CONFIG.height, CONFIG.fps).unwrap();

    let total_frames = CONFIG.fps * CONFIG.duration;

    info!("Recording {} frames at {}x{}", total_frames, CONFIG.width, CONFIG.height);

    // let content: ContentType = TYPES[0];

    // println!("simulation name: {}", content.name);
    // println!("simulation descripticon: {}", content.description);

    //setup simulation
    let (mut balls, mut colliders) = crate::simulation::setup_simulation();
    let mut sound = crate::audio::Audio::init_audio();

    let mut time = std::time::Instant::now();
    for frame_num in 0..total_frames {
        let frame_start = std::time::Instant::now();
        // This draws the ball (ball.rs)
        let img = simulation::new_frame(&mut balls, &mut colliders, &frame_num, &mut sound);
        
        encoder.write_frame(&img);
        
        if frame_num % CONFIG.fps == 0 {
            info!("Frame {}/{}, in {} seconds", frame_num, total_frames, time.elapsed().as_secs_f32());
            time = std::time::Instant::now();
        } else {
            //print which frame is done and frame time
            let time = frame_start.elapsed().as_secs_f32();
            trace!("Frame {}/{}, in {} seconds", frame_num, total_frames, time);        
        }
    }

    sound.finish_audio(total_frames as usize);
    encoder.finish(&filename);
    info!("{} Frames recorded in {} seconds", total_frames, timer.elapsed().as_secs_f32());
    info!("Video saved to {}.", filename);
}
