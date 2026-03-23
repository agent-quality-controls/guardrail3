use std::collections::BTreeSet;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family};

#[test]
fn populated_golden_fixture_has_no_unsafe_code_lint_hits() {
    let fixture = copy_fixture();

    let results = run_family(fixture.path());

    assert_eq!(files_for_rule(&results, "RS-CODE-12"), BTreeSet::new());
}
