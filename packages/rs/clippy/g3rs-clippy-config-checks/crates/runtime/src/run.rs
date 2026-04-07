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
    results
}
