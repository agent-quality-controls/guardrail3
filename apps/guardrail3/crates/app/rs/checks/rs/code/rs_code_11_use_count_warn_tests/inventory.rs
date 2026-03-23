use std::collections::BTreeSet;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family, write_file};

#[test]
fn warns_at_threshold_band_in_real_owned_file() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let rel = "apps/backend/crates/app/queries/src/lib.rs";
    let content = std::fs::read_to_string(root.join(rel)).expect("read source");
    let imports = (0..16)
        .map(|index| format!("use crate::warn_{index};"))
        .collect::<Vec<_>>()
        .join("\n");

    write_file(root, rel, &format!("{imports}\n{content}"));

    let results = run_family(root);

    assert_eq!(
        files_for_rule(&results, "RS-CODE-11"),
        BTreeSet::from([rel.to_owned()])
    );
}
