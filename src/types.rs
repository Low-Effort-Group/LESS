pub mod ball;
pub mod cube;
pub mod objects;

use hsl::HSL as NoSerdeHSL;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct HSL {
    pub h: f64,
    pub s: f64,
    pub l: f64,
}

impl HSL {
    pub fn to_rgb(&self) -> (u8, u8, u8) {
        NoSerdeHSL { h: self.h, s: self.s, l: self.l }.to_rgb()
    }
}

// /// This struct defines the type of content, and describes its properties. Example: A ball bouncing physics simulation
// #[derive(Debug, Copy,Clone)]
// pub struct ContentType {
//     /// Describes the name of the content. Can be used for file names and such.
//     pub name: &'static str,
//     /// Describes the purpose of the type. Something like this shall be included in description/title of video.
//     pub description: &'static str,
//     /// Pointer to the function to be called. Arguments and return type subject to change.
//     pub call: fn(f32) -> RgbaImage,
// }

// impl ContentType {
//     pub fn invoke(&self, time: f32) -> RgbaImage {
//         (self.call)(time)
//     }
// }

// // For these content options, we need to change name and description to something less "formal"

// /// Ball bouncing
// pub const TYPE_BALL: ContentType = ContentType {
//     name: "Ball",
//     description: "Physics simulation where ball bounces",
//     call: crate::types::ball::run,
// };

// /// Cube music
// const TYPE_CUBE_MUSIC: ContentType = ContentType {
//     name: "Cube music",
//     description: "A bouncing cube playing music",
//     call: crate::types::cube::run,
// };

// pub const TYPES: &[ContentType] = &[TYPE_BALL, TYPE_CUBE_MUSIC];