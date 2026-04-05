use g3_clippy_content_checks_types::G3ClippyContentChecksInput;
use guardrail3_check_types::G3CheckResult;

pub fn check(input: &G3ClippyContentChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    crate::rs_clippy_02_max_struct_bools::check(input, &mut results);
    crate::rs_clippy_03_max_fn_params_bools::check(input, &mut results);
    crate::rs_clippy_09_too_many_lines_threshold::check(input, &mut results);
    crate::rs_clippy_10_too_many_arguments_threshold::check(input, &mut results);
    crate::rs_clippy_11_excessive_nesting_threshold::check(input, &mut results);
    crate::rs_clippy_17_test_relaxations::check(input, &mut results);
    crate::rs_clippy_21_cognitive_complexity_threshold::check(input, &mut results);
    crate::rs_clippy_22_type_complexity_threshold::check(input, &mut results);
    results
}
