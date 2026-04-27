#[cfg(test)]
use g3rs_hooks_config_checks_assertions as _;

mod cargo_dupes_installed;
mod guardrail_binary_available;
mod required_tools_installed;
mod run;
mod support;

#[cfg(feature = "checks")]
pub use run::check;
