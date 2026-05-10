/// Document accessors that wrap a parsed `tsconfig.json` document.
mod document;
/// Error types returned by the `tsconfig.json` parser runtime.
mod error;
/// Filesystem boundary that funnels every disk read through one helper.
mod fs;
/// Parser entry points that turn `tsconfig.json` input into a typed document.
mod parser;
#[cfg(feature = "api")]
pub mod types;

#[cfg(feature = "api")]
pub use document::{
    bool_field_state, compiler_options, extends_entries, parse_error_reason, typed,
};
#[cfg(feature = "api")]
pub use error::Error;
#[cfg(feature = "api")]
pub use parser::{from_path, from_path_document, parse, parse_document};

#[cfg(test)]
use tempfile as _;
