use guardrail3_domain_report::{CheckResult, Severity};
use test_support::{entry, tree};

const REQUIRED_RUST_LINTS: &str = r#"
    [workspace.lints.rust]
    warnings = "deny"
    unsafe_code = "forbid"
    dead_code = "deny"
    unused_results = "deny"
    unused_crate_dependencies = "deny"
    missing_debug_implementations = "warn"
    unreachable_pub = "deny"
"#;

const REQUIRED_CLIPPY_LINTS: &str = r#"
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
    redundant_pub_crate = "allow"
"#;

#[test]
fn inventories_migrated_workspace_lints_rule_through_family_package_bridge() {
    let manifest = format!(
        r#"
            [workspace]
            members = []
            resolver = "2"

            [workspace.package]
            edition = "2024"
            rust-version = "1.85"

            {REQUIRED_RUST_LINTS}
            {REQUIRED_CLIPPY_LINTS}
        "#
    );
    let results = check_results(&tree(
        &[("", entry(&[], &["Cargo.toml"]))],
        &[("Cargo.toml", &manifest)],
    ));

    let package_results = results_for_id(&results, "RS-CARGO-CONFIG-01");
    assert_eq!(package_results.len(), 1);
    assert_eq!(package_results[0].severity(), Severity::Info);
    assert!(package_results[0].inventory());
    assert_eq!(package_results[0].title(), "workspace lint tables present");
    assert_eq!(package_results[0].file(), Some("Cargo.toml"));

    let parseability = results_for_id(&results, "RS-CARGO-14");
    assert_eq!(parseability.len(), 1);
    assert_eq!(parseability[0].severity(), Severity::Info);
    assert!(parseability[0].inventory());
}

#[test]
fn reports_migrated_workspace_lints_failure_through_family_package_bridge() {
    let manifest = format!(
        r#"
            [workspace]
            members = []
            resolver = "2"

            [workspace.package]
            edition = "2024"
            rust-version = "1.85"

            {REQUIRED_RUST_LINTS}
        "#
    );
    let results = check_results(&tree(
        &[("", entry(&[], &["Cargo.toml"]))],
        &[("Cargo.toml", &manifest)],
    ));

    let package_results = results_for_id(&results, "RS-CARGO-CONFIG-01");
    assert_eq!(package_results.len(), 1);
    assert_eq!(package_results[0].severity(), Severity::Error);
    assert!(!package_results[0].inventory());
    assert_eq!(package_results[0].title(), "clippy lint table missing");
    assert_eq!(package_results[0].file(), Some("Cargo.toml"));

    let parseability = results_for_id(&results, "RS-CARGO-14");
    assert_eq!(parseability.len(), 1);
    assert_eq!(parseability[0].severity(), Severity::Info);
    assert!(parseability[0].inventory());
}

fn results_for_id<'a>(results: &'a [CheckResult], id: &str) -> Vec<&'a CheckResult> {
    results.iter().filter(|result| result.id() == id).collect()
}

fn check_results(tree: &guardrail3_domain_project_tree::ProjectTree) -> Vec<CheckResult> {
    crate::check_test_tree(tree)
}
