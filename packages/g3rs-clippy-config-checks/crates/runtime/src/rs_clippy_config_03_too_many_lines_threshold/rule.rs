use g3rs_clippy_config_checks_types::G3RsClippyConfigChecksInput;
use guardrail3_domain_modules::clippy::TOO_MANY_LINES_THRESHOLD;
use guardrail3_check_types::G3CheckResult;

use crate::support::check_threshold;

const ID: &str = "RS-CLIPPY-CONFIG-03";

pub(crate) fn check(input: &G3RsClippyConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    check_threshold(
        ID,
        &input.clippy_rel_path,
        "too-many-lines-threshold",
        input.clippy.too_many_lines_threshold,
        TOO_MANY_LINES_THRESHOLD,
        results,
    );
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod tests;
