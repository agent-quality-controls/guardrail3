/// Error types for garde ingestion.
mod error;

#[cfg(feature = "api")]
pub use error::G3RsGardeIngestionError;
