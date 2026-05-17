//! Runtime crate for the `.jscpd.json` parser facade.

/// Typed-document accessor helpers exposed via the public API.
mod document;
/// Error variants surfaced by the parser entry points.
mod error;
/// Filesystem ingress helpers for `.jscpd.json` files.
mod fs;
/// Parse implementation that drives serde-based normalization.
mod parser;
/// Re-exports for the typed document model (feature `api`).
#[cfg(feature = "api")]
pub mod types;

#[cfg(feature = "api")]
pub use document::{parse_error_reason, typed};
#[cfg(feature = "api")]
pub use error::Error;
#[cfg(feature = "api")]
pub use parser::{from_path, from_path_document, parse, parse_document};
