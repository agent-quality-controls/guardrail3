use g3rs_deny_types::G3RsDenyFileTreeChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

/// Stable identifier for the deny coverage rule.
const ID: &str = "g3rs-deny/coverage";

/// Runs the deny coverage rule and appends its findings to `results`.
pub(crate) fn check(input: &G3RsDenyFileTreeChecksInput, results: &mut Vec<G3CheckResult>) {
    for failure in &input.input_failures {
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            failure.title.clone(),
            failure.message.clone(),
            Some(failure.rel_path.clone()),
            None,
        ));
    }

    if let Some(selected_rel_path) = &input.selected_deny_rel_path {
        results.push(
            G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Info,
                "workspace root covered by deny config".to_owned(),
                format!("workspace root `.` is covered by `{selected_rel_path}`."),
                Some(selected_rel_path.clone()),
                None,
            )
            .into_inventory(),
        );
        return;
    }

    results.push(G3CheckResult::new(
        ID.to_owned(),
        G3Severity::Error,
        "workspace root uncovered by deny config".to_owned(),
        "workspace root `.` is not covered by any allowed deny config.".to_owned(),
        None,
        None,
    ));
}

#[cfg(test)]
#[path = "coverage_tests/mod.rs"]
// reason: file module tests live in the owned coverage_tests sidecar directory.
mod coverage_tests;
