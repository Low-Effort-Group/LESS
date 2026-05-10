use hsl::HSL;
use crate::video::graphics::*;
use image::{RgbaImage};
use rand::Rng;

use crate::video::{HEIGHT, WIDTH};
use crate::types::objects::Circle;

#[derive(Clone)]
pub struct Ball {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub radius: f32,
    pub gravity: f32,
    pub restitution: f32,
    pub friction: f32,
    pub color: HSL,
}

impl Ball {
    pub fn update(&mut self, dt: f32, colliders: &mut Vec<Circle>) {
        // Apply gravity
        self.vy += self.gravity * dt;

        // Update position
        self.x += self.vx * dt;
        self.y += self.vy * dt;

        // Bounce off walls
        if self.x - self.radius < 0.0 {
            self.x = self.radius;
            self.vx = -self.vx * self.friction;
            self.color.h = (self.color.h + 30.0) % 360.0; // Change color on bounce
        } else if self.x + self.radius > WIDTH as f32 {
            self.x = WIDTH as f32 - self.radius;
            self.vx = -self.vx * self.friction;
            self.color.h = (self.color.h + 30.0) % 360.0; // Change color on bounce
        }

        // Bounce off floor and ceiling
        if self.y - self.radius < 0.0 {
            self.y = self.radius;
            self.vy = -self.vy * self.restitution;
            self.color.h = (self.color.h + 30.0) % 360.0; // Change color on bounce
        } else if self.y + self.radius > HEIGHT as f32 {
            self.y = HEIGHT as f32 - self.radius;
            self.vy = -self.vy * self.restitution;
            self.color.h = (self.color.h + 30.0) % 360.0; // Change color on bounce
        }

        // Bounce off colliders
        for collider in colliders {
            let dx = self.x - collider.x;
            let dy = self.y - collider.y;
            let dist = (dx * dx + dy * dy).sqrt();

            //Normals, balls supposed to be outside(true)
            if collider.normal && dist < self.radius + collider.radius {
                // Simple elastic collision response
                self.radius += 2.0;
                let overlap = self.radius + collider.radius - dist;
                let nx = dx / dist;
                let ny = dy / dist;
                self.x += nx * overlap;
                self.y += ny * overlap;
                let dot = self.vx * nx + self.vy * ny;
                self.vx -= 2.0 * dot * nx * self.restitution;
                self.vy -= 2.0 * dot * ny * self.restitution;
                self.color.h = (self.color.h + 30.0) % 360.0; // Change color on bounce
            }
            //Non-normals, ball is supposed to be inside(false)
            else if !collider.normal && dist > collider.radius - self.radius {
                // Simple elastic collision response

                self.radius += 2.0; // Grow the ball when it hits a non-normal collider
                //move ball to the inside of the collider

                //move ball to the inside of the collider
                let overlap = dist - (collider.radius - self.radius);
                let nx = dx / dist;
                let ny = dy / dist;
                self.x -= nx * overlap;
                self.y -= ny * overlap;
                let dot = self.vx * nx + self.vy * ny;
                self.vx -= 2.0 * dot * nx * self.restitution;
                self.vy -= 2.0 * dot * ny * self.restitution;
                self.color.h = (self.color.h + 30.0) % 360.0; // Change color on bounce
                collider.radius += 20.0; // Grow the collider when it hits the ball
            }
        }
    }
    pub fn draw(&self, img: &mut RgbaImage) {
        draw_circle(
            img,
            self.x as i32,
            self.y as i32,
            self.radius as i32,
            self.color.to_rgb(),
        );
    }
}