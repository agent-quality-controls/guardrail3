/// Error types for clippy config ingestion.
mod error;

#[cfg(feature = "api")]
pub use error::G3RsClippyConfigIngestionError;
