use g3rs_test_types::G3RsTestConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

pub fn check(input: &G3RsTestConfigChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    let mutation_active = input.mutants_exists
        || input.cargo.profile.contains_key("mutants")
        || input.mutation_hook_active;

    if input.has_tests {
        crate::nextest::nextest_timeouts::check(input, &mut results);
    }
    if mutation_active {
        crate::mutants::cargo_mutants_installed::check(input, &mut results);
        crate::mutants::mutants_toml_exists::check(input, &mut results);
        crate::mutants::mutants_profile_present::check(input, &mut results);
        crate::mutants::mutation_hook_present::check(input, &mut results);
        crate::mutants::mutants_config_sane::check(input, &mut results);
    }

    results
}
