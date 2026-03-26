use guardrail3_domain_report::{CheckResult, Severity};

use super::facts::TestFileKind;
use super::inputs::TestFunctionInput;

const ID: &str = "RS-TEST-17";

pub fn check(input: &TestFunctionInput<'_>, results: &mut Vec<CheckResult>) {
    if !matches!(input.file.kind, TestFileKind::ExternalHarness)
        || !input.function.has_assertion_macro
    {
        return;
    }

    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Error,
        title: "external harness asserts directly".to_owned(),
        message: "External harnesses must prove through the owned assertions crate, not through direct assertion macros in `runtime/tests/*.rs`.".to_owned(),
        file: Some(input.file.rel_path.clone()),
        line: Some(input.function.line),
        inventory: false,
    });
}

#[cfg(test)]
#[path = "rs_test_17_external_harnesses_use_assertions_tests/mod.rs"]
mod rs_test_17_external_harnesses_use_assertions_tests;
