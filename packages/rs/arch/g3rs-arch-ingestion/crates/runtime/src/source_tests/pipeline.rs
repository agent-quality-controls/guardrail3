use g3rs_arch_ingestion_assertions::source as assertions;
use guardrail3_check_types::G3Severity;

use super::helpers::{make_dir, source_inputs, source_results, temp_workspace_root, write_file};

#[test]
fn source_pipeline_reports_lib_body_logic_and_path_attr() {
    let root = temp_workspace_root();

    write_file(
        &root,
        "Cargo.toml",
        "[workspace]\nmembers = [\"crate_a\"]\n",
    );
    make_dir(&root, "crate_a/src/nested");
    write_file(
        &root,
        "crate_a/Cargo.toml",
        "[package]\nname = \"crate_a\"\nversion = \"0.1.0\"\n\n[features]\nall = [\"api\"]\ndefault = [\"all\"]\napi = []\n",
    );
    write_file(
        &root,
        "crate_a/src/lib.rs",
        "pub fn leaked() {}\n\n#[path = \"nested/custom.rs\"]\npub mod nested;\n",
    );
    write_file(&root, "crate_a/src/nested/custom.rs", "pub struct Value;");

    let results = source_results(&root);
    assertions::assert_has_result(
        &results,
        "RS-ARCH-SOURCE-02",
        G3Severity::Error,
        Some("crate_a/src/lib.rs"),
    );
    assertions::assert_has_result(
        &results,
        "RS-ARCH-SOURCE-09",
        G3Severity::Error,
        Some("crate_a/src/lib.rs"),
    );
}

#[test]
fn source_pipeline_reports_only_source_half_of_feature_gating_rule() {
    let root = temp_workspace_root();

    write_file(
        &root,
        "Cargo.toml",
        "[workspace]\nmembers = [\"crate_a\"]\n",
    );
    make_dir(&root, "crate_a/src");
    write_file(
        &root,
        "crate_a/Cargo.toml",
        "[package]\nname = \"crate_a\"\nversion = \"0.1.0\"\n",
    );
    write_file(&root, "crate_a/src/lib.rs", "pub mod api;\n");

    let results = source_results(&root);
    assertions::assert_has_result(
        &results,
        "RS-ARCH-SOURCE-08",
        G3Severity::Error,
        Some("crate_a/src/lib.rs"),
    );
    assertions::assert_missing_result(&results, "RS-ARCH-CONFIG-08");
}

#[test]
fn source_pipeline_allows_mod_dispatcher_with_restricted_use() {
    let root = temp_workspace_root();

    write_file(
        &root,
        "Cargo.toml",
        "[workspace]\nmembers = [\"crate_a\"]\n",
    );
    make_dir(&root, "crate_a/src/api");
    write_file(
        &root,
        "crate_a/Cargo.toml",
        "[package]\nname = \"crate_a\"\nversion = \"0.1.0\"\n",
    );
    write_file(&root, "crate_a/src/lib.rs", "mod api;\n");
    write_file(
        &root,
        "crate_a/src/api/mod.rs",
        "mod rule;\n\npub(crate) use rule::check;\n",
    );
    write_file(
        &root,
        "crate_a/src/api/rule.rs",
        "pub(crate) fn check() {}\n",
    );

    let results = source_results(&root);
    assertions::assert_missing_result_with_severity(
        &results,
        "RS-ARCH-SOURCE-04",
        G3Severity::Error,
    );
    assertions::assert_has_result(
        &results,
        "RS-ARCH-SOURCE-04",
        G3Severity::Info,
        Some("crate_a/src/api/mod.rs"),
    );
}

#[test]
fn source_ingestion_stays_inside_the_pointed_workspace() {
    let root = temp_workspace_root();

    write_file(
        &root,
        "Cargo.toml",
        "[workspace]\nmembers = [\"crate_a\"]\n",
    );
    make_dir(&root, "crate_a/src");
    make_dir(&root, "foreign/src");
    write_file(
        &root,
        "crate_a/Cargo.toml",
        "[package]\nname = \"crate_a\"\nversion = \"0.1.0\"\n",
    );
    write_file(&root, "crate_a/src/lib.rs", "pub fn leaked() {}\n");
    write_file(
        &root,
        "foreign/Cargo.toml",
        "[package]\nname = \"foreign\"\nversion = \"0.1.0\"\n",
    );
    write_file(&root, "foreign/src/lib.rs", "pub fn stray() {}\n");

    let inputs = source_inputs(&root);

    assert_eq!(inputs.len(), 1);
    assert_eq!(inputs[0].lib_facade_checks.len(), 1);
    assert_eq!(inputs[0].lib_facade_checks[0].krate.rel_dir, "crate_a");
    assert!(
        inputs[0]
            .path_attr_sites
            .iter()
            .all(|site| site.rel_path.starts_with("crate_a/"))
    );
}

#[test]
fn source_ingestion_does_not_recurse_into_excluded_nested_crates() {
    let root = temp_workspace_root();

    write_file(
        &root,
        "Cargo.toml",
        "[workspace]\nmembers = [\"pkg\", \"pkg/crates/inner\"]\nexclude = [\"pkg/crates/inner\"]\n",
    );
    make_dir(&root, "pkg/src");
    make_dir(&root, "pkg/crates/inner/src");
    write_file(
        &root,
        "pkg/Cargo.toml",
        "[package]\nname = \"pkg\"\nversion = \"0.1.0\"\n",
    );
    write_file(&root, "pkg/src/lib.rs", "pub mod api;\n");
    write_file(
        &root,
        "pkg/crates/inner/Cargo.toml",
        "[package]\nname = \"inner\"\nversion = \"0.1.0\"\n",
    );
    write_file(
        &root,
        "pkg/crates/inner/src/lib.rs",
        "pub fn leaked_from_excluded_child() {}\n",
    );

    let inputs = source_inputs(&root);

    assert_eq!(inputs.len(), 1);
    assert_eq!(inputs[0].lib_facade_checks.len(), 1);
    assert_eq!(inputs[0].lib_facade_checks[0].krate.rel_dir, "pkg");
    assert!(
        inputs[0]
            .path_attr_sites
            .iter()
            .all(|site| !site.rel_path.starts_with("pkg/crates/inner/"))
    );
    assert!(
        inputs[0]
            .mod_facade_surfaces
            .iter()
            .all(|surface| !surface.rel_path.starts_with("pkg/crates/inner/"))
    );
}
