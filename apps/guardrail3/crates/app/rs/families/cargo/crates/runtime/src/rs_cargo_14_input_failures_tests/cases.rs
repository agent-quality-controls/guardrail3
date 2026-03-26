#[allow(unused_imports)]
use super::{check_results, entry, rule_results, tree};

#[allow(dead_code, non_upper_case_globals)]
const workspace_rust_lints: &str = r#"
    [workspace.lints.rust]
    warnings = "deny"
    unsafe_code = "forbid"
    dead_code = "deny"
    unused_results = "deny"
    unused_crate_dependencies = "deny"
    missing_debug_implementations = "warn"
"#;

#[allow(dead_code, non_upper_case_globals)]
const workspace_clippy_lints: &str = r#"
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

#[allow(dead_code, non_upper_case_globals)]
const standalone_rust_lints: &str = r#"
    [lints.rust]
    warnings = "deny"
    unsafe_code = "forbid"
    dead_code = "deny"
    unused_results = "deny"
    unused_crate_dependencies = "deny"
    missing_debug_implementations = "warn"
"#;

#[allow(dead_code, non_upper_case_globals)]
const standalone_clippy_lints: &str = r#"
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
fn malformed_owned_policy_root_manifest_surfaces_explicit_failure() {
    let results = check_results(&tree(
        &[("", entry(&[], &["Cargo.toml"]))],
        &[("Cargo.toml", "[workspace")],
    ));

    let rule = rule_results(&results, "RS-CARGO-14");
    assert_eq!(rule.len(), 1, "unexpected results: {rule:#?}");
    assert_eq!(rule[0].file.as_deref(), Some("Cargo.toml"));
}

#[test]
fn malformed_workspace_member_manifest_surfaces_explicit_failure() {
    let workspace_manifest = format!(
        r#"
            [workspace]
            members = ["crates/api"]
            resolver = "2"

            [workspace.package]
            edition = "2024"
            rust-version = "1.85"

            {workspace_rust_lints}
            {workspace_clippy_lints}
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

    let rule = rule_results(&results, "RS-CARGO-14");
    assert_eq!(rule.len(), 1, "unexpected results: {rule:#?}");
    assert_eq!(rule[0].file.as_deref(), Some("crates/api/Cargo.toml"));
}

#[test]
fn malformed_root_local_guardrail_surfaces_explicit_failure() {
    let workspace_manifest = format!(
        r#"
            [workspace]
            members = []
            resolver = "2"

            [workspace.package]
            edition = "2024"
            rust-version = "1.85"

            {workspace_rust_lints}
            {workspace_clippy_lints}
        "#
    );
    let results = check_results(&tree(
        &[("", entry(&[], &["Cargo.toml", "guardrail3.toml"]))],
        &[
            ("Cargo.toml", &workspace_manifest),
            ("guardrail3.toml", "[profile"),
        ],
    ));

    let rule = rule_results(&results, "RS-CARGO-14");
    assert_eq!(rule.len(), 1, "unexpected results: {rule:#?}");
    assert_eq!(rule[0].file.as_deref(), Some("guardrail3.toml"));
}
