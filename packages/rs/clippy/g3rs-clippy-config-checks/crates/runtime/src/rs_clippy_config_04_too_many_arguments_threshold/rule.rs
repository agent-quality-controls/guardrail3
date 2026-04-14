use g3rs_clippy_types::G3RsClippyConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

use crate::baseline::TOO_MANY_ARGUMENTS_THRESHOLD;
use crate::support::{check_threshold, typed_clippy};

const ID: &str = "RS-CLIPPY-CONFIG-04";

pub(crate) fn check(input: &G3RsClippyConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    let Some(clippy) = typed_clippy(input) else {
        return;
    };
    check_threshold(
        ID,
        &input.clippy_rel_path,
        "too-many-arguments-threshold",
        clippy.too_many_arguments_threshold,
        TOO_MANY_ARGUMENTS_THRESHOLD,
        results,
    );
}
