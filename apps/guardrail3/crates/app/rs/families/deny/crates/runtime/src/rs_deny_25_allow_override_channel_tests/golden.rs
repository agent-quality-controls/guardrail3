use super::super::build_fixture_deny_toml;

#[test]
fn emits_no_result_when_bans_allow_list_is_absent() {
    let results = super::super::run_check(&build_fixture_deny_toml("service"));

    assert!(
        results.is_empty(),
        "expected canonical bans state to avoid allow overrides: {results:#?}"
    );
}
