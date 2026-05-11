use crate::types::HSL;

pub struct Circle {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
    pub thickness: f32,
    pub color: HSL,
    pub normal: bool
}

impl Circle {
    pub fn draw(&self, img: &mut image::RgbaImage) {
        crate::video::graphics::draw_circle(
            img,
            self.x as i32,
            self.y as i32,
            self.radius as i32 + self.thickness as i32 / 2,
            self.color.to_rgb(),
        );
        crate::video::graphics::draw_circle(
            img,
            self.x as i32,
            self.y as i32,
            self.radius as i32 - self.thickness as i32 / 2,
            (0, 0, 0),
        );
    }
}