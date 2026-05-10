//! Runtime crate for the stylelint config parser.

/// Typed-document accessor helpers exposed via the public API.
mod document;
/// Error variants surfaced by the parser entry points.
mod error;
/// Parse implementation that drives the Node.js helper.
mod parser;

/// Re-exports for the typed document model (feature `api`).
#[cfg(feature = "api")]
pub mod types;

#[cfg(feature = "api")]
pub use document::{parse_error_reason, probe, rule_setting, typed};
#[cfg(feature = "api")]
pub use error::Error;
#[cfg(feature = "api")]
pub use parser::{from_path, parse, parse_document};
