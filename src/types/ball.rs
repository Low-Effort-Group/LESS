use hsl::HSL;
use crate::video::graphics::*;
use image::{RgbaImage};

use crate::video::{HEIGHT, WIDTH};

#[derive(Clone)]
pub struct Ball {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub radius: f32,
    pub gravity: f32,
    pub restitution: f32,
}

impl Ball {
    pub fn update(&mut self, dt: f32) {
        // Apply gravity
        self.vy += self.gravity * dt;

        // Update position
        self.x += self.vx * dt;
        self.y += self.vy * dt;

        // Bounce off walls
        if self.x - self.radius < 0.0 {
            self.x = self.radius;
            self.vx = -self.vx;
        } else if self.x + self.radius > WIDTH as f32 {
            self.x = WIDTH as f32 - self.radius;
            self.vx = -self.vx;
        }

        // Bounce off floor and ceiling
        if self.y - self.radius < 0.0 {
            self.y = self.radius;
            self.vy = -self.vy * self.restitution;
        } else if self.y + self.radius > HEIGHT as f32 {
            self.y = HEIGHT as f32 - self.radius;
            self.vy = -self.vy * self.restitution;
        }
    }
    pub fn draw(&self, img: &mut RgbaImage) {
        let color = HSL {
            h: 180.0,
            s: 1.0,
            l: 0.5,
        };
        draw_circle(
            img,
            self.x as i32,
            self.y as i32,
            self.radius as i32,
            color.to_rgb(),
        );
    }
}