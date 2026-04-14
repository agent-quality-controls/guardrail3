use g3rs_clippy_types::G3RsClippyConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

use crate::baseline::TYPE_COMPLEXITY_THRESHOLD;
use crate::support::{check_threshold, typed_clippy};

const ID: &str = "RS-CLIPPY-CONFIG-08";

pub(crate) fn check(input: &G3RsClippyConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    let Some(clippy) = typed_clippy(input) else {
        return;
    };
    check_threshold(
        ID,
        &input.clippy_rel_path,
        "type-complexity-threshold",
        clippy.type_complexity_threshold,
        TYPE_COMPLEXITY_THRESHOLD,
        results,
    );
}
