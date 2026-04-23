mod document;
mod error;
mod parser;
#[cfg(feature = "api")]
pub mod types;

#[cfg(feature = "api")]
pub use document::{has_integration, integration, parse_error_reason, typed};
#[cfg(feature = "api")]
pub use error::Error;
#[cfg(feature = "api")]
pub use parser::{from_path, parse, parse_document};

#[cfg(test)]
use tempfile as _;
