use guardrail3_app_rs_family_cargo_assertions::rs_cargo_06_no_weakened_overrides::{
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
    unreachable_pub = "deny"
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
fn weakened_member_override_is_error() {
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

                    [lints.rust]
                    warnings = "allow"
                "#,
            ),
        ],
    ));

    guardrail3_app_rs_family_cargo_assertions::rs_cargo_06_no_weakened_overrides::assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            file: None,
            title: Some("weakened member rust override"),
            inventory: None,
        }],
    );
}

#[test]
fn matching_member_policy_inventories_cleanly() {
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

    guardrail3_app_rs_family_cargo_assertions::rs_cargo_06_no_weakened_overrides::assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            file: None,
            title: None,
            inventory: Some(true),
        }],
    );
}

#[test]
fn forbid_to_deny_member_override_is_still_a_weakening() {
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

                    [lints.rust]
                    unsafe_code = "deny"
                "#,
            ),
        ],
    ));

    guardrail3_app_rs_family_cargo_assertions::rs_cargo_06_no_weakened_overrides::assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            file: None,
            title: Some("weakened member rust override"),
            inventory: None,
        }],
    );
}

#[test]
fn standalone_package_root_emits_no_member_results() {
    let manifest = r#"
        [package]
        name = "helper"
        edition = "2024"
    "#;

    let results = check_results(&tree(
        &[("pkg", entry(&[], &["Cargo.toml"]))],
        &[("pkg/Cargo.toml", manifest)],
    ));

    guardrail3_app_rs_family_cargo_assertions::rs_cargo_06_no_weakened_overrides::assert_rule_results(
        &results,
        &[],
    );
}

#[test]
fn malformed_workspace_member_manifest_is_owned_by_input_failures_rule() {
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
            ("crates/api/Cargo.toml", "[package"),
        ],
    ));

    guardrail3_app_rs_family_cargo_assertions::rs_cargo_06_no_weakened_overrides::assert_rule_results(
        &results,
        &[],
    );
}

#[test]
fn missing_workspace_lint_tables_do_not_emit_false_inventory() {
    let workspace_manifest = r#"
        [workspace]
        members = ["crates/api"]
        resolver = "2"

        [workspace.package]
        edition = "2024"
        rust-version = "1.85"
    "#;

    let results = check_results(&tree(
        &[
            ("", entry(&["crates"], &["Cargo.toml"])),
            ("crates", entry(&["api"], &[])),
            ("crates/api", entry(&[], &["Cargo.toml"])),
        ],
        &[
            ("Cargo.toml", workspace_manifest),
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

    guardrail3_app_rs_family_cargo_assertions::rs_cargo_06_no_weakened_overrides::assert_rule_results(
        &results,
        &[],
    );
}

#[test]
fn invalid_member_lint_table_shape_is_error() {
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
                    rust = "deny"
                "#,
            ),
        ],
    ));

    guardrail3_app_rs_family_cargo_assertions::rs_cargo_06_no_weakened_overrides::assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            file: None,
            title: Some("invalid member rust lint table"),
            inventory: None,
        }],
    );
}

#[test]
fn invalid_member_override_level_is_error() {
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

                    [lints.rust]
                    warnings = 7
                "#,
            ),
        ],
    ));

    guardrail3_app_rs_family_cargo_assertions::rs_cargo_06_no_weakened_overrides::assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            file: None,
            title: Some("invalid member rust override"),
            inventory: None,
        }],
    );
}

#[test]
fn invalid_member_clippy_override_level_is_error() {
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

                    [lints.clippy]
                    unwrap_used = "banana"
                "#,
            ),
        ],
    ));

    guardrail3_app_rs_family_cargo_assertions::rs_cargo_06_no_weakened_overrides::assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            file: None,
            title: Some("invalid member clippy override"),
            inventory: None,
        }],
    );
}
