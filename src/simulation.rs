use crate::video::{HEIGHT, WIDTH};
use image::RgbaImage;
use crate::types::ball::Ball;



pub fn setupSimulation() -> Vec<Ball> {
    let mut ball = Ball {
        x: WIDTH as f32 / 2.0,
        y: HEIGHT as f32 / 2.0,
        vx: 200.0, // pixels per second
        vy: 150.0, // pixels per second
        radius: 30.0,
    };

    let dt = 1.0 / 60.0; // simulate at 60 FPS
    vec![ball]
}

pub fn newFrame(balls: Vec<Ball>) -> RgbaImage {
    RgbaImage::new(WIDTH, HEIGHT)
    
    
}