use super::super::build_fixture_deny_toml;

#[test]
fn emits_no_result_for_generated_unknown_sources_policy() {
    let results = super::super::run_check(&build_fixture_deny_toml("service"));

    assert!(
        results.is_empty(),
        "expected canonical unknown-sources policy to pass: {results:#?}"
    );
}
