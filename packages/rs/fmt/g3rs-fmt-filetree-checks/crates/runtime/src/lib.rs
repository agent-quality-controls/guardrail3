#[cfg(test)]
use g3rs_fmt_filetree_checks_assertions as _;
#[cfg(test)]
use test_support as _;

mod dual_file_conflict;
mod exists;
mod per_crate_override;
mod run;

#[cfg(feature = "checks")]
pub use run::check;
