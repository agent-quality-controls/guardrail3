mod common;
mod cspell_config_present;
mod cspell_package_present;
mod policy_configured;
mod run;
mod spellcheck_fail_closed;
mod spellcheck_script;
mod syncpack_cspell_pin;
mod validate_runs_spellcheck;

#[cfg(feature = "api")]
pub use run::check;
