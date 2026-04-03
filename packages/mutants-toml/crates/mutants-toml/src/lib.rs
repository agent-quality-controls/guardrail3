/// Typed configuration for `.cargo/mutants.toml` parsing.
mod config;
/// Error types for parse failures.
mod error;
/// Filesystem boundary for file reading.
mod fs;

#[cfg(feature = "types")]
pub use config::MutantsConfig;
#[cfg(feature = "types")]
pub use error::Error;
#[cfg(feature = "types")]
pub use toml::Value;
