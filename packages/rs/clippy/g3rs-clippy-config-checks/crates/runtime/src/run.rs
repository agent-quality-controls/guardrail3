use g3rs_clippy_config_checks_types::G3RsClippyConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

pub fn check(input: &G3RsClippyConfigChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    crate::rs_clippy_config_01_max_struct_bools::check(input, &mut results);
    crate::rs_clippy_config_02_max_fn_params_bools::check(input, &mut results);
    crate::rs_clippy_config_03_too_many_lines_threshold::check(input, &mut results);
    crate::rs_clippy_config_04_too_many_arguments_threshold::check(input, &mut results);
    crate::rs_clippy_config_05_excessive_nesting_threshold::check(input, &mut results);
    crate::rs_clippy_config_06_test_relaxations::check(input, &mut results);
    crate::rs_clippy_config_07_cognitive_complexity_threshold::check(input, &mut results);
    crate::rs_clippy_config_08_type_complexity_threshold::check(input, &mut results);
    crate::rs_clippy_config_09_missing_method_ban::check(input, &mut results);
    crate::rs_clippy_config_10_missing_type_ban::check(input, &mut results);
    crate::rs_clippy_config_11_extra_method_ban::check(input, &mut results);
    crate::rs_clippy_config_12_extra_type_ban::check(input, &mut results);
    crate::rs_clippy_config_13_ban_reason_quality::check(input, &mut results);
    crate::rs_clippy_config_14_library_global_state::check(input, &mut results);
    crate::rs_clippy_config_15_avoid_breaking_exported_api::check(input, &mut results);
    crate::rs_clippy_config_16_duplicate_bans::check(input, &mut results);
    crate::rs_clippy_config_17_unknown_keys::check(input, &mut results);
    crate::rs_clippy_config_18_macro_bans::check(input, &mut results);
    crate::rs_clippy_config_19_policy_context_parseable::check(input, &mut results);
    crate::rs_clippy_config_20_forbid_clippy_conf_dir_override::check(input, &mut results);
    crate::rs_clippy_config_21_config_parseable::check(input, &mut results);
    results
}
