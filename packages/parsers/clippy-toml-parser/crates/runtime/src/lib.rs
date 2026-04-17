/// Document query helpers over passive parser types.
mod document;
/// Error types for parse failures.
mod error;
/// Filesystem boundary for file reading.
mod fs;
/// Parser entrypoints.
mod parser;

#[cfg(feature = "api")]
pub mod types;

#[cfg(feature = "api")]
pub use document::{ban_section, bool_setting, parse_error_reason, top_level_keys, typed};
#[cfg(feature = "api")]
pub use error::Error;
#[cfg(feature = "api")]
pub use parser::{from_path, parse, parse_document};
