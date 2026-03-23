use std::collections::BTreeSet;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family, write_file};

#[test]
fn skips_test_files_and_src_fs_rs_exemption() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let test_rel = "apps/backend/crates/app/commands/tests/fs_glob.rs";
    let exempt_rel = "apps/backend/crates/app/commands/src/fs.rs";

    write_file(root, test_rel, "use std::fs::*;\n#[test]\nfn smoke() {}\n");
    write_file(
        root,
        exempt_rel,
        "use std::fs::*;\npub fn allowed_probe() {}\n",
    );

    let results = run_family(root);

    assert_eq!(files_for_rule(&results, "RS-CODE-21"), BTreeSet::new());
}
