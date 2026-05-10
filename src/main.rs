mod startup;
use crate::startup::startup;

mod video;
mod types;
mod simulation;

use log::*;

use pretty_env_logger::*;
use log::*;

fn main() {
    pretty_env_logger::formatted_builder()
        .parse_filters(&std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()))
        .init();
    info!("Low Effort Slop System initialization...");
    crate::video::setup_encoder();
}
