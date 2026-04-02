use super::super::build_fixture_deny_toml;

#[test]
fn emits_no_result_for_generated_service_ban_baseline() {
    let results = super::super::run_check(&build_fixture_deny_toml("service"));

    assert!(
        results.is_empty(),
        "expected canonical service deny baseline to pass: {results:#?}"
    );
}
