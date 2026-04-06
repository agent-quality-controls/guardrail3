/// Error types for garde config ingestion.
mod error;
/// Result type for garde config ingestion.
mod result;

#[cfg(feature = "api")]
pub use error::G3RsGardeConfigIngestionError;
#[cfg(feature = "api")]
pub use result::G3RsGardeConfigIngestionResult;
