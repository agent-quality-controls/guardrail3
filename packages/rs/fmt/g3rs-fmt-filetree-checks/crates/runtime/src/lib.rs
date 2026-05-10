//! Runtime rules for the `g3rs-fmt-filetree-checks` family.

#[cfg(test)]
use g3rs_fmt_filetree_checks_assertions as _;
#[cfg(test)]
use test_support as _;

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
