#[cfg(test)]
use g3rs_code_config_checks_assertions as _;

/// exception comment inventory module.
mod exception_comment_inventory;
/// run module.
mod run;
/// unsafe code lint module.
mod unsafe_code_lint;

#[cfg(feature = "checks")]
pub use run::check;
