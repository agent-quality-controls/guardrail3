mod helpers;
use guardrail3_app_rs_family_toolchain_assertions::rs_toolchain_01_exists::{
    ExpectedRuleResult, Severity, assert_invalid_root_cargo_rust_version_type,
    assert_legacy_only_family_results, assert_malformed_modern_and_legacy_results,
    assert_rule_results,
};

use helpers::{nested_workspace_root_tree, run_family_check, test_input, test_tree};
use super::check;
fn workspace_tree_with_nested_non_member_package() -> guardrail3_app_rs_family_view::FamilyView {
    use std::collections::BTreeMap;
    use std::path::PathBuf;

    use guardrail3_app_rs_family_view::{DirEntry, FamilyView as ProjectTree};

    let structure = BTreeMap::from([
        (
            String::new(),
            DirEntry::new(
                vec!["nested".to_owned()],
                vec!["Cargo.toml".to_owned(), "rust-toolchain.toml".to_owned()],
                Vec::new(),
                Vec::new(),
            ),
        ),
        (
            "nested".to_owned(),
            DirEntry::new(
                Vec::new(),
                vec!["Cargo.toml".to_owned()],
                Vec::new(),
                Vec::new(),
            ),
        ),
    ]);
    let content = BTreeMap::from([
        (
            "Cargo.toml".to_owned(),
            "[workspace]\nmembers = []\n".to_owned(),
        ),
        (
            "rust-toolchain.toml".to_owned(),
            "[toolchain]\nchannel = \"stable\"\ncomponents = [\"clippy\", \"rustfmt\"]".to_owned(),
        ),
        (
            "nested/Cargo.toml".to_owned(),
            "[package]\nname = \"nested\"\nversion = \"0.1.0\"\nedition = \"2024\"\n".to_owned(),
        ),
    ]);

    ProjectTree::build(
        PathBuf::from("/tmp/toolchain-nested-non-member-package"),
        &structure,
        &content,
        &["".to_owned()],
        &[],
        &[],
        None,
        &[],
    )
}

fn workspace_tree_with_non_package_descendant_toolchain()
-> guardrail3_app_rs_family_view::FamilyView {
    use std::collections::BTreeMap;
    use std::path::PathBuf;

    use guardrail3_app_rs_family_view::{DirEntry, FamilyView as ProjectTree};

    let structure = BTreeMap::from([
        (
            String::new(),
            DirEntry::new(
                vec!["docs".to_owned()],
                vec!["Cargo.toml".to_owned(), "rust-toolchain.toml".to_owned()],
                Vec::new(),
                Vec::new(),
            ),
        ),
        (
            "docs".to_owned(),
            DirEntry::new(
                Vec::new(),
                vec!["rust-toolchain.toml".to_owned()],
                Vec::new(),
                Vec::new(),
            ),
        ),
    ]);
    let content = BTreeMap::from([
        (
            "Cargo.toml".to_owned(),
            "[workspace]\nmembers = []\n".to_owned(),
        ),
        (
            "rust-toolchain.toml".to_owned(),
            "[toolchain]\nchannel = \"stable\"\ncomponents = [\"clippy\", \"rustfmt\"]".to_owned(),
        ),
        (
            "docs/rust-toolchain.toml".to_owned(),
            "[toolchain]\nchannel = \"beta\"\ncomponents = [\"clippy\", \"rustfmt\"]".to_owned(),
        ),
    ]);

    ProjectTree::build(
        PathBuf::from("/tmp/toolchain-non-package-descendant"),
        &structure,
        &content,
        &["".to_owned()],
        &[],
        &[],
        None,
        &[],
    )
}

#[test]
fn inventories_when_toolchain_toml_exists() {
    let input = test_input(
        Some("rust-toolchain.toml"),
        None,
        None,
        None,
        Some("1.85"),
        None,
    );
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Severity::Info,
            inventory: true,
            title: "rust-toolchain.toml exists",
            message: "Found rust-toolchain.toml at workspace root.",
            file: Some("rust-toolchain.toml"),
        }],
    );
}

#[test]
fn errors_when_no_supported_toolchain_file_exists() {
    let input = test_input(None, None, None, None, Some("1.85"), None);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Severity::Error,
            inventory: false,
            title: "rust-toolchain.toml missing",
            message: "Expected rust-toolchain.toml at workspace root. Create a `rust-toolchain.toml` with a `[toolchain]` section containing `channel` and `components`.",
            file: Some("rust-toolchain.toml"),
        }],
    );
}

#[test]
fn family_reports_legacy_only_as_missing_modern_toolchain() {
    let tree = test_tree(
        &["Cargo.toml", "rust-toolchain"],
        &[("Cargo.toml", "[workspace]\n")],
    );

    let results = run_family_check(&tree);

    assert_legacy_only_family_results(&results);
}

#[test]
fn family_reports_malformed_modern_toolchain_and_legacy_ambiguity() {
    let tree = test_tree(
        &["Cargo.toml", "rust-toolchain.toml", "rust-toolchain"],
        &[
            ("Cargo.toml", "[workspace]\n"),
            ("rust-toolchain.toml", "toolchain = ["),
        ],
    );

    let results = run_family_check(&tree);

    assert_malformed_modern_and_legacy_results(&results);
}

#[test]
fn family_propagates_invalid_root_cargo_rust_version_type() {
    let tree = test_tree(
        &["rust-toolchain.toml", "Cargo.toml"],
        &[
            (
                "rust-toolchain.toml",
                "[toolchain]\nchannel = \"1.85.1\"\ncomponents = [\"clippy\", \"rustfmt\"]",
            ),
            (
                "Cargo.toml",
                "[workspace]\n[workspace.package]\nrust-version = 185\n",
            ),
        ],
    );

    let results = run_family_check(&tree);

    assert_invalid_root_cargo_rust_version_type(&results);
}

#[test]
fn family_targets_nested_workspace_root_instead_of_validation_root() {
    let tree = nested_workspace_root_tree();
    let results = run_family_check(&tree);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Severity::Error,
            inventory: false,
            title: "rust-toolchain.toml missing",
            message: "Expected rust-toolchain.toml at workspace root. Create a `rust-toolchain.toml` with a `[toolchain]` section containing `channel` and `components`.",
            file: Some("apps/guardrail3/rust-toolchain.toml"),
        }],
    );
}

#[test]
fn family_does_not_require_local_toolchain_for_non_member_package_inside_workspace_tree() {
    let tree = workspace_tree_with_nested_non_member_package();
    let results = run_family_check(&tree);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Severity::Info,
            inventory: true,
            title: "rust-toolchain.toml exists",
            message: "Found rust-toolchain.toml at workspace root.",
            file: Some("rust-toolchain.toml"),
        }],
    );
    assert!(
        !results
            .iter()
            .any(|result| result.file() == Some("nested/rust-toolchain.toml")),
        "nested non-member package should inherit workspace toolchain: {results:#?}"
    );
}

#[test]
fn family_ignores_descendant_toolchain_without_nested_cargo_root() {
    let tree = workspace_tree_with_non_package_descendant_toolchain();
    let results = run_family_check(&tree);

    assert!(
        !results
            .iter()
            .any(|result| result.file() == Some("docs/rust-toolchain.toml")),
        "descendant non-root toolchain placement is filtered by shared legality, not toolchain: {results:#?}"
    );
}
