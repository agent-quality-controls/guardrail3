use super::{entry, tree};
use guardrail3_app_rs_family_cargo_assertions::rs_cargo_02_lint_levels::{
    ExpectedRuleResult, assert_rule_results, check_results, has_result, rule_results,
};

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
fn stricter_than_baseline_is_accepted_silently() {
    let manifest = format!(
        r#"
            [package]
            name = "helper"
            edition = "2024"

            {STANDALONE_RUST_LINTS}
            {STANDALONE_CLIPPY_LINTS}
        "#
    )
    .replace(
        r#"missing_debug_implementations = "warn""#,
        r#"missing_debug_implementations = "deny""#,
    );

    let results = check_results(&tree(
        &[("pkg", entry(&[], &["Cargo.toml"]))],
        &[("pkg/Cargo.toml", &manifest)],
    ));

    guardrail3_app_rs_family_cargo_assertions::rs_cargo_02_lint_levels::assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            file: None,
            title: None,
            inventory: Some(true),
        }],
    );
}

#[test]
fn weaker_levels_are_errors() {
    let manifest = format!(
        r#"
            [package]
            name = "helper"
            edition = "2024"

            {STANDALONE_RUST_LINTS}
            {STANDALONE_CLIPPY_LINTS}
        "#
    )
    .replace(r#"warnings = "deny""#, r#"warnings = "warn""#);

    let results = check_results(&tree(
        &[("pkg", entry(&[], &["Cargo.toml"]))],
        &[("pkg/Cargo.toml", &manifest)],
    ));

    guardrail3_app_rs_family_cargo_assertions::rs_cargo_02_lint_levels::assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            file: None,
            title: Some("lint `warnings` weakens policy"),
            inventory: None,
        }],
    );
}
