mod ball;
mod cube;

use image::RgbaImage;

/// This struct defines the type of content, and describes its properties. Example: A ball bouncing physics simulation
#[derive(Debug)]
pub struct ContentType {
    /// Describes the name of the content. Can be used for file names and such.
    name: &'static str,
    /// Describes the purpose of the type. Something like this shall be included in description/title of video.
    description: &'static str,
    /// Pointer to the function to be called. Arguments and return type subject to change.
    call: fn(&ContentType, f32) -> RgbaImage,
}

impl ContentType {
    pub fn invoke(&self, time: f32) -> RgbaImage {
        (self.call)(self, time)
    }
}

// For these content options, we need to change name and description to something less "formal"

/// Ball bouncing
pub const TYPE_BALL: ContentType = ContentType {
    name: "Ball",
    description: "Physics simulation where ball bounces",
    call: crate::types::ball::run,
};

/// Cube music
const TYPE_CUBE_MUSIC: ContentType = ContentType {
    name: "Cube music",
    description: "A bouncing cube playing music",
    call: crate::types::cube::run,
};

const TYPES: &[ContentType] = &[TYPE_BALL, TYPE_CUBE_MUSIC];

pub fn call_id(id: usize) {
    TYPES[id].invoke(0.0);
}
