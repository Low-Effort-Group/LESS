mod startup;
use crate::startup::startup;

mod types;
mod video;

fn main() {
    startup();
    let id: usize = rand::random_range(0..2);
    crate::types::call_id(id);
    crate::video::setup_encoder();
}
