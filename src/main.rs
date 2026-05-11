mod video;
mod types;
mod simulation;
mod audio;
mod config;

use log::*;
use crate::config::Config;
use once_cell::sync::Lazy;

static CONFIG: Lazy<Config> = Lazy::new(|| Config::read_args("less.config.json").unwrap());

// use std::sync::OnceLock;

// static CONFIG: OnceLock<Config> = OnceLock::new();

fn main() {
    pretty_env_logger::formatted_builder()
        .parse_filters(&std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()))
        .init();
    // CONFIG.get_or_init(|| Config::read_args("CONFIG.toml").unwrap());
    
    info!("Low Effort Slop System initialization...");
    crate::video::start();
}
