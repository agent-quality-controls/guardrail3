use std::collections::BTreeSet;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family, write_file};

#[test]
fn attacks_file_length_using_real_owned_file_surface() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let rel = "apps/backend/crates/adapters/inbound/rest/src/lib.rs";
    let content = std::fs::read_to_string(root.join(rel)).expect("read source");
    let filler = "fn filler() {}\n".repeat(501);

    write_file(root, rel, &format!("{content}\n{filler}"));

    let results = run_family(root);

    assert_eq!(
        files_for_rule(&results, "RS-CODE-09"),
        BTreeSet::from([rel.to_owned()])
    );
}
