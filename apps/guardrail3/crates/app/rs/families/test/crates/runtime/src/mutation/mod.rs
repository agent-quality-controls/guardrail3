pub(crate) mod rs_test_09_nextest_timeouts;
pub(crate) mod rs_test_10_input_failures;
pub(crate) mod rs_test_11_cargo_mutants_installed;
pub(crate) mod rs_test_12_mutants_toml_exists;
pub(crate) mod rs_test_13_mutants_profile_present;
pub(crate) mod rs_test_14_mutation_hook_present;
pub(crate) mod rs_test_15_mutants_config_sane;

#[cfg(test)]
mod rs_test_09_nextest_timeouts_tests;
#[cfg(test)]
mod rs_test_10_input_failures_tests;
#[cfg(test)]
mod rs_test_11_cargo_mutants_installed_tests;
#[cfg(test)]
mod rs_test_12_mutants_toml_exists_tests;
#[cfg(test)]
mod rs_test_13_mutants_profile_present_tests;
#[cfg(test)]
mod rs_test_14_mutation_hook_present_tests;
#[cfg(test)]
mod rs_test_15_mutants_config_sane_tests;
