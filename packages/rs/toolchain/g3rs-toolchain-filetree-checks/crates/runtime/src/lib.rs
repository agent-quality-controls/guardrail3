#[cfg(test)]
use g3rs_toolchain_filetree_checks_assertions as _;

/// `exists` module.
mod exists;
/// `legacy_file` module.
mod legacy_file;
/// `run` module.
mod run;

#[cfg(feature = "checks")]
pub use run::check;
