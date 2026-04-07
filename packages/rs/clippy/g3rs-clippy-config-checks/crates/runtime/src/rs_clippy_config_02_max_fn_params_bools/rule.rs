use g3rs_clippy_config_checks_types::G3RsClippyConfigChecksInput;
use guardrail3_domain_modules::clippy::MAX_FN_PARAMS_BOOLS;
use guardrail3_check_types::G3CheckResult;

use crate::support::check_threshold;

const ID: &str = "RS-CLIPPY-CONFIG-02";

pub(crate) fn check(input: &G3RsClippyConfigChecksInput, results: &mut Vec<G3CheckResult>) {
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
