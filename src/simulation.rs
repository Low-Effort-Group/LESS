use image::RgbaImage;
use crate::types::ball::Ball;
use crate::types::objects::Circle;
use crate::CONFIG;
use crate::types::HSL;

use crate::audio::Audio;

pub fn setup_simulation() -> (Vec<Ball>, Vec<Circle>) {
    let ball1 = Ball {
        x: CONFIG.width as f32 / 2.0 + 100.0,
        y: CONFIG.height as f32 / 2.0,
        vx: 200.0, // pixels per second
        vy: 150.0, // pixels per second
        radius: 30.0,
        gravity: 0.0, // pixels per second squared
        restitution: 1.05, // bounciness self.x = CONFIG.width as f32 - self.radius;
        friction: 0.8, // friction on bounce
        color: HSL { h: 240.0, s: 1.0, l: 0.5 },
    };

    let circle = Circle {
            x: CONFIG.width as f32 / 2.0,
            y: CONFIG.height as f32 / 2.0,
            radius: 300.0,
            thickness: 50.0,
            color: HSL { h: 120.0, s: 1.0, l: 0.5 },
            normal: false
        };
    let circle2 = Circle {
            x: CONFIG.width as f32 / 2.0,
            y: CONFIG.height as f32 / 2.0,
            radius: 75.0,
            thickness: 5.0,
            color: HSL { h: 120.0, s: 1.0, l: 0.5 }.into(),
            normal: true
        };
    let colliders = vec![
        circle,
        circle2
    ];

    let balls = vec![ball1];
    (balls, colliders)
}

pub fn new_frame(balls: &mut Vec<Ball>, mut colliders: &mut Vec<Circle>, frame: &u32, mut sound: &mut Audio) -> RgbaImage {
    let mut img = RgbaImage::new(CONFIG.width, CONFIG.height);

    //draw colliders
    colliders.iter_mut().for_each(|collider| {
        collider.radius -= 0.5; // Shrink the collider
        collider.draw(&mut img);
    });

    //draw balls
    balls.iter_mut().for_each(|ball| {
        ball.update(1.0 / 60.0, 
            &mut colliders,
            &frame,
            &mut sound);
        ball.draw(&mut img);
    });//move ball to the inside of the collider
    img
}