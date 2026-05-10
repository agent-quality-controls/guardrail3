//! Shared types for the g3rs deps ingestion family.

/// Error type definitions exposed via the facade re-exports.
mod error;

#[cfg(feature = "api")]
pub use error::G3RsDepsIngestionError;
