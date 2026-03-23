use std::collections::BTreeSet;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family, write_file};

#[test]
fn attacks_std_fs_glob_imports_across_multiple_owned_files() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let backend_rel = "apps/backend/crates/adapters/outbound/postgres/src/lib.rs";
    let devctl_rel = "apps/devctl/crates/adapters/outbound/fs/src/lib.rs";

    let backend_content =
        std::fs::read_to_string(root.join(backend_rel)).expect("read backend source");
    let devctl_content =
        std::fs::read_to_string(root.join(devctl_rel)).expect("read devctl source");

    write_file(
        root,
        backend_rel,
        &format!("use std::fs::*;\n{backend_content}"),
    );
    write_file(
        root,
        devctl_rel,
        &format!("use std::{{fs::*, path::PathBuf}};\n{devctl_content}"),
    );

    let results = run_family(root);

    assert_eq!(
        files_for_rule(&results, "RS-CODE-21"),
        BTreeSet::from([backend_rel.to_owned(), devctl_rel.to_owned()])
    );
}
