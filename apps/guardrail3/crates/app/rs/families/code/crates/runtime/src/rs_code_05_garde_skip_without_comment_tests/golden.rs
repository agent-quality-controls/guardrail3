use std::collections::BTreeSet;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family};

#[test]
fn populated_golden_fixture_has_no_undocumented_garde_skip_hits() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let results = run_family(root);

    assert_eq!(files_for_rule(&results, "RS-CODE-05"), BTreeSet::new());
}
