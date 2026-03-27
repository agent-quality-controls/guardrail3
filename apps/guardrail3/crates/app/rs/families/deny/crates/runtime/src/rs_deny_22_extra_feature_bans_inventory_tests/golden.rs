use super::super::build_fixture_deny_toml;

#[test]
fn emits_no_result_for_canonical_feature_ban_state() {
    let results = super::super::run_check(&build_fixture_deny_toml("service"));

    assert!(
        results.is_empty(),
        "expected canonical feature-ban state to avoid extra inventory: {results:#?}"
    );
}
