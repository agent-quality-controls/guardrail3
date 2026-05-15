//! JSON fixture output for `g3rs-code-ingestion`.

/// Fixture-output command implementation.
mod run;

#[cfg(feature = "api")]
pub use run::{FixtureOutputError, render_path, run_from_env};
