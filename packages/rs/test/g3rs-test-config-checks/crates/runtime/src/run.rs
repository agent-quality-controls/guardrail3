use g3rs_test_config_checks_types::G3RsTestConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

pub fn check(input: &G3RsTestConfigChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    let mutation_active =
        input.mutants_exists || input.cargo.profile.contains_key("mutants") || input.mutation_hook_active;

    if input.has_tests {
        crate::rs_test_config_09_nextest_timeouts::check(input, &mut results);
    }
    if mutation_active {
        crate::rs_test_config_11_cargo_mutants_installed::check(input, &mut results);
        crate::rs_test_config_12_mutants_toml_exists::check(input, &mut results);
        crate::rs_test_config_13_mutants_profile_present::check(input, &mut results);
        crate::rs_test_config_14_mutation_hook_present::check(input, &mut results);
        crate::rs_test_config_15_mutants_config_sane::check(input, &mut results);
    }

    results
}
