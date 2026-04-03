mod config;
mod error;

#[cfg(feature = "types")]
pub use config::RustfmtConfig;
#[cfg(feature = "types")]
pub use error::Error;
#[cfg(feature = "types")]
pub use toml::Value;
