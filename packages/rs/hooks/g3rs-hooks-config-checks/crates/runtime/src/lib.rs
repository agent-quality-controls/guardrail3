#[cfg(test)]
use g3rs_hooks_config_checks_assertions as _;

mod hook_rs_06_required_tools_installed;
mod hook_rs_14_guardrail_binary_available;
mod hook_rs_15_cargo_dupes_installed;
mod run;
mod support;

#[cfg(feature = "checks")]
pub use run::check;
