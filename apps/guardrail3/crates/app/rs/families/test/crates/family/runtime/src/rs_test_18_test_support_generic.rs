use std::collections::BTreeSet;

use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::TestSupportFileInput;

const ID: &str = "RS-TEST-18";

pub fn check(input: &TestSupportFileInput<'_>, results: &mut Vec<CheckResult>) {
    let disallowed_packages = input
        .local_runtime_packages
        .iter()
        .chain(input.local_assertions_packages.iter())
        .cloned()
        .collect::<BTreeSet<_>>();
    if disallowed_packages.is_empty() {
        return;
    }

    for binding in &input.parsed.imports {
        let Some(first) = binding.path_segments.first() else {
            continue;
        };
        if !disallowed_packages.contains(first) {
            continue;
        }
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: "test_support imports local component crate".to_owned(),
            message: format!(
                "Shared `test_support` must stay generic and must not import local runtime/assertions crate `{first}`."
            ),
            file: Some(input.file.rel_path.clone()),
            line: Some(binding.line),
            inventory: false,
        });
    }

    let mut called_packages = BTreeSet::new();
    for call_path in &input.parsed.file_call_paths {
        let Some(first) = call_path.first() else {
            continue;
        };
        if !disallowed_packages.contains(first) || !called_packages.insert(first.clone()) {
            continue;
        }
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: "test_support calls local component crate".to_owned(),
            message: format!(
                "Shared `test_support` must stay generic and must not call local runtime/assertions crate `{first}` directly."
            ),
            file: Some(input.file.rel_path.clone()),
            line: None,
            inventory: false,
        });
    }
}

#[cfg(test)]
#[path = "rs_test_18_test_support_generic_tests/mod.rs"]
mod rs_test_18_test_support_generic_tests;
