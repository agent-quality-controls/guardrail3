#[cfg(feature = "api")]
use g3ts_spelling_config_checks_runtime as _;

#[cfg(feature = "api")]
pub mod cspell_config_present;
#[cfg(feature = "api")]
pub mod cspell_package_present;
#[cfg(feature = "api")]
pub mod policy_configured;
#[cfg(feature = "api")]
pub mod run;
#[cfg(feature = "api")]
pub mod spellcheck_fail_closed;
#[cfg(feature = "api")]
pub mod spellcheck_script;
#[cfg(feature = "api")]
pub mod syncpack_cspell_pin;
#[cfg(feature = "api")]
pub mod validate_runs_spellcheck;
