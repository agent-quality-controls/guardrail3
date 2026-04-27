use g3rs_clippy_types::G3RsClippyConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

pub fn check(input: &G3RsClippyConfigChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    crate::max_struct_bools::check(input, &mut results);
    crate::max_fn_params_bools::check(input, &mut results);
    crate::too_many_lines_threshold::check(input, &mut results);
    crate::too_many_arguments_threshold::check(input, &mut results);
    crate::excessive_nesting_threshold::check(input, &mut results);
    crate::test_relaxations::check(input, &mut results);
    crate::cognitive_complexity_threshold::check(input, &mut results);
    crate::type_complexity_threshold::check(input, &mut results);
    crate::missing_method_ban::check(input, &mut results);
    crate::missing_type_ban::check(input, &mut results);
    crate::extra_method_ban::check(input, &mut results);
    crate::extra_type_ban::check(input, &mut results);
    crate::ban_reason_quality::check(input, &mut results);
    crate::library_global_state::check(input, &mut results);
    crate::avoid_breaking_exported_api::check(input, &mut results);
    crate::duplicate_bans::check(input, &mut results);
    crate::unknown_keys::check(input, &mut results);
    crate::macro_bans::check(input, &mut results);
    crate::policy_context_parseable::check(input, &mut results);
    crate::forbid_clippy_conf_dir_override::check(input, &mut results);
    crate::config_parseable::check(input, &mut results);
    results
}
