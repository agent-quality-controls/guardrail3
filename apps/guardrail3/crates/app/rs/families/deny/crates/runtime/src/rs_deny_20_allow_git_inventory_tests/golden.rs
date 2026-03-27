use super::super::build_fixture_deny_toml;

#[test]
fn emits_no_result_when_allow_git_is_empty() {
    let results = super::super::run_check(&build_fixture_deny_toml("service"));

    assert!(
        results.is_empty(),
        "expected empty allow-git baseline to pass: {results:#?}"
    );
}
