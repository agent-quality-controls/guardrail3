use super::super::build_fixture_deny_toml;

#[test]
fn emits_no_result_for_generated_confidence_threshold() {
    let results = super::super::run_check(&build_fixture_deny_toml("service"));

    assert!(
        results.is_empty(),
        "expected canonical confidence-threshold to pass: {results:#?}"
    );
}
