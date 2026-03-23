use std::collections::BTreeSet;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family, write_file};

#[test]
fn skips_same_line_reason_documented_attrs() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let backend_rel = "apps/backend/crates/domain/types/src/lib.rs";
    let backend_content =
        std::fs::read_to_string(root.join(backend_rel)).expect("read backend source");

    write_file(
        root,
        backend_rel,
        &format!(
            "{backend_content}\n#[deny(clippy::panic)] // reason: domain models stay panic free\nfn documented_probe() {{}}\n"
        ),
    );

    let results = run_family(root);

    assert_eq!(files_for_rule(&results, "RS-CODE-22"), BTreeSet::new());
}
