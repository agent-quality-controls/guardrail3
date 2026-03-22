use crate::domain::report::Severity;

use super::super::check;
use super::super::test_support::{entry, has_result, tree};

#[test]
fn matching_workspace_levels_inventory_passes() {
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
            "#,
        )],
    );

    let results = check(&tree);
    assert!(has_result(&results, "RS-CARGO-02", |result| result.inventory));
}

#[test]
fn weakened_lint_levels_error() {
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

                [workspace.lints.rust]
                warnings = "warn"
                unsafe_code = "allow"
                dead_code = "deny"
                unused_results = "deny"
                unused_crate_dependencies = "deny"
                missing_debug_implementations = "warn"
            "#,
        )],
    );

    let results = check(&tree);
    assert!(has_result(&results, "RS-CARGO-02", |result| {
        matches!(result.severity, Severity::Error)
    }));
}
