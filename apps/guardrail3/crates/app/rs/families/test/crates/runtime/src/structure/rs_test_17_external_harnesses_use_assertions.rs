use crate::{CheckResult, Severity};

use super::facts::TestFileKind;
use super::inputs::TestFunctionInput;
use super::rs_test_07_real_proof_site::has_owned_assertion_proof;

const ID: &str = "RS-TEST-17";

pub fn check(input: &TestFunctionInput<'_>, results: &mut Vec<CheckResult>) {
    if !matches!(input.file.kind, TestFileKind::ExternalHarness) {
        return;
    }

    if input.function.has_assertion_macro {
        results.push(CheckResult::from_parts(
    ID.to_owned(),
    Severity::Error,
    "external harness asserts directly".to_owned(),
    "External harnesses must prove through the owned assertions crate, not through direct assertion macros in `runtime/tests/*.rs`.".to_owned(),
    Some(input.file.rel_path.clone()),
    Some(input.function.line),
    false,
        ));
        return;
    }

    if has_owned_assertion_proof(
        input.function,
        &input.parsed.imports,
        &input.parsed.file_function_names,
        input.file.assertions_package_name.as_deref(),
        input.proof_bearing_assertion_functions,
    ) {
        results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                "external harness uses owned assertions".to_owned(),
                "External harnesses stay black-box and prove through the owned assertions crate rather than direct assertion macros.".to_owned(),
                Some(input.file.rel_path.clone()),
                Some(input.function.line),
                false,
            )
            .as_inventory(),
        );
    }
}

#[cfg(test)]
pub(crate) fn run_family(root: &std::path::Path) -> Vec<CheckResult> {
    let tree = test_support::walk(root);
    super::check_test_tree(&tree, &test_support::StubToolChecker::default())
}
#[cfg(test)]
#[path = "rs_test_17_external_harnesses_use_assertions_tests/mod.rs"]
mod rs_test_17_external_harnesses_use_assertions_tests;
