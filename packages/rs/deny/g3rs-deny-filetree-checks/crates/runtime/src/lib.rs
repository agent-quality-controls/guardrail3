//! File-tree checks for the g3rs deny family.

/// Coverage rule: requires presence of a `deny.toml` configuration.
mod coverage;
/// Public dispatch entry point combining all file-tree rules.
mod run;
/// Shadowing rule: rejects multiple deny configurations at the same root.
mod shadowing;

#[cfg(feature = "checks")]
pub use run::check;
