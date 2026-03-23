use std::collections::BTreeSet;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family, write_file};

#[test]
fn ignores_exception_like_text_outside_supported_config_comment_forms() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let root_package_rel = "package.json";
    let backend_cargo_rel = "apps/backend/Cargo.toml";

    let root_package = std::fs::read_to_string(root.join(root_package_rel)).expect("read package");
    let backend_cargo =
        std::fs::read_to_string(root.join(backend_cargo_rel)).expect("read backend cargo");

    write_file(
        root,
        root_package_rel,
        &format!("{root_package}\n// EXCEPTION: package metadata note\n"),
    );
    write_file(
        root,
        backend_cargo_rel,
        &format!("{backend_cargo}\n# exception backend note without required marker\n"),
    );

    let results = run_family(root);

    assert_eq!(files_for_rule(&results, "RS-CODE-07"), BTreeSet::new());
}
