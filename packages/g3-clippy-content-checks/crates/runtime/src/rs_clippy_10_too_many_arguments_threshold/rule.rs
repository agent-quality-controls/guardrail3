use g3_clippy_content_checks_types::G3ClippyContentChecksInput;
use guardrail3_domain_modules::clippy::TOO_MANY_ARGUMENTS_THRESHOLD;
use guardrail3_check_types::G3CheckResult;

use crate::support::check_threshold;

const ID: &str = "RS-CLIPPY-10";

pub(crate) fn check(input: &G3ClippyContentChecksInput, results: &mut Vec<G3CheckResult>) {
    check_threshold(
        ID,
        &input.clippy_rel_path,
        "too-many-arguments-threshold",
        input.clippy.too_many_arguments_threshold,
        TOO_MANY_ARGUMENTS_THRESHOLD,
        results,
    );
}
