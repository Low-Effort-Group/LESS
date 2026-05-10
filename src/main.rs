mod startup;
use crate::startup::startup;

mod video;
mod types;
mod simulation;

use log::*;

fn main() {
    startup();
    info!("Starting simulation...");
    crate::video::setup_encoder();
}
