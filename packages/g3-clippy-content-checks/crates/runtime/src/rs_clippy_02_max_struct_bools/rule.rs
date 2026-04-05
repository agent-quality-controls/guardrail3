use g3_clippy_content_checks_types::G3ClippyContentChecksInput;
use guardrail3_domain_modules::clippy::MAX_STRUCT_BOOLS;
use guardrail3_check_types::G3CheckResult;

use crate::support::check_threshold;

const ID: &str = "RS-CLIPPY-02";

pub(crate) fn check(input: &G3ClippyContentChecksInput, results: &mut Vec<G3CheckResult>) {
    check_threshold(
        ID,
        &input.clippy_rel_path,
        "max-struct-bools",
        input.clippy.max_struct_bools,
        MAX_STRUCT_BOOLS,
        results,
    );
}
