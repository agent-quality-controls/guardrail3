use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_07_workspace_members_match_crate_dirs as assertions;
use test_support::copy_fixture;

#[test]
fn golden_fixture_has_no_rule_07_errors() {
    let tmp = copy_fixture();
    let results = assertions::run_family(tmp.path());
    assert!(
        assertions::errors_by_id(&results, "RS-HEXARCH-07").is_empty(),
        "{results:#?}"
    );
}
