use guardrail3_check_types::G3CheckResult;

pub(super) fn run_check(release_plz_toml: &str) -> Vec<G3CheckResult> {
    let input = crate::test_support::config_input_for_repo(Some(release_plz_toml), None);
    let mut results = Vec::new();
    crate::rs_release_config_10_release_plz_baseline::check(
        input.repo.as_ref().expect("repo should exist"),
        &mut results,
    );
    results
}
