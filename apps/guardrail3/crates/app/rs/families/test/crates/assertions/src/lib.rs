use guardrail3_app_rs_family_test as _;

use guardrail3_app_rs_family_test::CheckResult;

pub mod rs_test_01_inline_test_bodies;
pub mod rs_test_02_owned_sidecar_shape;
pub mod rs_test_03_runtime_assertions_split;
pub mod rs_test_04_ignore_reason;
pub mod rs_test_05_should_panic_expected;
pub mod rs_test_06_tautological_assertions;
pub mod rs_test_07_real_proof_site;
pub mod rs_test_08_weak_matches_assert;
pub mod rs_test_09_nextest_timeouts;
pub mod rs_test_10_input_failures;
pub mod rs_test_11_cargo_mutants_installed;
pub mod rs_test_12_mutants_toml_exists;
pub mod rs_test_13_mutants_profile_present;
pub mod rs_test_14_mutation_hook_present;
pub mod rs_test_15_mutants_config_sane;
pub mod rs_test_16_assertions_modules_prove;
pub mod rs_test_17_external_harnesses_use_assertions;
pub mod rs_test_18_test_support_generic;

pub(crate) fn expected_finding<'a>(results: &'a [CheckResult], rule_id: &str) -> &'a CheckResult {
    results
        .iter()
        .find(|result| result.id == rule_id)
        .unwrap_or_else(|| std::panic::panic_any(format!("expected {rule_id} finding")))
}
