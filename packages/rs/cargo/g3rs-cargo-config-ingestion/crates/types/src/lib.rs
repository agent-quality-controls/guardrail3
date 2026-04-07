/// Error types for Cargo config ingestion.
mod error;

#[cfg(feature = "api")]
pub use error::G3RsCargoConfigIngestionError;
