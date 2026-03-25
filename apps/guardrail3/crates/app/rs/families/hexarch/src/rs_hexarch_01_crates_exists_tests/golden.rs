use super::super::super::test_support::{copy_fixture, errors_by_id, run_family};

#[test]
fn golden_has_no_rule_01_errors() {
    let tmp = copy_fixture();
    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-01");
    assert!(errors.is_empty(), "golden should pass rule 01: {errors:#?}");
}
