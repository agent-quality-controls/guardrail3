#[cfg(test)]
use g3rs_test_config_checks_assertions as _;

mod rs_test_config_09_nextest_timeouts;
mod rs_test_config_11_cargo_mutants_installed;
mod rs_test_config_12_mutants_toml_exists;
mod rs_test_config_13_mutants_profile_present;
mod rs_test_config_14_mutation_hook_present;
mod rs_test_config_15_mutants_config_sane;
mod run;
#[cfg(test)]
mod test_helpers;

#[cfg(feature = "checks")]
pub use run::check;
