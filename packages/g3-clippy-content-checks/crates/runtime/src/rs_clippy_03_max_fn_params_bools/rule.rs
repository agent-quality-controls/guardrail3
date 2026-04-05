use g3_clippy_content_checks_types::G3ClippyContentChecksInput;
use guardrail3_domain_modules::clippy::MAX_FN_PARAMS_BOOLS;
use guardrail3_check_types::G3CheckResult;

use crate::support::check_threshold;

const ID: &str = "RS-CLIPPY-03";

pub(crate) fn check(input: &G3ClippyContentChecksInput, results: &mut Vec<G3CheckResult>) {
    check_threshold(
        ID,
        &input.clippy_rel_path,
        "max-fn-params-bools",
        input.clippy.max_fn_params_bools,
        MAX_FN_PARAMS_BOOLS,
        results,
    );
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod tests;
