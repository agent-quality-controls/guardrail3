#[cfg(test)]
use g3rs_clippy_filetree_checks_assertions as _;
#[cfg(test)]
use test_support as _;

/// coverage exists module.
mod coverage_exists;
/// run module.
mod run;
/// same root conflict module.
mod same_root_conflict;

#[cfg(feature = "checks")]
pub use run::check;
