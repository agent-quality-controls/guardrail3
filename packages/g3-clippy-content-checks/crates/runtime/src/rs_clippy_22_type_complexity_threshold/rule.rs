use g3_clippy_content_checks_types::G3ClippyContentChecksInput;
use guardrail3_domain_modules::clippy::TYPE_COMPLEXITY_THRESHOLD;
use guardrail3_check_types::G3CheckResult;

use crate::support::check_threshold;

const ID: &str = "RS-CLIPPY-22";

pub(crate) fn check(input: &G3ClippyContentChecksInput, results: &mut Vec<G3CheckResult>) {
    check_threshold(
        ID,
        &input.clippy_rel_path,
        "type-complexity-threshold",
        input.clippy.type_complexity_threshold,
        TYPE_COMPLEXITY_THRESHOLD,
        results,
    );
}
