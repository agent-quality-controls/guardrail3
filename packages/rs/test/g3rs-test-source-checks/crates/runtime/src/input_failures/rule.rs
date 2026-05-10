use guardrail3_check_types::{G3CheckResult, G3Severity};

/// `ID` constant.
const ID: &str = "g3rs-test/source-input-failures";

/// `check` function.
pub(crate) fn check(
    _root_rel_dir: &str,
    rel_path: &str,
    message: &str,
    results: &mut Vec<G3CheckResult>,
) {
    results.push(G3CheckResult::new(
        ID.to_owned(),
        G3Severity::Error,
        "failed to read test input".to_owned(),
        message.to_owned(),
        Some(rel_path.to_owned()),
        None,
    ));
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rule_tests;
