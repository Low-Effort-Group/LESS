use crate::types::ContentType;
use image::RgbaImage;

// Get rid of these println! macros. They are only here for base implementation structure.
/// Music plays with ball
pub fn run(_time: f32) -> RgbaImage {
    // <add cube related stuff here>
    RgbaImage::new(crate::video::WIDTH, crate::video::HEIGHT)
}
