use std::collections::BTreeSet;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family, write_file};

#[test]
fn skips_consts_types_and_explicit_pub_reexports_in_lib_rs() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let package_rel = "packages/shared-types/src/lib.rs";
    let package_content =
        std::fs::read_to_string(root.join(package_rel)).expect("read package source");

    write_file(
        root,
        package_rel,
        &format!(
            "{package_content}\n\npub const API_VERSION: &str = \"v1\";\npub struct FacadeMarker;\npub mod internal {{ pub struct Visible; }}\npub use internal::Visible;\n"
        ),
    );

    let results = run_family(root);
    let rs_code_27_results = results
        .iter()
        .filter(|result| result.id == "RS-CODE-27")
        .collect::<Vec<_>>();

    assert_eq!(files_for_rule(&results, "RS-CODE-27"), BTreeSet::new());
    assert!(rs_code_27_results.is_empty());
}
