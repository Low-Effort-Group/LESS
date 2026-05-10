mod video;
mod types;
mod simulation;
mod audio;

use pretty_env_logger::*;
use log::*;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "LESS", about = "Low Effort Slop System", long_about = "Low Effort Slop System, A short-form braindead content generator.")]
pub struct Args {
    /// Frames per second
    #[arg(long="fps", default_value_t = 60)]
    fps: u32,

    /// Max duration in seconds
    #[arg(long="duration", short='d', default_value_t = 20)]
    duration: u32,    
}

fn main() {
    let args = Args::parse();
    pretty_env_logger::formatted_builder()
        .parse_filters(&std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()))
        .init();
    info!("Low Effort Slop System initialization...");
    crate::video::start(args);
}
