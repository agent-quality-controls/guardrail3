use std::collections::BTreeSet;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family, write_file};

#[test]
fn skips_test_modules_and_src_fs_rs_exemption() {
    let fixture = copy_fixture();
    let root = fixture.path();

    write_file(
        root,
        "apps/backend/crates/app/queries/tests/fs_usage_tests.rs",
        "use std::fs;\n#[test]\nfn probe() { let _ = std::fs::read_to_string(\"fixture\"); }\n",
    );
    write_file(
        root,
        "apps/backend/crates/app/queries/src/fs.rs",
        "use std::fs;\npub fn allowed_probe() { let _ = std::fs::read_to_string(\"fixture\"); }\n",
    );

    let results = run_family(root);

    assert_eq!(files_for_rule(&results, "RS-CODE-15"), BTreeSet::new());
}
