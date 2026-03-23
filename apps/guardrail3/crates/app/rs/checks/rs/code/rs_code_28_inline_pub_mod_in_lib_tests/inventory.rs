use std::collections::BTreeSet;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family, write_file};

#[test]
fn warns_on_inline_public_module_in_real_library_lib_rs() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let package_rel = "packages/shared-types/src/lib.rs";
    let package_content =
        std::fs::read_to_string(root.join(package_rel)).expect("read package source");

    write_file(
        root,
        package_rel,
        &format!("{package_content}\n\npub mod api {{ pub fn ping() {{}} }}\n"),
    );

    let results = run_family(root);

    assert_eq!(
        files_for_rule(&results, "RS-CODE-28"),
        BTreeSet::from([package_rel.to_owned()])
    );
}
