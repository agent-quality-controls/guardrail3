use std::collections::BTreeSet;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family, write_file};

#[test]
fn inventories_large_traits_in_real_library_profile_files() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let package_rel = "packages/shared-types/src/lib.rs";
    let package_content =
        std::fs::read_to_string(root.join(package_rel)).expect("read package source");

    let mut warn_methods = String::new();
    for index in 0..9 {
        warn_methods.push_str(&format!("    fn warn_{index}(&self);\n"));
    }
    let mut error_methods = String::new();
    for index in 0..13 {
        error_methods.push_str(&format!("    fn error_{index}(&self);\n"));
    }

    write_file(
        root,
        package_rel,
        &format!(
            "{package_content}\n\npub trait SharedSurface {{\n{warn_methods}}}\n\npub trait OversizedSurface {{\n{error_methods}}}\n"
        ),
    );

    let results = run_family(root);

    assert_eq!(
        files_for_rule(&results, "RS-CODE-29"),
        BTreeSet::from([package_rel.to_owned()])
    );
}
