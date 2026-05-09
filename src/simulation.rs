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
        gravity: 500.0, // pixels per second squared
    };

    let dt = 1.0 / 60.0; // simulate at 60 FPS
    vec![ball]
}

pub fn newFrame(balls: &mut Vec<Ball>) -> RgbaImage {
    let mut img = RgbaImage::new(WIDTH, HEIGHT);
    balls.into_iter().for_each(|mut ball| {
        ball.update(1.0 / 60.0);
        ball.draw(&mut img);
    });
    img
}