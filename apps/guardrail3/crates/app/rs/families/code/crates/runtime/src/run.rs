use crate::facts::collect;
use crate::inputs::{
    CodeInputFailureInput, ExceptionCommentInput, RustCodeFileInput, StructuralCapInput,
    UnsafeCodeLintInput,
};

pub fn check(
    surface: &guardrail3_app_rs_family_view::FamilyView,
    route: &guardrail3_app_rs_family_mapper::RsCodeRoute,
) -> Vec<guardrail3_domain_report::CheckResult> {
    mark_runtime_dependencies_used();
    let tree = surface;
    let facts = collect(tree, route);
    let mut results = Vec::new();

    for failure in &facts.input_failures {
        let input = CodeInputFailureInput::new(failure);
        crate::lint_policy::rs_code_30_input_failures::check(&input, &mut results);
    }

    for lint in &facts.unsafe_code_lints {
        let input = UnsafeCodeLintInput::new(lint);
        crate::lint_policy::rs_code_12_unsafe_code_lint::check(&input, &mut results);
    }

    for exception_comment in &facts.exception_comments {
        let input = ExceptionCommentInput::new(exception_comment);
        crate::lint_policy::rs_code_07_exception_comment_inventory::check(&input, &mut results);
    }

    for structural_cap in &facts.structural_caps {
        let input = StructuralCapInput::new(structural_cap);
        crate::inventory::rs_code_35_root_structural_cap::check(&input, &mut results);
    }

    for file in &facts.files {
        if route
            .scoped_files()
            .is_some_and(|files| !files.contains(&file.rel_path))
        {
            continue;
        }
        let Some(abs) = tree.abs_path(&file.rel_path) else { continue };
        let content = match guardrail3_shared_fs::read_file_err(&abs) {
            Ok(content) => content,
            Err(read_error) => {
                let message = format!("Failed to read Rust source file: {read_error}");
                let failure = CodeInputFailureInput {
                    rel_path: &file.rel_path,
                    message: &message,
                };
                crate::lint_policy::rs_code_30_input_failures::check(&failure, &mut results);
                continue;
            }
        };
        let ast = match crate::parse::parse_rust_file(&content) {
            Ok(ast) => ast,
            Err(parse_error) => {
                let message = format!("Failed to parse Rust source file: {parse_error}");
                let failure = CodeInputFailureInput {
                    rel_path: &file.rel_path,
                    message: &message,
                };
                crate::lint_policy::rs_code_30_input_failures::check(&failure, &mut results);
                continue;
            }
        };

        let input = RustCodeFileInput::new(file, &content, &ast);
        crate::lint_policy::rs_code_01_crate_level_allow::check(&input, &mut results);
        crate::lint_policy::rs_code_02_unused_crate_dependencies_allow::check(&input, &mut results);
        crate::lint_policy::rs_code_03_item_level_allow_without_reason::check(&input, &mut results);
        crate::lint_policy::rs_code_04_item_level_allow_with_reason::check(&input, &mut results);
        crate::hygiene::rs_code_05_garde_skip_without_comment::check(&input, &mut results);
        crate::hygiene::rs_code_06_garde_skip_with_comment::check(&input, &mut results);
        crate::lint_policy::rs_code_08_cfg_attr_allow_inventory::check(&input, &mut results);
        crate::hygiene::rs_code_09_file_length::check(&input, &mut results);
        crate::hygiene::rs_code_10_use_count_error::check(&input, &mut results);
        crate::hygiene::rs_code_11_use_count_warn::check(&input, &mut results);
        crate::hygiene::rs_code_13_todo_macros::check(&input, &mut results);
        crate::hygiene::rs_code_15_direct_fs_usage::check(&input, &mut results);
        crate::hygiene::rs_code_16_panic_macro::check(&input, &mut results);
        crate::cfg_and_paths::rs_code_17_impl_allow_blast_radius::check(&input, &mut results);
        crate::cfg_and_paths::rs_code_18_always_true_cfg_attr_bypass::check(&input, &mut results);
        crate::inventory::rs_code_19_large_type_inventory::check(&input, &mut results);
        crate::cfg_and_paths::rs_code_20_extern_allow::check(&input, &mut results);
        crate::cfg_and_paths::rs_code_21_fs_glob_import::check(&input, &mut results);
        crate::lint_policy::rs_code_22_deny_forbid_without_reason::check(&input, &mut results);
        crate::cfg_and_paths::rs_code_23_include_bypass::check(&input, &mut results);
        // RS-CODE-24 removed: #[path] detection moved to RS-ARCH-09.
        crate::api_shape::rs_code_31_public_struct_named_fields::check(&input, &mut results);
        crate::api_shape::rs_code_25_public_result_error_type::check(&input, &mut results);
        crate::api_shape::rs_code_33_public_weak_error_forms::check(&input, &mut results);
        crate::api_shape::rs_code_34_generic_parameter_cap::check(&input, &mut results);
        crate::api_shape::rs_code_26_lib_glob_reexport::check(&input, &mut results);
        crate::api_shape::rs_code_27_facade_only_lib::check(&input, &mut results);
        crate::inventory::rs_code_29_large_trait_inventory::check(&input, &mut results);
        crate::inventory::rs_code_32_test_expect_message_quality::check(&input, &mut results);
        crate::cfg_and_paths::rs_code_36_string_dispatch_cap::check(&input, &mut results);
    }

    results
}

fn mark_runtime_dependencies_used() {
    use {
        glob as _, guardrail3_domain_config as _, guardrail3_domain_modules as _,
        guardrail3_outbound_traits as _, quote as _, semver as _, serde_yaml as _,
    };
}
