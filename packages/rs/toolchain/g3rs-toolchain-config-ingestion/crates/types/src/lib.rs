/// Error types for toolchain config ingestion.
mod error;

#[cfg(feature = "api")]
pub use error::G3RsToolchainConfigIngestionError;
