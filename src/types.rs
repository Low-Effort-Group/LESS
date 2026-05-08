/// This struct defines the type of content, and describes its properties. Example: A ball bouncing physics simulation
#[derive(Debug)]
pub struct ContentType {
    /// Should be incremented
    id: u32,
    /// Describes the name of the content. Can be used for file names and such.
    name: &'static str,
    /// Describes the purpose of the type. Something like this shall be included in description/title of video.
    description: &'static str,
    /// Pointer to the function to be called. Arguments and return type subject to change.
    pub call: fn(&ContentType, Vec<u32>) -> (),
}

impl ContentType {
    pub fn invoke (&self, v: Vec<u32>) {
        (self.call)(self, v)
    }
}


// Get rid of these println! macros. They are only here for base implementation structure.
/// Bouncing ball
fn ball_bounce(content: &ContentType, test: Vec<u32>) -> () {
    println!("simulation name: {}", content.name);
    println!("simulation description: {}", content.description);
    println!("Provided vector: {:#?}", test);
    // <add ball bounce related stuff here>
    ()
}

/// Music plays with ball
fn cube_music(content: &ContentType, test: Vec<u32>) -> () {
    println!("simulation name: {}", content.name);
    println!("simulation description: {}", content.description);
    println!("Provided vector: {:#?}", test);
    // <add cube related stuff here>
    ()
}

// For these content options, we need to change name and description to something less "formal"

/// Ball bouncing 
pub const CONTENT_BALL: ContentType = ContentType {
    id: 0,
    name: "Ball",
    description: "Physics simulation where ball bounces",
    call: ball_bounce,
};

/// Cube music
pub const CONTENT_CUBE_MUSIC: ContentType = ContentType {
    id: 1,
    name: "Cube music",
    description: "A bouncing cube playing music",
    call: cube_music,
};