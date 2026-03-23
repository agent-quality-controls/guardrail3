use std::collections::BTreeSet;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family, write_file};

#[test]
fn skips_test_file_even_when_it_exceeds_threshold() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let rel = "apps/backend/crates/app/commands/tests/long_case_tests.rs";
    let filler = "fn filler() {}\n".repeat(600);
    write_file(root, rel, &filler);

    let results = run_family(root);

    assert_eq!(files_for_rule(&results, "RS-CODE-09"), BTreeSet::new());
}
