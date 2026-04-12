#[cfg(test)]
use g3rs_toolchain_filetree_checks_assertions as _;

mod rs_toolchain_filetree_01_exists;
mod rs_toolchain_filetree_04_legacy_file;
mod run;
#[cfg(test)]
mod test_support;

#[cfg(feature = "checks")]
pub use run::check;
