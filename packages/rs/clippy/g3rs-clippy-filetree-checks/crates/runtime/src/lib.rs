#[cfg(test)]
use g3rs_clippy_filetree_checks_assertions as _;
#[cfg(test)]
use test_support as _;

mod coverage_exists;
mod run;
mod same_root_conflict;

#[cfg(feature = "checks")]
pub use run::check;
