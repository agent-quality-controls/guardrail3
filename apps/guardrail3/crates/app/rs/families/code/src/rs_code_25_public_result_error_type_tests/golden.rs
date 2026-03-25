use std::collections::BTreeSet;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family};

#[test]
fn populated_golden_fixture_has_no_public_result_error_type_hits() {
    let fixture = copy_fixture();

    let results = run_family(fixture.path());
    let rs_code_25_results = results
        .iter()
        .filter(|result| result.id == "RS-CODE-25")
        .collect::<Vec<_>>();

    assert_eq!(files_for_rule(&results, "RS-CODE-25"), BTreeSet::new());
    assert!(rs_code_25_results.is_empty());
}
