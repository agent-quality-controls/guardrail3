use std::collections::BTreeSet;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family, write_file};

#[test]
fn ignores_exception_like_text_outside_supported_config_comment_forms() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let root_package_rel = "package.json";
    let backend_cargo_rel = "apps/backend/Cargo.toml";
    let root_guardrail_rel = "guardrail3.toml";

    let root_package = std::fs::read_to_string(root.join(root_package_rel)).expect("read package");
    let backend_cargo =
        std::fs::read_to_string(root.join(backend_cargo_rel)).expect("read backend cargo");
    let root_guardrail =
        std::fs::read_to_string(root.join(root_guardrail_rel)).expect("read root guardrail");

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
    write_file(
        root,
        root_guardrail_rel,
        &format!("{root_guardrail}\nnote = \"# EXCEPTION: literal text only\"\n"),
    );

    let results = run_family(root);
    let rs_code_07_results = results
        .iter()
        .filter(|result| result.id == "RS-CODE-07")
        .collect::<Vec<_>>();

    assert_eq!(files_for_rule(&results, "RS-CODE-07"), BTreeSet::new());
    assert!(rs_code_07_results.is_empty());
}
