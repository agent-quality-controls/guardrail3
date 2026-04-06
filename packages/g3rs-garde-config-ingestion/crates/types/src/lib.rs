/// Error types for garde config ingestion.
mod error;

#[cfg(feature = "api")]
pub use error::G3RsGardeConfigIngestionError;
