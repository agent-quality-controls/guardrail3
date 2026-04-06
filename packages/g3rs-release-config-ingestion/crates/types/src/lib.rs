/// Error types for release config ingestion.
mod error;

#[cfg(feature = "api")]
pub use error::G3RsReleaseConfigIngestionError;
