use crate::types::ContentType;

// Get rid of these println! macros. They are only here for base implementation structure.
/// Music plays with ball
pub fn run(content: &ContentType) -> () {
    println!("simulation name: {}", content.name);
    println!("simulation description: {}", content.description);
    // <add cube related stuff here>
    ()
}