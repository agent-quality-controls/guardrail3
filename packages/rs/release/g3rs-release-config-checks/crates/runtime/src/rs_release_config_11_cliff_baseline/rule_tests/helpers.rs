use guardrail3_check_types::G3CheckResult;

pub(super) fn run_check(cliff_toml: &str) -> Vec<G3CheckResult> {
    let input = crate::test_support::config_input_for_repo(None, Some(cliff_toml));
    let mut results = Vec::new();
    crate::rs_release_config_11_cliff_baseline::check(
        input.repo.as_ref().expect("repo should exist"),
        &mut results,
    );
    results
}
