use super::super::super::test_support::{assert_no_error, copy_fixture, run_family};

#[test]
fn golden_fixture_has_no_adapter_public_trait_errors() {
    let tmp = copy_fixture();
    let results = run_family(tmp.path());
    assert_no_error(&results, "RS-HEXARCH-23");
}
