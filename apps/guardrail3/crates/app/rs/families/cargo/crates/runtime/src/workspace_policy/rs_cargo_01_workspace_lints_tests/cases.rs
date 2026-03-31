use guardrail3_app_rs_family_cargo_assertions::rs_cargo_01_workspace_lints::{
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

#[test]
fn nested_package_under_workspace_is_not_treated_as_a_second_policy_root() {
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
    let nested_manifest = format!(
        r#"
            [package]
            name = "helper"
            edition = "2024"
            rust-version = "1.85"

            {WORKSPACE_RUST_LINTS}
            {WORKSPACE_CLIPPY_LINTS}
        "#
    );

    let results = check_results(&tree(
        &[
            ("", entry(&["crates", "tools"], &["Cargo.toml"])),
            ("crates", entry(&["api"], &[])),
            ("crates/api", entry(&[], &["Cargo.toml"])),
            ("tools", entry(&["helper"], &[])),
            ("tools/helper", entry(&[], &["Cargo.toml"])),
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
            ("tools/helper/Cargo.toml", &nested_manifest),
        ],
    ));

    guardrail3_app_rs_family_cargo_assertions::rs_cargo_01_workspace_lints::assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            file: Some("Cargo.toml"),
            title: None,
            inventory: Some(true),
        }],
    );
    assert!(
        results
            .iter()
            .all(|result| result.file() != Some("tools/helper/Cargo.toml")),
        "cargo family should ignore undeclared nested packages because placement legality belongs to topology: {results:#?}"
    );
}

#[test]
fn local_library_profile_requires_unreachable_pub() {
    let standalone_manifest = workspace_root_manifest(&format!(
        r#"
            [package]
            name = "helper"
            edition = "2024"

            {WORKSPACE_RUST_LINTS}
            {WORKSPACE_CLIPPY_LINTS}
        "#
    ));
    let standalone_manifest = standalone_manifest.replace(r#"unreachable_pub = "deny""#, "");

    let results = check_results(&tree(
        &[("", entry(&[], &["Cargo.toml", "guardrail3.toml"]))],
        &[
            ("Cargo.toml", &standalone_manifest),
            ("guardrail3.toml", "[profile]\nname = \"library\"\n"),
        ],
    ));

    guardrail3_app_rs_family_cargo_assertions::rs_cargo_01_workspace_lints::assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            file: Some("Cargo.toml"),
            title: Some("missing library rust lint `unreachable_pub`"),
            inventory: Some(false),
        }],
    );
}

#[test]
fn malformed_root_local_guardrail_suppresses_clean_inventory() {
    let standalone_manifest = format!(
        r#"
            [package]
            name = "helper"
            edition = "2024"

            {WORKSPACE_RUST_LINTS}
            {WORKSPACE_CLIPPY_LINTS}
        "#
    );

    let results = check_results(&tree(
        &[("", entry(&[], &["Cargo.toml", "guardrail3.toml"]))],
        &[
            ("Cargo.toml", &standalone_manifest),
            ("guardrail3.toml", "[profile"),
        ],
    ));

    guardrail3_app_rs_family_cargo_assertions::rs_cargo_01_workspace_lints::assert_rule_results(
        &results,
        &[],
    );
}

#[test]
fn invalid_clippy_table_shape_is_explicit_error() {
    let parsed = toml::from_str::<toml::Value>(
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

            [workspace.lints]
            clippy = "deny"
        "#,
    )
    .expect("valid test manifest");
    let root = crate::facts::PolicyRootCargoFacts {
        kind: crate::facts::PolicyRootKind::WorkspaceRoot,
        rel_dir: String::new(),
        cargo_rel_path: "Cargo.toml".to_owned(),
        parsed: Some(parsed),
        parse_error: None,
        guardrail_parse_error: false,
        members_parse_error: false,
        edition: None,
        edition_invalid: false,
        rust_version: None,
        rust_version_invalid: false,
        resolver: None,
        resolver_invalid: false,
        profile_name: None,
        escape_hatches: Vec::new(),
    };
    let input = crate::inputs::PolicyRootCargoInput::new(&root);
    let mut results = Vec::new();
    crate::rs_cargo_01_workspace_lints::check(&input, &mut results);

    guardrail3_app_rs_family_cargo_assertions::rs_cargo_01_workspace_lints::assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            file: Some("Cargo.toml"),
            title: Some("clippy lint table has invalid shape"),
            inventory: Some(false),
        }],
    );
}
