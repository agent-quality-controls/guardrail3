use super::helpers::build_fixture_deny_toml;

#[test]
fn emits_no_result_for_generated_allow_wildcard_paths_baseline() {
    let results = super::helpers::run_check(&build_fixture_deny_toml("service"));

    assert!(
        results.is_empty(),
        "expected canonical allow-wildcard-paths baseline to pass: {results:#?}"
    );
}
