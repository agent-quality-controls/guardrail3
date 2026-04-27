use g3rs_clippy_types::G3RsClippyConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

use crate::baseline::TOO_MANY_LINES_THRESHOLD;
use crate::support::{check_threshold, typed_clippy};

const ID: &str = "g3rs-clippy/too-many-lines-threshold";

pub(crate) fn check(input: &G3RsClippyConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    let Some(clippy) = typed_clippy(input) else {
        return;
    };
    check_threshold(
        ID,
        &input.clippy_rel_path,
        "too-many-lines-threshold",
        clippy.too_many_lines_threshold,
        TOO_MANY_LINES_THRESHOLD,
        results,
    );
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rule_tests;
