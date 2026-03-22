use std::collections::BTreeMap;
use std::path::PathBuf;

use crate::domain::project_tree::{DirEntry, ProjectTree};

use super::check;
use super::discover::collect;
use super::inputs::WorkspaceMembersSetInput;

#[test]
fn workspace_metadata_and_resolver_are_checked() {
    let tree = ProjectTree {
        root: PathBuf::from("/tmp/project"),
        structure: BTreeMap::from([(
            "".to_owned(),
            DirEntry {
                dirs: vec!["crates".to_owned()],
                files: vec!["Cargo.toml".to_owned()],
            },
        )]),
        content: BTreeMap::from([(
            "Cargo.toml".to_owned(),
            r#"
                [workspace]
                members = ["crates/*"]
                resolver = "2"

                [workspace.package]
                edition = "2024"
                rust-version = "1.85"

                [workspace.lints.rust]
                warnings = "deny"
                unsafe_code = "forbid"
                dead_code = "deny"
                unused_results = "deny"
                unused_crate_dependencies = "deny"
                missing_debug_implementations = "warn"

                [workspace.lints.clippy]
                all = { level = "deny", priority = -1 }
                pedantic = { level = "deny", priority = -1 }
                cargo = { level = "deny", priority = -1 }
                nursery = { level = "deny", priority = -1 }
                unwrap_used = "deny"
                expect_used = "deny"
                panic = "deny"
                unimplemented = "deny"
                todo = "deny"
                dbg_macro = "deny"
                print_stdout = "deny"
                print_stderr = "deny"
                disallowed_methods = "deny"
                disallowed_types = "deny"
                indexing_slicing = "deny"
                string_slice = "deny"
                arithmetic_side_effects = "deny"
                shadow_unrelated = "deny"
                missing_assert_message = "deny"
                partial_pub_fields = "deny"
                str_to_string = "deny"
                implicit_clone = "deny"
                as_conversions = "deny"
                float_cmp = "deny"
                lossy_float_literal = "deny"
                wildcard_enum_match_arm = "deny"
                rest_pat_in_fully_bound_structs = "deny"
                large_stack_arrays = "deny"
                needless_pass_by_value = "deny"
                redundant_else = "deny"
                large_futures = "deny"
                semicolon_if_nothing_returned = "deny"
                redundant_closure_for_method_calls = "deny"
                map_unwrap_or = "deny"
                verbose_file_reads = "deny"
                missing_docs_in_private_items = "allow"
                module_name_repetitions = "allow"
                must_use_candidate = "allow"
                option_if_let_else = "allow"
                empty_line_after_doc_comments = "allow"
                single_match_else = "allow"
                ref_option_ref = "allow"
                trivially_copy_pass_by_ref = "allow"
                multiple_crate_versions = "allow"
            "#
            .to_owned(),
        )]),
    };

    let results = check(&tree);
    assert!(results.iter().any(|r| r.id == "RS-CARGO-01" && r.inventory));
    assert!(results.iter().any(|r| r.id == "RS-CARGO-02" && r.inventory));
    assert!(results.iter().any(|r| r.id == "RS-CARGO-03" && r.inventory));
    assert!(results.iter().any(|r| r.id == "RS-CARGO-05" && r.inventory));
    assert!(results.iter().any(|r| r.id == "RS-CARGO-07" && r.inventory));
    assert!(results.iter().any(|r| r.id == "RS-CARGO-08" && r.inventory));
}

#[test]
fn virtual_workspace_missing_resolver_is_reported() {
    let tree = ProjectTree {
        root: PathBuf::from("/tmp/project"),
        structure: BTreeMap::from([(
            "".to_owned(),
            DirEntry {
                dirs: vec!["crates".to_owned()],
                files: vec!["Cargo.toml".to_owned()],
            },
        )]),
        content: BTreeMap::from([(
            "Cargo.toml".to_owned(),
            r#"
                [workspace]
                members = ["crates/*"]

                [workspace.package]
                edition = "2024"
            "#
            .to_owned(),
        )]),
    };

    let results = check(&tree);
    assert!(
        results
            .iter()
            .any(|r| r.id == "RS-CARGO-08" && matches!(r.severity, crate::domain::report::Severity::Error))
    );
}

#[test]
fn declared_members_are_paired_and_membership_sets_are_bound() {
    let tree = ProjectTree {
        root: PathBuf::from("/tmp/project"),
        structure: BTreeMap::from([
            (
                "".to_owned(),
                DirEntry {
                    dirs: vec!["crates".to_owned()],
                    files: vec!["Cargo.toml".to_owned()],
                },
            ),
            (
                "crates".to_owned(),
                DirEntry {
                    dirs: vec!["api".to_owned(), "domain".to_owned()],
                    files: vec![],
                },
            ),
            (
                "crates/api".to_owned(),
                DirEntry {
                    dirs: vec![],
                    files: vec!["Cargo.toml".to_owned()],
                },
            ),
            (
                "crates/domain".to_owned(),
                DirEntry {
                    dirs: vec![],
                    files: vec!["Cargo.toml".to_owned()],
                },
            ),
        ]),
        content: BTreeMap::from([
            (
                "Cargo.toml".to_owned(),
                r#"
                    [workspace]
                    members = ["crates/*"]
                    resolver = "2"

                    [workspace.package]
                    edition = "2024"
                "#
                .to_owned(),
            ),
            (
                "crates/api/Cargo.toml".to_owned(),
                r#"
                    [package]
                    name = "api"
                    edition = "2024"

                    [lints]
                    workspace = true
                "#
                .to_owned(),
            ),
            (
                "crates/domain/Cargo.toml".to_owned(),
                r#"
                    [package]
                    name = "domain"
                    edition = "2024"
                "#
                .to_owned(),
            ),
        ]),
    };

    let facts = collect(&tree).expect("workspace facts");
    let set_input = WorkspaceMembersSetInput::from_facts(&facts);
    assert_eq!(set_input.workspace.rel_path, "Cargo.toml");
    assert!(set_input.declared_members.contains("crates/api"));
    assert!(set_input.discovered_members.contains("crates/domain"));

    let results = check(&tree);
    assert!(
        results
            .iter()
            .any(|r| r.id == "RS-CARGO-04" && r.inventory && r.file.as_deref() == Some("crates/api/Cargo.toml"))
    );
    assert!(
        results
            .iter()
            .any(|r| r.id == "RS-CARGO-04" && !r.inventory && r.file.as_deref() == Some("crates/domain/Cargo.toml"))
    );
}

#[test]
fn older_member_edition_is_warned() {
    let tree = ProjectTree {
        root: PathBuf::from("/tmp/project"),
        structure: BTreeMap::from([
            (
                "".to_owned(),
                DirEntry {
                    dirs: vec!["crates".to_owned()],
                    files: vec!["Cargo.toml".to_owned()],
                },
            ),
            (
                "crates".to_owned(),
                DirEntry {
                    dirs: vec!["legacy".to_owned()],
                    files: vec![],
                },
            ),
            (
                "crates/legacy".to_owned(),
                DirEntry {
                    dirs: vec![],
                    files: vec!["Cargo.toml".to_owned()],
                },
            ),
        ]),
        content: BTreeMap::from([
            (
                "Cargo.toml".to_owned(),
                r#"
                    [workspace]
                    members = ["crates/*"]
                    resolver = "2"

                    [workspace.package]
                    edition = "2024"
                "#
                .to_owned(),
            ),
            (
                "crates/legacy/Cargo.toml".to_owned(),
                r#"
                    [package]
                    name = "legacy"
                    edition = "2021"

                    [lints]
                    workspace = true
                "#
                .to_owned(),
            ),
        ]),
    };

    let results = check(&tree);
    assert!(
        results
            .iter()
            .any(|r| r.id == "RS-CARGO-09" && matches!(r.severity, crate::domain::report::Severity::Warn))
    );
}

#[test]
fn weakened_member_override_is_reported() {
    let tree = ProjectTree {
        root: PathBuf::from("/tmp/project"),
        structure: BTreeMap::from([
            (
                "".to_owned(),
                DirEntry {
                    dirs: vec!["crates".to_owned()],
                    files: vec!["Cargo.toml".to_owned()],
                },
            ),
            (
                "crates".to_owned(),
                DirEntry {
                    dirs: vec!["api".to_owned()],
                    files: vec![],
                },
            ),
            (
                "crates/api".to_owned(),
                DirEntry {
                    dirs: vec![],
                    files: vec!["Cargo.toml".to_owned()],
                },
            ),
        ]),
        content: BTreeMap::from([
            (
                "Cargo.toml".to_owned(),
                r#"
                    [workspace]
                    members = ["crates/*"]
                    resolver = "2"

                    [workspace.package]
                    edition = "2024"

                    [workspace.lints.rust]
                    warnings = "deny"
                "#
                .to_owned(),
            ),
            (
                "crates/api/Cargo.toml".to_owned(),
                r#"
                    [package]
                    name = "api"
                    edition = "2024"

                    [lints]
                    workspace = true

                    [lints.rust]
                    warnings = "allow"
                "#
                .to_owned(),
            ),
        ]),
    };

    let results = check(&tree);
    assert!(
        results
            .iter()
            .any(|r| r.id == "RS-CARGO-06" && matches!(r.severity, crate::domain::report::Severity::Error))
    );
}

#[test]
fn negative_specific_lint_priority_is_warned() {
    let tree = ProjectTree {
        root: PathBuf::from("/tmp/project"),
        structure: BTreeMap::from([(
            "".to_owned(),
            DirEntry {
                dirs: vec![],
                files: vec!["Cargo.toml".to_owned()],
            },
        )]),
        content: BTreeMap::from([(
            "Cargo.toml".to_owned(),
            r#"
                [workspace]
                members = []
                resolver = "2"

                [workspace.package]
                edition = "2024"

                [workspace.lints.rust]
                warnings = "deny"
                unsafe_code = "forbid"
                dead_code = "deny"
                unused_results = "deny"
                unused_crate_dependencies = "deny"
                missing_debug_implementations = "warn"

                [workspace.lints.clippy]
                all = { level = "deny", priority = -1 }
                pedantic = { level = "deny", priority = -1 }
                cargo = { level = "deny", priority = -1 }
                nursery = { level = "deny", priority = -1 }
                unwrap_used = { level = "deny", priority = -2 }
                expect_used = "deny"
                panic = "deny"
                unimplemented = "deny"
                todo = "deny"
                dbg_macro = "deny"
                print_stdout = "deny"
                print_stderr = "deny"
                disallowed_methods = "deny"
                disallowed_types = "deny"
                indexing_slicing = "deny"
                string_slice = "deny"
                arithmetic_side_effects = "deny"
                shadow_unrelated = "deny"
                missing_assert_message = "deny"
                partial_pub_fields = "deny"
                str_to_string = "deny"
                implicit_clone = "deny"
                as_conversions = "deny"
                float_cmp = "deny"
                lossy_float_literal = "deny"
                wildcard_enum_match_arm = "deny"
                rest_pat_in_fully_bound_structs = "deny"
                large_stack_arrays = "deny"
                needless_pass_by_value = "deny"
                redundant_else = "deny"
                large_futures = "deny"
                semicolon_if_nothing_returned = "deny"
                redundant_closure_for_method_calls = "deny"
                map_unwrap_or = "deny"
                verbose_file_reads = "deny"
            "#
            .to_owned(),
        )]),
    };

    let results = check(&tree);
    assert!(
        results
            .iter()
            .any(|r| r.id == "RS-CARGO-07" && matches!(r.severity, crate::domain::report::Severity::Warn))
    );
}
