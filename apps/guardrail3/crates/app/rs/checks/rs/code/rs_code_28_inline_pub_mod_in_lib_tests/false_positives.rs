use std::collections::BTreeSet;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family, write_file};

#[test]
fn skips_non_public_or_file_backed_modules() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let package_rel = "packages/shared-types/src/lib.rs";
    let package_content =
        std::fs::read_to_string(root.join(package_rel)).expect("read package source");

    write_file(
        root,
        package_rel,
        &format!("{package_content}\n\nmod internal {{ pub fn ping() {{}} }}\npub mod api;\n"),
    );

    let results = run_family(root);
    let rs_code_28_results = results
        .iter()
        .filter(|result| result.id == "RS-CODE-28")
        .collect::<Vec<_>>();

    assert_eq!(files_for_rule(&results, "RS-CODE-28"), BTreeSet::new());
    assert!(rs_code_28_results.is_empty());
}
