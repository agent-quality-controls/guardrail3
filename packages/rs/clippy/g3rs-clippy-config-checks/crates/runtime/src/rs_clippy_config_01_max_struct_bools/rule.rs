use g3rs_clippy_types::G3RsClippyConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

use crate::baseline::MAX_STRUCT_BOOLS;
use crate::support::{check_threshold, has_matching_waiver, typed_clippy};

const ID: &str = "RS-CLIPPY-CONFIG-01";
const SELECTOR: &str = "key:max-struct-bools";

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
