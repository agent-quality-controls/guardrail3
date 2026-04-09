/// Error types for deny ingestion.
mod error;

#[cfg(feature = "api")]
pub use error::G3RsDenyIngestionError;
