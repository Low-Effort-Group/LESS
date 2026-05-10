use crate::video::{HEIGHT, WIDTH};
use image::RgbaImage;
use crate::types::ball::Ball;
use crate::types::objects::Circle;
use hsl::HSL;


pub fn setupSimulation() -> (Vec<Ball>, Vec<Circle>) {
    let mut ball1 = Ball {
        x: WIDTH as f32 / 2.0,
        y: HEIGHT as f32 / 2.0,
        vx: 200.0, // pixels per second
        vy: 150.0, // pixels per second
        radius: 30.0,
        gravity: 0.0, // pixels per second squared
        restitution: 1.0, // bounciness self.x = WIDTH as f32 - self.radius;
        friction: 0.8, // friction on bounce
        color: HSL { h: 240.0, s: 1.0, l: 0.5 },
    };
    // let mut ball2 = Ball {
    //     x: WIDTH as f32 / 3.0,
    //     y: HEIGHT as f32 / 3.0,
    //     vx: -150.0, // pixels per second
    //     vy: 100.0, // pixels per second
    //     radius: 20.0,
    //     gravity: 0.0, // pixels per second squared
    //     restitution: 1.1, // bounciness
    //     friction: 0.8, // friction on bounce
    //     color: HSL { h: 0.0, s: 1.0, l: 0.5 },
    // };
    let mut circle = Circle {
            x: WIDTH as f32 / 2.0,
            y: HEIGHT as f32 / 2.0,
            radius: 300.0,
            thickness: 5.0,
            color: HSL { h: 120.0, s: 1.0, l: 0.5 },
            normal: false
        };
    let colliders = vec![
        circle
    ];

    let dt = 1.0 / 60.0; // simulate at 60 FPS
    let balls = vec![ball1];
    (balls, colliders)
}

pub fn newFrame(balls: &mut Vec<Ball>, mut colliders: &mut Vec<Circle>) -> RgbaImage {
    let mut img = RgbaImage::new(WIDTH, HEIGHT);

    //draw colliders
    colliders.iter_mut().for_each(|collider| {
        collider.radius -= 1.0; // Shrink the collider
        collider.draw(&mut img);
    });

    //draw balls
    balls.iter_mut().for_each(|ball| {
        ball.update(1.0 / 60.0, &mut colliders);
        ball.draw(&mut img);
    });//move ball to the inside of the collider
    img
}