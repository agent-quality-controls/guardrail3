mod cargo_lock_present;
mod gitignore_not_ignoring_cargo_lock;
mod run;
#[cfg(test)]
use g3rs_deps_filetree_checks_assertions as _;

#[cfg(feature = "checks")]
pub use run::check;
