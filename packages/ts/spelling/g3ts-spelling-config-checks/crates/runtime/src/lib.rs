/// Internal `common` module.
mod common;
/// Internal `cspell_config_present` module.
mod cspell_config_present;
/// Internal `cspell_package_present` module.
mod cspell_package_present;
/// Internal `policy_configured` module.
mod policy_configured;
/// Internal `run` module.
mod run;
/// Internal `spellcheck_fail_closed` module.
mod spellcheck_fail_closed;
/// Internal `spellcheck_script` module.
mod spellcheck_script;
/// Internal `syncpack_cspell_pin` module.
mod syncpack_cspell_pin;
/// Internal `validate_runs_spellcheck` module.
mod validate_runs_spellcheck;

#[cfg(feature = "api")]
pub use run::check;
