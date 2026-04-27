#[cfg(test)]
use g3rs_toolchain_filetree_checks_assertions as _;

mod exists;
mod legacy_file;
mod run;

#[cfg(feature = "checks")]
pub use run::check;
