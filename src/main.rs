mod startup;
use crate::startup::startup;

mod video;
mod types;
mod simulation;

fn main() {
    startup();
    crate::video::setup_encoder();
}
