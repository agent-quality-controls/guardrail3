/// Document accessors that wrap a parsed cspell JSON document.
mod document;
/// Error types returned by the cspell config parser runtime.
mod error;
/// Filesystem boundary that funnels every disk read through one helper.
mod fs;
/// Parser entry points that turn cspell JSON input into a typed document.
mod parser;
#[cfg(feature = "api")]
pub mod types;

#[cfg(feature = "api")]
pub use document::{parse_error_reason, typed};
#[cfg(feature = "api")]
pub use error::Error;
#[cfg(feature = "api")]
pub use parser::{from_path_document, parse_document};
