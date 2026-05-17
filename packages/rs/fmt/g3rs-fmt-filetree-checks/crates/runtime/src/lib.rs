//! Runtime rules for the `g3rs-fmt-filetree-checks` family.

/// Rule implementation for `dual file conflict`.
mod dual_file_conflict;
/// Rule implementation for `exists`.
mod exists;
/// Rule implementation for `per crate override`.
mod per_crate_override;
/// Family entry point that runs all rules.
mod run;

#[cfg(feature = "checks")]
pub use run::check;
