use guardrail3_app_rs_family_cargo_assertions::rs_cargo_10_missing_member_cargo::{
    ExpectedRuleResult, check_results,
};
use test_support::{entry, tree};

const WORKSPACE_RUST_LINTS: &str = r#"
    [workspace.lints.rust]
    warnings = "deny"
    unsafe_code = "forbid"
    dead_code = "deny"
    unused_results = "deny"
    unused_crate_dependencies = "deny"
    missing_debug_implementations = "warn"
"#;

const WORKSPACE_CLIPPY_LINTS: &str = r#"
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
    disallowed_macros = "deny"
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
"#;

#[test]
fn declared_member_without_manifest_warns() {
    let workspace_manifest = format!(
        r#"
            [workspace]
            members = ["crates/api", "crates/missing"]
            resolver = "2"

            [workspace.package]
            edition = "2024"
            rust-version = "1.85"

            {WORKSPACE_RUST_LINTS}
            {WORKSPACE_CLIPPY_LINTS}
        "#
    );
    let results = check_results(&tree(
        &[
            ("", entry(&["crates"], &["Cargo.toml"])),
            ("crates", entry(&["api", "missing"], &[])),
            ("crates/api", entry(&[], &["Cargo.toml"])),
            ("crates/missing", entry(&[], &[])),
        ],
        &[
            ("Cargo.toml", &workspace_manifest),
            (
                "crates/api/Cargo.toml",
                r#"
                    [package]
                    name = "api"
                    edition = "2024"

                    [lints]
                    workspace = true
                "#,
            ),
        ],
    ));

    guardrail3_app_rs_family_cargo_assertions::rs_cargo_10_missing_member_cargo::assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            file: None,
            title: Some("declared workspace member missing Cargo.toml"),
            inventory: None,
        }],
    );
    assert!(results.iter().any(|result| {
        result.id() == "RS-CARGO-10"
            && result.title() == "declared workspace member missing Cargo.toml"
            && result.severity() == guardrail3_domain_report::Severity::Warn
    }));
}

#[test]
fn complete_member_set_is_inventory() {
    let workspace_manifest = format!(
        r#"
            [workspace]
            members = ["crates/api"]
            resolver = "2"

            [workspace.package]
            edition = "2024"
            rust-version = "1.85"

            {WORKSPACE_RUST_LINTS}
            {WORKSPACE_CLIPPY_LINTS}
        "#
    );
    let results = check_results(&tree(
        &[
            ("", entry(&["crates"], &["Cargo.toml"])),
            ("crates", entry(&["api"], &[])),
            ("crates/api", entry(&[], &["Cargo.toml"])),
        ],
        &[
            ("Cargo.toml", &workspace_manifest),
            (
                "crates/api/Cargo.toml",
                r#"
                    [package]
                    name = "api"
                    edition = "2024"

                    [lints]
                    workspace = true
                "#,
            ),
        ],
    ));

    guardrail3_app_rs_family_cargo_assertions::rs_cargo_10_missing_member_cargo::assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            file: None,
            title: Some("all declared workspace members have Cargo.toml"),
            inventory: Some(true),
        }],
    );
}

#[test]
fn scoped_workspace_run_does_not_warn_for_unrouted_sibling_member_manifest() {
    let workspace_manifest = format!(
        r#"
            [workspace]
            members = ["crates/api", "crates/other"]
            resolver = "2"

            [workspace.package]
            edition = "2024"
            rust-version = "1.85"

            {WORKSPACE_RUST_LINTS}
            {WORKSPACE_CLIPPY_LINTS}
        "#
    );
    let results = crate::check_test_tree_with_validation_scope(
        &tree(
            &[
                ("", entry(&["crates"], &["Cargo.toml"])),
                ("crates", entry(&["api", "other"], &[])),
                ("crates/api", entry(&["src"], &["Cargo.toml"])),
                ("crates/api/src", entry(&[], &["lib.rs"])),
                ("crates/other", entry(&[], &["Cargo.toml"])),
            ],
            &[
                ("Cargo.toml", &workspace_manifest),
                (
                    "crates/api/Cargo.toml",
                    r#"
                        [package]
                        name = "api"
                        edition = "2024"

                        [lints]
                        workspace = true
                    "#,
                ),
                (
                    "crates/other/Cargo.toml",
                    r#"
                        [package]
                        name = "other"
                        edition = "2024"

                        [lints]
                        workspace = true
                    "#,
                ),
            ],
        ),
        "crates/api/src",
    );

    guardrail3_app_rs_family_cargo_assertions::rs_cargo_10_missing_member_cargo::assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            file: Some("Cargo.toml"),
            title: Some("all declared workspace members have Cargo.toml"),
            inventory: Some(true),
        }],
    );
}

#[test]
fn malformed_workspace_members_shape_does_not_emit_clean_inventory() {
    let workspace_manifest = format!(
        r#"
            [workspace]
            members = "crates/api"
            resolver = "2"

            [workspace.package]
            edition = "2024"
            rust-version = "1.85"

            {WORKSPACE_RUST_LINTS}
            {WORKSPACE_CLIPPY_LINTS}
        "#
    );

    let results = check_results(&tree(
        &[
            ("", entry(&["crates"], &["Cargo.toml"])),
            ("crates", entry(&["api"], &[])),
            ("crates/api", entry(&[], &["Cargo.toml"])),
        ],
        &[
            ("Cargo.toml", &workspace_manifest),
            (
                "crates/api/Cargo.toml",
                r#"
                    [package]
                    name = "api"
                    edition = "2024"

                    [lints]
                    workspace = true
                "#,
            ),
        ],
    ));

    guardrail3_app_rs_family_cargo_assertions::rs_cargo_10_missing_member_cargo::assert_rule_results(
        &results,
        &[],
    );
}
