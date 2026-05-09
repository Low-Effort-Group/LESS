use crate::video::{HEIGHT, WIDTH};
use image::RgbaImage;
use crate::types::ball::Ball;



pub fn setupSimulation() -> Vec<Ball> {
    let mut ball1 = Ball {
        x: WIDTH as f32 / 2.0,
        y: HEIGHT as f32 / 2.0,
        vx: 200.0, // pixels per second
        vy: 150.0, // pixels per second
        radius: 30.0,
        gravity: 1000.0, // pixels per second squared
        restitution: 0.8, // bounciness
    };
    let mut ball2 = Ball {
        x: WIDTH as f32 / 3.0,
        y: HEIGHT as f32 / 3.0,
        vx: -150.0, // pixels per second
        vy: 100.0, // pixels per second
        radius: 20.0,
        gravity: 1000.0, // pixels per second squared
        restitution: 0.8, // bounciness
    };

    let dt = 1.0 / 60.0; // simulate at 60 FPS
    vec![ball1, ball2]
}

pub fn newFrame(balls: &mut Vec<Ball>) -> RgbaImage {
    let mut img = RgbaImage::new(WIDTH, HEIGHT);
    balls.into_iter().for_each(|mut ball| {
        ball.update(1.0 / 60.0);
        ball.draw(&mut img);
    });
    img
}