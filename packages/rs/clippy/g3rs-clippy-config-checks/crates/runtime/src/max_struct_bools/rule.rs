use g3rs_clippy_types::G3RsClippyConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

use crate::baseline::MAX_STRUCT_BOOLS;
use crate::support::{check_threshold, has_matching_waiver, typed_clippy};

/// I D const.
const ID: &str = "g3rs-clippy/max-struct-bools";
/// S E L E C T O R const.
const SELECTOR: &str = "key:max-struct-bools";

/// check fn.
pub(crate) fn check(input: &G3RsClippyConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    if has_matching_waiver(input, ID, SELECTOR) {
        return;
    }
    let Some(clippy) = typed_clippy(input) else {
        return;
    };
    check_threshold(
        ID,
        &input.clippy_rel_path,
        "max-struct-bools",
        clippy.max_struct_bools,
        MAX_STRUCT_BOOLS,
        results,
    );
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rule_tests;
