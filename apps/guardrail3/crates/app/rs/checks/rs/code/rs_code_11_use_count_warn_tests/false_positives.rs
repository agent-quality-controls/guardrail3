use std::collections::BTreeSet;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family, write_file};

#[test]
fn skips_below_threshold_and_test_files() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let rel = "apps/backend/crates/app/commands/tests/use_band_tests.rs";
    let imports = (0..20)
        .map(|index| format!("use crate::test_{index};"))
        .collect::<Vec<_>>()
        .join("\n");

    write_file(root, rel, &imports);

    let results = run_family(root);

    assert_eq!(files_for_rule(&results, "RS-CODE-11"), BTreeSet::new());
}
