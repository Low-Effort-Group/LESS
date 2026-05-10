use pretty_env_logger::*;
use log::*;

// This part of the program initializes and checks all before starting, and defines configuration variables.

/// Initializes the program.
/// This reads and returns all configuration.
pub fn startup() {
    pretty_env_logger::formatted_builder()
        .parse_filters(&std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()))
        .init();
    info!("Low Effort Slop System initialization...");
}
