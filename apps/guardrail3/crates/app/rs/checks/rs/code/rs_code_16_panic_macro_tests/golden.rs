use std::collections::BTreeSet;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family};

#[test]
fn populated_golden_fixture_has_no_panic_macro_hits() {
    let fixture = copy_fixture();

    let results = run_family(fixture.path());
    let rs_code_16_results = results
        .iter()
        .filter(|result| result.id == "RS-CODE-16")
        .collect::<Vec<_>>();

    assert_eq!(files_for_rule(&results, "RS-CODE-16"), BTreeSet::new());
    assert!(rs_code_16_results.is_empty());
}
