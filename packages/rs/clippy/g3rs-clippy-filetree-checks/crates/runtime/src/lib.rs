#[cfg(test)]
use g3rs_clippy_filetree_checks_assertions as _;

mod rs_clippy_filetree_01_coverage_exists;
mod rs_clippy_filetree_02_same_root_conflict;
mod run;
#[cfg(test)]
mod test_support;

#[cfg(feature = "checks")]
pub use run::check;
