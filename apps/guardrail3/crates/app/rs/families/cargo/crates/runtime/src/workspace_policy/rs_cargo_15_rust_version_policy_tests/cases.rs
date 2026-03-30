use guardrail3_app_rs_family_cargo_assertions::rs_cargo_15_rust_version_policy::{
    ExpectedRuleResult, check_results,
};
use test_support::{entry, tree};

const STANDALONE_RUST_LINTS: &str = r#"
    [lints.rust]
    warnings = "deny"
    unsafe_code = "forbid"
    dead_code = "deny"
    unused_results = "deny"
    unused_crate_dependencies = "deny"
    missing_debug_implementations = "warn"
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
fn library_profile_missing_rust_version_is_error() {
    let manifest = format!(
        r#"
            [package]
            name = "helper"
            edition = "2024"

            {STANDALONE_RUST_LINTS}
            {STANDALONE_CLIPPY_LINTS}
        "#
    );
    let results = check_results(&tree(
        &[("pkg", entry(&[], &["Cargo.toml", "guardrail3.toml"]))],
        &[
            ("pkg/Cargo.toml", &manifest),
            ("pkg/guardrail3.toml", "[profile]\nname = \"library\"\n"),
        ],
    ));

    guardrail3_app_rs_family_cargo_assertions::rs_cargo_15_rust_version_policy::assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            file: None,
            title: Some("library rust-version missing"),
            inventory: None,
        }],
    );
}

#[test]
fn non_library_missing_rust_version_is_inventory_only() {
    let manifest = format!(
        r#"
            [package]
            name = "helper"
            edition = "2024"

            {STANDALONE_RUST_LINTS}
            {STANDALONE_CLIPPY_LINTS}
        "#
    );
    let results = check_results(&tree(
        &[("pkg", entry(&[], &["Cargo.toml"]))],
        &[("pkg/Cargo.toml", &manifest)],
    ));

    guardrail3_app_rs_family_cargo_assertions::rs_cargo_15_rust_version_policy::assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            file: None,
            title: None,
            inventory: Some(true),
        }],
    );
}

#[test]
fn library_profile_with_rust_version_is_inventory() {
    let manifest = format!(
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
        &[("pkg", entry(&[], &["Cargo.toml", "guardrail3.toml"]))],
        &[
            ("pkg/Cargo.toml", &manifest),
            ("pkg/guardrail3.toml", "[profile]\nname = \"library\"\n"),
        ],
    ));

    guardrail3_app_rs_family_cargo_assertions::rs_cargo_15_rust_version_policy::assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            file: None,
            title: None,
            inventory: Some(true),
        }],
    );
}

#[test]
fn malformed_root_local_guardrail_does_not_downgrade_missing_rust_version() {
    let manifest = format!(
        r#"
            [package]
            name = "helper"
            edition = "2024"

            {STANDALONE_RUST_LINTS}
            {STANDALONE_CLIPPY_LINTS}
        "#
    );

    let results = check_results(&tree(
        &[("pkg", entry(&[], &["Cargo.toml", "guardrail3.toml"]))],
        &[
            ("pkg/Cargo.toml", &manifest),
            ("pkg/guardrail3.toml", "[profile"),
        ],
    ));

    guardrail3_app_rs_family_cargo_assertions::rs_cargo_15_rust_version_policy::assert_rule_results(
        &results,
        &[],
    );
}

#[test]
fn workspace_root_library_profile_missing_rust_version_is_error() {
    let manifest = r#"
        [workspace]
        members = ["crates/api"]
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

    let results = check_results(&tree(
        &[
            ("", entry(&["crates"], &["Cargo.toml", "guardrail3.toml"])),
            ("crates", entry(&["api"], &[])),
            ("crates/api", entry(&[], &["Cargo.toml"])),
        ],
        &[
            ("Cargo.toml", manifest),
            ("guardrail3.toml", "[profile]\nname = \"library\"\n"),
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

    guardrail3_app_rs_family_cargo_assertions::rs_cargo_15_rust_version_policy::assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            file: None,
            title: Some("library rust-version missing"),
            inventory: None,
        }],
    );
}

#[test]
fn invalid_rust_version_type_is_error() {
    let manifest = format!(
        r#"
            [package]
            name = "helper"
            edition = "2024"
            rust-version = 1.85

            {STANDALONE_RUST_LINTS}
            {STANDALONE_CLIPPY_LINTS}
        "#
    );
    let results = check_results(&tree(
        &[("pkg", entry(&[], &["Cargo.toml", "guardrail3.toml"]))],
        &[
            ("pkg/Cargo.toml", &manifest),
            ("pkg/guardrail3.toml", "[profile]\nname = \"library\"\n"),
        ],
    ));

    guardrail3_app_rs_family_cargo_assertions::rs_cargo_15_rust_version_policy::assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            file: None,
            title: Some("rust-version invalid"),
            inventory: None,
        }],
    );
}

#[test]
fn invalid_workspace_package_rust_version_does_not_fallback_to_local_package_value() {
    let manifest = r#"
        [workspace]
        members = []
        resolver = "2"

        [workspace.package]
        edition = "2024"
        rust-version = 1.85

        [package]
        name = "helper"
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

    let results = check_results(&tree(
        &[("", entry(&[], &["Cargo.toml", "guardrail3.toml"]))],
        &[
            ("Cargo.toml", manifest),
            ("guardrail3.toml", "[profile]\nname = \"library\"\n"),
        ],
    ));

    guardrail3_app_rs_family_cargo_assertions::rs_cargo_15_rust_version_policy::assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            file: None,
            title: Some("rust-version invalid"),
            inventory: None,
        }],
    );
}
