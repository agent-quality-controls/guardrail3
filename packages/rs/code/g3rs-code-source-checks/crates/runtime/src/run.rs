use g3rs_code_types::G3RsCodeSourceChecksInput;
use guardrail3_check_types::G3CheckResult;

#[must_use]
pub fn check(input: &G3RsCodeSourceChecksInput) -> Vec<G3CheckResult> {
    let prepared_input = match crate::support::parse_input(input) {
        Ok(prepared_input) => prepared_input,
        Err(parse_failure) => {
            let mut results = Vec::new();
            crate::input_failures::check(&parse_failure, &mut results);
            return results;
        }
    };
    let rule_input = prepared_input.rule_input();
    let mut results = Vec::new();

    crate::crate_level_allow::check(&rule_input, &mut results);
    crate::unused_crate_dependencies_allow::check(&rule_input, &mut results);
    crate::item_level_allow_without_reason::check(&rule_input, &mut results);
    crate::item_level_allow_with_reason::check(&rule_input, &mut results);
    crate::garde_skip_without_comment::check(&rule_input, &mut results);
    crate::garde_skip_with_comment::check(&rule_input, &mut results);
    crate::cfg_attr_allow_inventory::check(&rule_input, &mut results);
    crate::too_many_effective_code_lines::check(&rule_input, &mut results);
    crate::too_many_use_imports::check(&rule_input, &mut results);
    crate::many_use_imports::check(&rule_input, &mut results);
    crate::todo_macros::check(&rule_input, &mut results);
    crate::direct_fs_usage::check(&rule_input, &mut results);
    crate::panic_macro::check(&rule_input, &mut results);
    crate::impl_allow_blast_radius::check(&rule_input, &mut results);
    crate::always_true_cfg_attr_bypass::check(&rule_input, &mut results);
    crate::large_type_inventory::check(&rule_input, &mut results);
    crate::extern_allow::check(&rule_input, &mut results);
    crate::fs_glob_import::check(&rule_input, &mut results);
    crate::deny_forbid_without_reason::check(&rule_input, &mut results);
    crate::include_bypass::check(&rule_input, &mut results);
    crate::path_attr_with_reason::check(&rule_input, &mut results);
    crate::large_trait_surface::check(&rule_input, &mut results);
    crate::test_expect_message_quality::check(&rule_input, &mut results);
    crate::public_struct_named_fields::check(&rule_input, &mut results);
    crate::public_weak_error_forms::check(&rule_input, &mut results);
    crate::generic_parameter_cap::check(&rule_input, &mut results);
    crate::string_dispatch_cap::check(&rule_input, &mut results);

    results
}
