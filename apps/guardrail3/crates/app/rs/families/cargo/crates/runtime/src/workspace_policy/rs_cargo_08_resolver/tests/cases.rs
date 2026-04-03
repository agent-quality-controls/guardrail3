use guardrail3_app_rs_family_cargo_assertions::rs_cargo_08_resolver::{
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

const STANDALONE_RUST_LINTS: &str = r#"
    [lints.rust]
    warnings = "deny"
    unsafe_code = "forbid"
    dead_code = "deny"
    unused_results = "deny"
    unused_crate_dependencies = "deny"
    missing_debug_implementations = "warn"
    unreachable_pub = "deny"
"#;

const STANDALONE_CLIPPY_LINTS: &str = r#"
    [lints.clippy]
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
fn explicit_workspace_resolver_is_inventory() {
    let workspace_manifest = format!(
        r#"
            [workspace]
            members = []
            resolver = "2"

            [workspace.package]
            edition = "2024"
            rust-version = "1.85"

            {WORKSPACE_RUST_LINTS}
            {WORKSPACE_CLIPPY_LINTS}
        "#
    );
    let results = check_results(&tree(
        &[("", entry(&[], &["Cargo.toml"]))],
        &[("Cargo.toml", &workspace_manifest)],
    ));

    guardrail3_app_rs_family_cargo_assertions::rs_cargo_08_resolver::assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            file: None,
            title: None,
            inventory: Some(true),
        }],
    );
}

#[test]
fn missing_workspace_resolver_is_error() {
    let workspace_manifest = format!(
        r#"
            [workspace]
            members = []

            [workspace.package]
            edition = "2024"
            rust-version = "1.85"

            {WORKSPACE_RUST_LINTS}
            {WORKSPACE_CLIPPY_LINTS}
        "#
    );
    let results = check_results(&tree(
        &[("", entry(&[], &["Cargo.toml"]))],
        &[("Cargo.toml", &workspace_manifest)],
    ));

    guardrail3_app_rs_family_cargo_assertions::rs_cargo_08_resolver::assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            file: None,
            title: Some("workspace resolver missing"),
            inventory: None,
        }],
    );
}

#[test]
fn unsupported_workspace_resolver_is_error() {
    let workspace_manifest = format!(
        r#"
            [workspace]
            members = []
            resolver = "1"

            [workspace.package]
            edition = "2024"
            rust-version = "1.85"

            {WORKSPACE_RUST_LINTS}
            {WORKSPACE_CLIPPY_LINTS}
        "#
    );
    let results = check_results(&tree(
        &[("", entry(&[], &["Cargo.toml"]))],
        &[("Cargo.toml", &workspace_manifest)],
    ));

    guardrail3_app_rs_family_cargo_assertions::rs_cargo_08_resolver::assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            file: None,
            title: Some("unsupported workspace resolver"),
            inventory: None,
        }],
    );
}

#[test]
fn invalid_workspace_resolver_type_is_error() {
    let workspace_manifest = format!(
        r#"
            [workspace]
            members = []
            resolver = 2

            [workspace.package]
            edition = "2024"
            rust-version = "1.85"

            {WORKSPACE_RUST_LINTS}
            {WORKSPACE_CLIPPY_LINTS}
        "#
    );
    let results = check_results(&tree(
        &[("", entry(&[], &["Cargo.toml"]))],
        &[("Cargo.toml", &workspace_manifest)],
    ));

    guardrail3_app_rs_family_cargo_assertions::rs_cargo_08_resolver::assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            file: None,
            title: Some("workspace resolver invalid"),
            inventory: None,
        }],
    );
}

#[test]
fn standalone_root_does_not_emit_workspace_only_rule() {
    let workspace_manifest = format!(
        r#"
            [workspace]
            members = []
            resolver = "2"

            [workspace.package]
            edition = "2024"
            rust-version = "1.85"

            {WORKSPACE_RUST_LINTS}
            {WORKSPACE_CLIPPY_LINTS}
        "#
    );
    let standalone_manifest = format!(
        r#"
            [package]
            name = "helper"
            edition = "2024"
            rust-version = "1.85"

            {STANDALONE_RUST_LINTS}
            {STANDALONE_CLIPPY_LINTS}
        "#
    );
    let results = check_results(&tree(
        &[
            ("", entry(&["tools"], &["Cargo.toml"])),
            ("tools", entry(&["helper"], &[])),
            ("tools/helper", entry(&[], &["Cargo.toml"])),
        ],
        &[
            ("Cargo.toml", &workspace_manifest),
            ("tools/helper/Cargo.toml", &standalone_manifest),
        ],
    ));

    guardrail3_app_rs_family_cargo_assertions::rs_cargo_08_resolver::assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            file: Some("Cargo.toml"),
            title: None,
            inventory: None,
        }],
    );
}

#[test]
fn malformed_root_local_guardrail_suppresses_clean_inventory() {
    let workspace_manifest = format!(
        r#"
            [workspace]
            members = []
            resolver = "2"

            [workspace.package]
            edition = "2024"
            rust-version = "1.85"

            {WORKSPACE_RUST_LINTS}
            {WORKSPACE_CLIPPY_LINTS}
        "#
    );
    let results = check_results(&tree(
        &[("", entry(&[], &["Cargo.toml", "guardrail3.toml"]))],
        &[
            ("Cargo.toml", &workspace_manifest),
            ("guardrail3.toml", "[profile"),
        ],
    ));

    guardrail3_app_rs_family_cargo_assertions::rs_cargo_08_resolver::assert_rule_results(
        &results,
        &[],
    );
}

#[test]
fn scoped_run_excludes_sibling_workspace_root_results() {
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
    let sibling_workspace_manifest = format!(
        r#"
            [workspace]
            members = ["crate"]
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
                ("", entry(&["crates", "tools"], &["Cargo.toml"])),
                ("crates", entry(&["api"], &[])),
                ("crates/api", entry(&["src"], &["Cargo.toml"])),
                ("crates/api/src", entry(&[], &["lib.rs"])),
                ("tools", entry(&["helper"], &[])),
                ("tools/helper", entry(&["crate"], &["Cargo.toml"])),
                ("tools/helper/crate", entry(&[], &["Cargo.toml"])),
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
                ("tools/helper/Cargo.toml", &sibling_workspace_manifest),
                (
                    "tools/helper/crate/Cargo.toml",
                    r#"
                        [package]
                        name = "helper-crate"
                        edition = "2024"

                        [lints]
                        workspace = true
                    "#,
                ),
            ],
        ),
        "crates/api/src",
    );

    guardrail3_app_rs_family_cargo_assertions::rs_cargo_08_resolver::assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            file: Some("Cargo.toml"),
            title: Some("workspace resolver set"),
            inventory: Some(true),
        }],
    );
}
