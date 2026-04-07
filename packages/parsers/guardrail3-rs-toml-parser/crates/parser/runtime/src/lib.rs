/// Error surface for parser failures.
mod error;
/// Centralized filesystem boundary for parser file reads.
mod fs;
/// Parser module facade.
mod parser;

#[cfg(feature = "api")]
pub use error::Error;
#[cfg(feature = "api")]
pub use guardrail3_rs_toml_parser_types::{
    Guardrail3RsToml, RustChecksConfig, RustProfile, WaiverConfig,
};
#[cfg(feature = "api")]
pub use parser::{from_path, parse};
#[cfg(feature = "api")]
pub use toml::Value;
