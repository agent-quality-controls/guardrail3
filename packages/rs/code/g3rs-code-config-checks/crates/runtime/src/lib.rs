#[cfg(test)]
use g3rs_code_config_checks_assertions as _;

mod exception_comment_inventory;
mod run;
mod unsafe_code_lint;

#[cfg(feature = "checks")]
pub use run::check;
