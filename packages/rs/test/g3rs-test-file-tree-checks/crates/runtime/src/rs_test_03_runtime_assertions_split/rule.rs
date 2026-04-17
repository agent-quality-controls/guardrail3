use g3rs_test_types::G3RsTestFileTreeChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

use super::violations::collect_violations;
use crate::support::RootAnalysis;

const ID: &str = "RS-TEST-FILETREE-03";

#[cfg(test)]
#[path = "rule_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rule_tests;

pub(crate) fn collect(
    input: &G3RsTestFileTreeChecksInput,
    analysis: &RootAnalysis,
    results: &mut Vec<G3CheckResult>,
) {
    let violations = collect_violations(input, analysis);
    if violations.is_empty() {
        results.push(
            G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Info,
                "runtime/assertions split confirmed".to_owned(),
                format!(
                    "Root `{}` keeps runtime harnesses separated from sibling assertions crates.",
                    input.root_rel_dir
                ),
                Some(input.cargo_rel_path.clone()),
                None,
            )
            .into_inventory(),
        );
        return;
    }

    for violation in violations {
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            violation.title,
            violation.message,
            Some(violation.rel_path),
            violation.line,
        ));
    }
}
