#[cfg(test)]
use g3rs_fmt_filetree_checks_assertions as _;
#[cfg(test)]
use test_support as _;

mod rs_fmt_filetree_01_exists;
mod rs_fmt_filetree_05_per_crate_override;
mod rs_fmt_filetree_08_dual_file_conflict;
mod run;

#[cfg(feature = "checks")]
pub use run::check;
