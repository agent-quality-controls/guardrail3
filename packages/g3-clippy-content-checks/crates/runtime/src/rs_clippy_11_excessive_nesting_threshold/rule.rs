use g3_clippy_content_checks_types::G3ClippyContentChecksInput;
use guardrail3_domain_modules::clippy::EXCESSIVE_NESTING_THRESHOLD;
use guardrail3_check_types::G3CheckResult;

use crate::support::check_threshold;

const ID: &str = "RS-CLIPPY-11";

pub(crate) fn check(input: &G3ClippyContentChecksInput, results: &mut Vec<G3CheckResult>) {
    check_threshold(
        ID,
        &input.clippy_rel_path,
        "excessive-nesting-threshold",
        input.clippy.excessive_nesting_threshold,
        EXCESSIVE_NESTING_THRESHOLD,
        results,
    );
}
