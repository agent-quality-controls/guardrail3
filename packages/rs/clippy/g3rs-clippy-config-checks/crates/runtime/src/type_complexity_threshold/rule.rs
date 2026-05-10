use g3rs_clippy_types::G3RsClippyConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

use crate::baseline::TYPE_COMPLEXITY_THRESHOLD;
use crate::support::{check_threshold, typed_clippy};

/// I D const.
const ID: &str = "g3rs-clippy/type-complexity-threshold";

/// check fn.
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

#[cfg(test)]
#[path = "rule_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rule_tests;
