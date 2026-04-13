use guardrail3_check_types::G3CheckResult;

pub(super) fn run_check(cargo_toml: &str) -> Vec<G3CheckResult> {
    let input = crate::test_support::config_input_for_crate(cargo_toml, None);
    let mut results = Vec::new();
    crate::rs_release_config_07_docs_rs_metadata::check(&input.crates[0], &mut results);
    results
}
