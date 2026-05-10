/// Document accessors that wrap a parsed `ESLint` config document.
mod document;
/// Error types returned by the `ESLint` config parser runtime.
mod error;
/// Parser entry points that turn `ESLint` config input into a typed document.
mod parser;

#[cfg(feature = "api")]
pub mod types;

#[cfg(feature = "api")]
pub use document::{parse_error_reason, probe, rule_setting, typed};
#[cfg(feature = "api")]
pub use error::Error;
#[cfg(feature = "api")]
pub use parser::{from_path, parse, parse_document};
