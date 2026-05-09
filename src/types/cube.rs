use crate::types::ContentType;
use image::RgbaImage;

// Get rid of these println! macros. They are only here for base implementation structure.
/// Music plays with ball
pub fn run(content: &ContentType, _time: f32) -> RgbaImage {
    println!("simulation name: {}", content.name);
    println!("simulation description: {}", content.description);
    // <add cube related stuff here>
    RgbaImage::new(crate::video::WIDTH, crate::video::HEIGHT)
}
