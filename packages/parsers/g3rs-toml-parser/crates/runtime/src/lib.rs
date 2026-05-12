/// Error surface for parser failures.
#[cfg(feature = "api")]
mod error;
/// Centralized filesystem boundary for parser file reads.
#[cfg(feature = "api")]
mod fs;
/// Parser module facade.
#[cfg(feature = "api")]
mod parser;

#[cfg(feature = "api")]
pub use error::Error;
#[cfg(feature = "api")]
pub mod types;
#[cfg(feature = "api")]
pub use parser::{from_path, parse};

#[cfg(not(feature = "api"))]
use g3rs_toml_parser_types as _;
#[cfg(not(feature = "api"))]
use toml as _;
