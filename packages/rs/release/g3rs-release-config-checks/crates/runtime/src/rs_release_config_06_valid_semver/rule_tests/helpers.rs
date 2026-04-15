use guardrail3_check_types::G3CheckResult;

pub(super) fn run_check(cargo_toml: &str) -> Vec<G3CheckResult> {
    run_check_with_workspace(cargo_toml, None)
}

pub(super) fn run_check_with_workspace(
    cargo_toml: &str,
    workspace_cargo_toml: Option<&str>,
) -> Vec<G3CheckResult> {
    let input =
        crate::test_support::config_input_for_publishable_crate(cargo_toml, workspace_cargo_toml);
    let mut results = Vec::new();
    crate::rs_release_config_06_valid_semver::check(&input.crates[0], &mut results);
    results
}
