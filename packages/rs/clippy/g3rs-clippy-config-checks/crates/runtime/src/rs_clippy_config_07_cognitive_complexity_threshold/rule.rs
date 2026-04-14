use g3rs_clippy_config_checks_types::G3RsClippyConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

use crate::baseline::COGNITIVE_COMPLEXITY_THRESHOLD;
use crate::support::{check_threshold, typed_clippy};

const ID: &str = "RS-CLIPPY-CONFIG-07";

pub(crate) fn check(input: &G3RsClippyConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    let Some(clippy) = typed_clippy(input) else {
        return;
    };
    check_threshold(
        ID,
        &input.clippy_rel_path,
        "cognitive-complexity-threshold",
        clippy.cognitive_complexity_threshold,
        COGNITIVE_COMPLEXITY_THRESHOLD,
        results,
    );
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod tests;
