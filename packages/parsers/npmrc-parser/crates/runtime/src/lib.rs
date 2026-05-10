/// Document accessors that wrap a parsed `.npmrc` document.
mod document;
/// Error types returned by the `.npmrc` parser runtime.
mod error;
/// Filesystem boundary that funnels every disk read through one helper.
mod fs;
/// Parser entry points that turn `.npmrc` input into a typed document.
mod parser;
#[cfg(feature = "api")]
pub mod types;

#[cfg(feature = "api")]
pub use document::{effective_value, parse_error_reason, typed};
#[cfg(feature = "api")]
pub use error::Error;
#[cfg(feature = "api")]
pub use parser::{from_path, from_path_document, parse, parse_document};

#[cfg(test)]
use tempfile as _;
