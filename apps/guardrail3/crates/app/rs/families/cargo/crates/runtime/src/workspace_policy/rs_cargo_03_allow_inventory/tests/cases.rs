use guardrail3_app_rs_family_cargo_assertions::rs_cargo_03_allow_inventory::check_results;
use test_support::{entry, tree};

const STANDALONE_RUST_LINTS: &str = r#"
    [workspace.lints.rust]
    warnings = "deny"
    unsafe_code = "forbid"
    dead_code = "deny"
    unused_results = "deny"
    unused_crate_dependencies = "deny"
    missing_debug_implementations = "warn"
"#;

const STANDALONE_CLIPPY_LINTS: &str = r#"
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

const APPROVED_ALLOWS: &[&str] = &[
    "missing_docs_in_private_items",
    "module_name_repetitions",
    "must_use_candidate",
    "option_if_let_else",
    "empty_line_after_doc_comments",
    "single_match_else",
    "ref_option_ref",
    "trivially_copy_pass_by_ref",
    "multiple_crate_versions",
];

fn workspace_root_manifest(body: &str) -> String {
    format!(
        r#"
            [workspace]
            members = []
            resolver = "2"

            {body}
        "#
    )
}

fn approved_allow_guardrail(rel_path: &str) -> String {
    let mut toml = String::new();
    for lint in APPROVED_ALLOWS {
        toml.push_str(&format!(
            r#"[[escape_hatches]]
family = "cargo"
file = "{rel_path}"
kind = "lint_allow"
selector = "clippy:{lint}"
reason = "Legacy lint suppression while API cleanup lands."

"#
        ));
    }
    toml
}

#[test]
fn inventories_every_approved_allow_entry() {
    let manifest = workspace_root_manifest(&format!(
        r#"
            [package]
            name = "helper"
            edition = "2024"

            {STANDALONE_RUST_LINTS}
            {STANDALONE_CLIPPY_LINTS}
        "#
    ));
    let guardrail = approved_allow_guardrail("Cargo.toml");

    let results = check_results(&tree(
        &[("", entry(&[], &["Cargo.toml", "guardrail3.toml"]))],
        &[("Cargo.toml", &manifest), ("guardrail3.toml", &guardrail)],
    ));

    let rs_cargo_03: Vec<_> = results
        .iter()
        .filter(|result| result.id() == "RS-CARGO-03")
        .collect();
    assert_eq!(
        rs_cargo_03.len(),
        10,
        "unexpected RS-CARGO-03 results: {results:#?}"
    );
    for lint in APPROVED_ALLOWS {
        let expected_message = format!(
            "`Cargo.toml` explicitly allows `{lint}` in `clippy` with documented reason `Legacy lint suppression while API cleanup lands.`."
        );
        assert!(
            rs_cargo_03.iter().any(|result| {
                result.title() == "approved allow entry"
                    && result.message() == expected_message
                    && result.file() == Some("Cargo.toml")
                    && !result.inventory()
            }),
            "missing documented RS-CARGO-03 result for {lint}: {results:#?}"
        );
    }
    assert!(rs_cargo_03.iter().any(|result| {
        result.title() == "approved allow count"
            && result.message()
                == "`Cargo.toml` has 9 approved manifest allow entries (9 documented, 0 missing reasons, 0 weak reasons)."
            && result.file().is_none()
            && !result.inventory()
    }));
}

#[test]
fn invalid_allow_entry_value_is_inventoried_accurately() {
    let manifest = workspace_root_manifest(&format!(
        r#"
            [package]
            name = "helper"
            edition = "2024"

            {STANDALONE_RUST_LINTS}
            {STANDALONE_CLIPPY_LINTS}
        "#
    ))
    .replace(
        r#"module_name_repetitions = "allow""#,
        r#"module_name_repetitions = "banana""#,
    );
    let guardrail = approved_allow_guardrail("Cargo.toml");

    let results = check_results(&tree(
        &[("", entry(&[], &["Cargo.toml", "guardrail3.toml"]))],
        &[("Cargo.toml", &manifest), ("guardrail3.toml", &guardrail)],
    ));

    let rs_cargo_03: Vec<_> = results
        .iter()
        .filter(|result| result.id() == "RS-CARGO-03")
        .collect();
    assert_eq!(
        rs_cargo_03.len(),
        9,
        "unexpected RS-CARGO-03 results: {results:#?}"
    );
    assert!(
        rs_cargo_03
            .iter()
            .all(|result| !result.message().contains("module_name_repetitions")),
        "RS-CARGO-03 should ignore malformed allow entries owned by other rules: {results:#?}"
    );
    let count = results
        .iter()
        .find(|result| result.id() == "RS-CARGO-03" && result.title() == "approved allow count")
        .expect("expected RS-CARGO-03 count summary");
    assert_eq!(
        count.message(),
        "`Cargo.toml` has 8 approved manifest allow entries (8 documented, 0 missing reasons, 0 weak reasons)."
    );
}

#[test]
fn invalid_clippy_table_shape_emits_no_allow_inventory() {
    let manifest = workspace_root_manifest(&format!(
        r#"
            [package]
            name = "helper"
            edition = "2024"

            {STANDALONE_RUST_LINTS}

            [lints]
            clippy = "deny"
        "#
    ));

    let results = check_results(&tree(
        &[("", entry(&[], &["Cargo.toml", "guardrail3.toml"]))],
        &[("Cargo.toml", &manifest)],
    ));

    guardrail3_app_rs_family_cargo_assertions::rs_cargo_03_allow_inventory::assert_rule_results(
        &results,
        &[],
    );
}

#[test]
fn invalid_allow_priority_is_inventoried_accurately() {
    let manifest = workspace_root_manifest(&format!(
        r#"
            [package]
            name = "helper"
            edition = "2024"

            {STANDALONE_RUST_LINTS}
            {STANDALONE_CLIPPY_LINTS}
        "#
    ))
    .replace(
        r#"module_name_repetitions = "allow""#,
        r#"module_name_repetitions = { level = "allow", priority = "banana" }"#,
    );
    let guardrail = approved_allow_guardrail("Cargo.toml");

    let results = check_results(&tree(
        &[("", entry(&[], &["Cargo.toml", "guardrail3.toml"]))],
        &[("Cargo.toml", &manifest), ("guardrail3.toml", &guardrail)],
    ));

    let rs_cargo_03: Vec<_> = results
        .iter()
        .filter(|result| result.id() == "RS-CARGO-03")
        .collect();
    assert_eq!(
        rs_cargo_03.len(),
        10,
        "unexpected RS-CARGO-03 results: {results:#?}"
    );
    assert!(
        rs_cargo_03
            .iter()
            .any(|result| result.message().contains("`module_name_repetitions`")),
        "RS-CARGO-03 should still inventory allow-shaped entries even when RS-CARGO-02 owns the malformed priority: {results:#?}"
    );
    let count = results
        .iter()
        .find(|result| result.id() == "RS-CARGO-03" && result.title() == "approved allow count")
        .expect("expected RS-CARGO-03 count summary");
    assert_eq!(
        count.message(),
        "`Cargo.toml` has 9 approved manifest allow entries (9 documented, 0 missing reasons, 0 weak reasons)."
    );
}
