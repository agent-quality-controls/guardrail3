/// Error surface for parser failures.
mod error;
/// Centralized filesystem boundary for parser file reads.
mod fs;
/// Parser module facade.
mod parser;

#[cfg(feature = "api")]
pub mod types;
#[cfg(feature = "api")]
pub use error::Error;
#[cfg(feature = "api")]
pub use parser::{from_path, parse};
