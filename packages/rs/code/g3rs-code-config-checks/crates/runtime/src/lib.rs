#[cfg(test)]
use g3rs_code_config_checks_assertions as _;

mod rs_code_config_07_exception_comment_inventory;
mod rs_code_config_12_unsafe_code_lint;
mod run;

#[cfg(feature = "checks")]
pub use run::check;
