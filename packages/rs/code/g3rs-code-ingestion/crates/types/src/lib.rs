//! Type definitions shared across the `g3rs-code-ingestion` family.

/// Rule implementation for `error`.
mod error;

#[cfg(feature = "api")]
pub use error::G3RsCodeIngestionError;
