use g3rs_code_types::G3RsCodeSourceChecksInput;
use guardrail3_check_types::G3CheckResult;

pub fn check(input: &G3RsCodeSourceChecksInput) -> Vec<G3CheckResult> {
    let parsed = match crate::support::parse_input(input) {
        Ok(parsed) => parsed,
        Err(parse_error) => {
            let parse_failure = crate::support::parse_failure_input(input, &parse_error);
            let mut results = Vec::new();
            crate::rs_code_ast_30_input_failures::check(&parse_failure, &mut results);
            return results;
        }
    };
    let rule_input = crate::support::CodeSourceRuleInput {
        rel_path: &parsed.source_file.rel_path,
        content: &parsed.source_file.content,
        source: &parsed.source,
        is_test: parsed.source_file.is_test,
        is_shared_crate: input.is_shared_crate,
        profile_name: parsed.source_file.profile_name.as_deref(),
        is_library_root: parsed.source_file.is_library_root,
    };
    let mut results = Vec::new();

    crate::rs_code_ast_01_crate_level_allow::check(&rule_input, &mut results);
    crate::rs_code_ast_02_unused_crate_dependencies_allow::check(&rule_input, &mut results);
    crate::rs_code_ast_03_item_level_allow_without_reason::check(&rule_input, &mut results);
    crate::rs_code_ast_04_item_level_allow_with_reason::check(&rule_input, &mut results);
    crate::rs_code_ast_05_garde_skip_without_comment::check(&rule_input, &mut results);
    crate::rs_code_ast_06_garde_skip_with_comment::check(&rule_input, &mut results);
    crate::rs_code_ast_08_cfg_attr_allow_inventory::check(&rule_input, &mut results);
    crate::rs_code_ast_09_too_many_effective_code_lines::check(&rule_input, &mut results);
    crate::rs_code_ast_10_too_many_use_imports::check(&rule_input, &mut results);
    crate::rs_code_ast_11_many_use_imports::check(&rule_input, &mut results);
    crate::rs_code_ast_13_todo_macros::check(&rule_input, &mut results);
    crate::rs_code_ast_15_direct_fs_usage::check(&rule_input, &mut results);
    crate::rs_code_ast_16_panic_macro::check(&rule_input, &mut results);
    crate::rs_code_ast_17_impl_allow_blast_radius::check(&rule_input, &mut results);
    crate::rs_code_ast_18_always_true_cfg_attr_bypass::check(&rule_input, &mut results);
    crate::rs_code_ast_19_large_type_inventory::check(&rule_input, &mut results);
    crate::rs_code_ast_20_extern_allow::check(&rule_input, &mut results);
    crate::rs_code_ast_21_fs_glob_import::check(&rule_input, &mut results);
    crate::rs_code_ast_22_deny_forbid_without_reason::check(&rule_input, &mut results);
    crate::rs_code_ast_23_include_bypass::check(&rule_input, &mut results);
    crate::rs_code_ast_24_path_attr_with_reason::check(&rule_input, &mut results);
    crate::rs_code_ast_29_large_trait_surface::check(&rule_input, &mut results);
    crate::rs_code_ast_32_test_expect_message_quality::check(&rule_input, &mut results);
    crate::rs_code_ast_31_public_struct_named_fields::check(&rule_input, &mut results);
    crate::rs_code_ast_33_public_weak_error_forms::check(&rule_input, &mut results);
    crate::rs_code_ast_34_generic_parameter_cap::check(&rule_input, &mut results);
    crate::rs_code_ast_36_string_dispatch_cap::check(&rule_input, &mut results);

    results
}
