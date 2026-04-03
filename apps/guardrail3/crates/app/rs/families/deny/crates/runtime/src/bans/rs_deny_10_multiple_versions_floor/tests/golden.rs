use super::helpers::build_fixture_deny_toml;

#[test]
fn emits_no_result_for_generated_multiple_versions_floor() {
    let results = super::helpers::run_check(&build_fixture_deny_toml("service"));

    assert!(
        results.is_empty(),
        "expected canonical multiple-versions floor to pass: {results:#?}"
    );
}
