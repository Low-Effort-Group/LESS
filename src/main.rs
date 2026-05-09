mod startup;
use crate::startup::startup;

mod video;
mod types;

fn main() {
    startup();
    crate::video::setup_encoder();
}
