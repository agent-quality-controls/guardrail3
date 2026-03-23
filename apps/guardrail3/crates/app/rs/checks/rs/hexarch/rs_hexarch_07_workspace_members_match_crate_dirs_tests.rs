use super::super::test_support::{copy_fixture, errors_by_id, run_family, write_file};

#[test]
fn crate_not_in_workspace_members_is_error() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/devctl/crates/domain/events/Cargo.toml",
        "[package]\nname = \"devctl-domain-events\"\nversion = \"0.1.0\"\n",
    );
    write_file(
        tmp.path(),
        "apps/devctl/crates/domain/events/src/lib.rs",
        "// events",
    );

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-07");
    assert_eq!(errors.len(), 1, "expected one missing-member error: {errors:#?}");
    assert!(errors[0].title.contains("not a workspace member"));
    assert!(errors[0].title.contains("crates/domain/events"));
}

#[test]
fn malformed_app_cargo_is_owned_by_rule_08_not_rule_07() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "apps/devctl/Cargo.toml", "[workspace");

    let results = run_family(tmp.path());
    let rule_07 = errors_by_id(&results, "RS-HEXARCH-07");
    let rule_08 = errors_by_id(&results, "RS-HEXARCH-08");
    assert!(rule_07.is_empty(), "rule 07 should skip parse-broken app cargo: {rule_07:#?}");
    assert_eq!(rule_08.len(), 1, "rule 08 should own parse errors: {rule_08:#?}");
}
