#[cfg(test)]
use g3rs_clippy_filetree_checks_assertions as _;
#[cfg(test)]
use test_support as _;

mod rs_clippy_filetree_01_coverage_exists;
mod rs_clippy_filetree_02_same_root_conflict;
mod run;

#[cfg(feature = "checks")]
pub use run::check;
