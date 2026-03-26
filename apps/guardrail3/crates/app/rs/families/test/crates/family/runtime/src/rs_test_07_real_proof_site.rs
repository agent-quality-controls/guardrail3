use std::collections::BTreeSet;

use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::TestFunctionInput;
use super::parse::{TestFunctionInfo, UseBinding};

const ID: &str = "RS-TEST-07";

pub fn check(input: &TestFunctionInput<'_>, results: &mut Vec<CheckResult>) {
    if input.function.has_assertion_macro
        || function_has_owned_assertion_proof(
            input.function,
            &input.parsed.imports,
            &input.parsed.file_function_names,
            input.file.assertions_package_name.as_deref(),
            input.proof_bearing_assertion_functions,
        )
    {
        return;
    }

    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Warn,
        title: "test lacks real proof site".to_owned(),
        message: format!(
            "Test `{}` must contain an assertion macro or call into the owned assertions module/crate.",
            input.function.name
        ),
        file: Some(input.file.rel_path.clone()),
        line: Some(input.function.line),
        inventory: false,
    });
}

fn function_has_owned_assertion_proof(
    function: &TestFunctionInfo,
    imports: &[UseBinding],
    file_function_names: &BTreeSet<String>,
    assertions_package_name: Option<&str>,
    proof_bearing_assertion_functions: Option<&BTreeSet<String>>,
) -> bool {
    let Some(assertions_package_name) = assertions_package_name else {
        return false;
    };
    let Some(proof_bearing_assertion_functions) = proof_bearing_assertion_functions else {
        return false;
    };
    let mut call_roots = BTreeSet::from([assertions_package_name.to_owned()]);
    let mut bare_call_idents = BTreeSet::new();
    let mut has_assertions_glob = false;

    for binding in imports {
        if binding
            .path_segments
            .first()
            .is_some_and(|segment| segment == assertions_package_name)
        {
            if let Some(local_name) = binding.local_name.as_ref() {
                let _ = call_roots.insert(local_name.clone());
                let _ = bare_call_idents.insert(local_name.clone());
            } else {
                has_assertions_glob = true;
            }
        }
    }

    let bare_call_is_owned = |name: &str| {
        !function.shadowed_idents.contains(name)
            && !file_function_names.contains(name)
            && proof_bearing_assertion_functions.contains(name)
            && (bare_call_idents.contains(name) || has_assertions_glob)
    };

    function.call_paths.iter().any(|path| match path.first() {
        Some(first) if path.len() == 1 => bare_call_is_owned(first),
        Some(first) => {
            call_roots.contains(first)
                && path
                    .last()
                    .is_some_and(|name| proof_bearing_assertion_functions.contains(name))
        }
        None => false,
    }) || function
        .method_receiver_paths
        .iter()
        .any(|path| match path.first() {
            Some(first) if path.len() == 1 => bare_call_is_owned(first),
            Some(first) => {
                call_roots.contains(first)
                    && path
                        .last()
                        .is_some_and(|name| proof_bearing_assertion_functions.contains(name))
            }
            None => false,
        })
}

#[cfg(test)]
#[path = "rs_test_07_real_proof_site_tests/mod.rs"]
mod rs_test_07_real_proof_site_tests;
