use crate::domain::modules::canonical;

use super::super::check;
use super::super::lint_support::{EXPECTED_CLIPPY_DENY, EXPECTED_CLIPPY_GROUPS, EXPECTED_RUST_LINTS};
use super::super::test_support::{entry, has_result, tree};

#[test]
fn complete_workspace_lints_inventory_passes() {
    let tree = tree(
        &[("", entry(&[], &["Cargo.toml"]))],
        &[(
            "Cargo.toml",
            r#"
                [workspace]
                members = []
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
            "#,
        )],
    );

    let results = check(&tree);
    assert!(has_result(&results, "RS-CARGO-01", |result| result.inventory));
}

#[test]
fn library_profile_requires_extra_workspace_rust_lints() {
    let tree = tree(
        &[("", entry(&[], &["Cargo.toml", "guardrail3.toml"]))],
        &[
            ("guardrail3.toml", "[profile]\nname = \"library\""),
            (
                "Cargo.toml",
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
                "#,
            ),
        ],
    );

    let results = check(&tree);
    assert!(has_result(&results, "RS-CARGO-01", |result| !result.inventory));
}

#[test]
fn expected_workspace_lints_match_canonical_module() {
    let parsed: toml::Value =
        toml::from_str(canonical::CARGO_LINTS.content).expect("canonical cargo lints should parse");
    let rust = parsed
        .get("workspace")
        .and_then(|value| value.get("lints"))
        .and_then(|value| value.get("rust"))
        .and_then(toml::Value::as_table)
        .expect("canonical rust lints table");
    let clippy = parsed
        .get("workspace")
        .and_then(|value| value.get("lints"))
        .and_then(|value| value.get("clippy"))
        .and_then(toml::Value::as_table)
        .expect("canonical clippy lints table");

    for expected in EXPECTED_RUST_LINTS {
        assert!(
            rust.contains_key(expected.name),
            "canonical cargo lints missing rust lint `{}`",
            expected.name
        );
    }

    for expected in EXPECTED_CLIPPY_GROUPS {
        assert!(
            clippy.contains_key(expected.name),
            "canonical cargo lints missing clippy group `{}`",
            expected.name
        );
    }

    for lint_name in EXPECTED_CLIPPY_DENY {
        assert!(
            clippy.contains_key(*lint_name),
            "canonical cargo lints missing clippy deny lint `{lint_name}`",
        );
    }
}
