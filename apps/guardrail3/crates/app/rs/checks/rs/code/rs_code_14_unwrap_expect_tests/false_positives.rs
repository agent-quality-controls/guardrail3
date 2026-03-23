use std::collections::BTreeSet;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family, write_file};

#[test]
fn skips_allow_scoped_unwrap_usage() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let rel = "apps/backend/crates/app/queries/src/lib.rs";
    let content = std::fs::read_to_string(root.join(rel)).expect("read source");

    write_file(
        root,
        rel,
        &format!(
            "{content}\n#[allow(clippy::unwrap_used)]\nfn allowed_probe() {{ let _ = Some(1).unwrap(); }}\n"
        ),
    );

    let results = run_family(root);

    assert_eq!(files_for_rule(&results, "RS-CODE-14"), BTreeSet::new());
}
