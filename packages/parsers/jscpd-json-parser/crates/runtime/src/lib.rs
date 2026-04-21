mod document;
mod error;
mod fs;
mod parser;
#[cfg(feature = "api")]
pub mod types;

#[cfg(feature = "api")]
pub use document::{parse_error_reason, typed};
#[cfg(feature = "api")]
pub use error::Error;
#[cfg(feature = "api")]
pub use parser::{from_path, from_path_document, parse, parse_document};

#[cfg(test)]
use tempfile as _;
