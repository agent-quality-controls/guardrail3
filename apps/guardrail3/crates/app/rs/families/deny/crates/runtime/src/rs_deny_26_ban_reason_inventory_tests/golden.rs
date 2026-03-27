use super::super::build_fixture_deny_toml;

#[test]
fn emits_no_result_for_reasoned_canonical_ban_entries() {
    let results = super::super::run_check(&build_fixture_deny_toml("service"));

    assert!(
        results.is_empty(),
        "expected canonical deny entries to have reasons: {results:#?}"
    );
}
