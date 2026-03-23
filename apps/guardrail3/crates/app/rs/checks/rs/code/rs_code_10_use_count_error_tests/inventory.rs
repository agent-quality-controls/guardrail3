use std::collections::BTreeSet;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family, write_file};

#[test]
fn attacks_excessive_top_level_use_counts_in_real_owned_file() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let rel = "apps/backend/crates/adapters/inbound/rest/src/lib.rs";
    let content = std::fs::read_to_string(root.join(rel)).expect("read source");
    let imports = (0..21)
        .map(|index| format!("use crate::synthetic_{index};"))
        .collect::<Vec<_>>()
        .join("\n");

    write_file(root, rel, &format!("{imports}\n{content}"));

    let results = run_family(root);

    assert_eq!(
        files_for_rule(&results, "RS-CODE-10"),
        BTreeSet::from([rel.to_owned()])
    );
}
