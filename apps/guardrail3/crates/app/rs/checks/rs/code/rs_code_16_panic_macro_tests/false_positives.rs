use std::collections::BTreeSet;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family, write_file};

#[test]
fn skips_panic_macro_in_test_files() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let rel = "apps/backend/crates/app/queries/tests/panic_macro_tests.rs";
    write_file(root, rel, "fn probe() { panic!(\"boom\"); }\n");

    let results = run_family(root);

    assert_eq!(files_for_rule(&results, "RS-CODE-16"), BTreeSet::new());
}
