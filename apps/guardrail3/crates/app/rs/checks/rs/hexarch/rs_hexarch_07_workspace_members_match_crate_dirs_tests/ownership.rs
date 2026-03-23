use super::super::super::test_support::{copy_fixture, errors_by_id, run_family, write_file};

#[test]
fn malformed_app_cargo_is_owned_by_rule_08_not_rule_07() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "apps/devctl/Cargo.toml", "[workspace");

    let results = run_family(tmp.path());
    let rule_07 = errors_by_id(&results, "RS-HEXARCH-07");
    let rule_08 = errors_by_id(&results, "RS-HEXARCH-08");
    assert!(
        rule_07.is_empty(),
        "rule 07 should skip parse-broken app cargo: {rule_07:#?}"
    );
    assert_eq!(
        rule_08.len(),
        1,
        "rule 08 should own parse errors: {rule_08:#?}"
    );
}
