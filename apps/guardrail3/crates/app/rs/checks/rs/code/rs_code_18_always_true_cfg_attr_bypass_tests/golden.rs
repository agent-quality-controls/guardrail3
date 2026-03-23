use std::collections::BTreeSet;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family};

#[test]
fn populated_golden_fixture_has_no_always_true_cfg_attr_hits() {
    let fixture = copy_fixture();

    let results = run_family(fixture.path());

    assert_eq!(files_for_rule(&results, "RS-CODE-18"), BTreeSet::new());
}
