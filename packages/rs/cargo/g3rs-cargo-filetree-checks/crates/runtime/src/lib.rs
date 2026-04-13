mod rs_cargo_filetree_10_missing_member_cargo;
mod rs_cargo_filetree_14_input_failures;
mod run;

#[cfg(test)]
use g3rs_cargo_filetree_checks_assertions as _;

#[cfg(feature = "checks")]
pub use run::check;
