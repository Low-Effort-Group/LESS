use pretty_env_logger::*;
use log::*;

// This part of the program initializes and checks all before starting, and defines configuration variables.

/// Initializes the program.
/// This reads and returns all configuration.
pub fn startup() {
    // pretty_env_logger::formatted_timed_builder()
    //     .filter_level(log::LevelFilter::Info)
    //     .init();
    pretty_env_logger::formatted_builder()
        .filter_level(log::LevelFilter::Info)
        .init();
    info!("Low Effort Slop System initialization...");
}
