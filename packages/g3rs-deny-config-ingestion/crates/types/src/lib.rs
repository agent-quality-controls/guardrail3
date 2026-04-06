/// Error types for deny config ingestion.
mod error;

#[cfg(feature = "api")]
pub use error::G3RsDenyConfigIngestionError;
