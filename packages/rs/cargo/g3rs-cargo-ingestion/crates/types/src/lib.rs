/// Error types for Cargo ingestion.
mod error;

#[cfg(feature = "api")]
pub use error::G3RsCargoIngestionError;
