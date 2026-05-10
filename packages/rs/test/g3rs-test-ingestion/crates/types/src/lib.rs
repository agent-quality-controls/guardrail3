//! Public types and error wrappers for the g3rs test ingestion crate.

/// Ingestion error variants surfaced by the test family.
mod error;

#[cfg(feature = "api")]
pub use g3rs_test_types::{
    G3RsTestConfigChecksInput, G3RsTestFileTreeChecksInput, G3RsTestSourceChecksInput,
};

#[cfg(feature = "api")]
pub use error::G3RsTestIngestionError;
