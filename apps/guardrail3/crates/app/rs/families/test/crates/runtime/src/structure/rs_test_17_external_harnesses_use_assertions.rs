use crate::{CheckResult, Severity};

use super::facts::TestFileKind;
use super::parse::FunctionInfo;
use super::inputs::TestFunctionInput;
use super::rs_test_07_real_proof_site::has_owned_assertion_proof;

const ID: &str = "RS-TEST-17";

pub fn check(input: &TestFunctionInput<'_>, results: &mut Vec<CheckResult>) {
    if !matches!(input.file.kind, TestFileKind::ExternalHarness) {
        return;
    }

    if input.function.has_assertion_macro || calls_local_assertion_helper(input) {
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

fn calls_local_assertion_helper(input: &TestFunctionInput<'_>) -> bool {
    let local_assertion_helpers = local_assertion_helper_names(
        &input.parsed.functions,
        &input.parsed.imports,
        &input.parsed.file_function_names,
        input.file.assertions_package_name.as_deref(),
        input.proof_bearing_assertion_functions,
    );

    input.function.call_paths.iter().any(|path| {
        if path.len() == 1 {
            let direct_local_helper = local_assertion_helpers.contains(path[0].as_str())
                && !input.function.shadowed_idents.contains(&path[0]);
            let aliased_local_helper = input
                .function
                .local_call_aliases
                .get(&path[0])
                .and_then(|alias| alias.last())
                .is_some_and(|name| local_assertion_helpers.contains(name.as_str()));
            return direct_local_helper || aliased_local_helper;
        }
        path.last()
            .is_some_and(|name| local_assertion_helpers.contains(name.as_str()))
            && path
                .first()
                .is_none_or(|first| !import_binds_name(&input.parsed.imports, first))
    }) || input
        .function
        .method_names
        .iter()
        .zip(input.function.method_receiver_paths.iter())
        .any(|(method, receiver)| {
            local_assertion_helpers.contains(method.as_str())
                && receiver
                    .first()
                    .is_some_and(|name| input.function.shadowed_idents.contains(name))
        })
}

fn local_assertion_helper_names<'a>(
    functions: &'a [FunctionInfo],
    imports: &[super::parse::UseBinding],
    file_function_names: &std::collections::BTreeSet<String>,
    assertions_package_name: Option<&str>,
    proof_bearing_assertion_functions: Option<&std::collections::BTreeSet<String>>,
) -> std::collections::BTreeSet<&'a str> {
    let mut assertion_helpers = functions
        .iter()
        .filter(|function| !function.is_test)
        .filter(|function| {
            function.has_assertion_macro
                || has_owned_assertion_proof(
                    &super::parse::TestFunctionInfo {
                        line: function.line,
                        name: function.name.clone(),
                        uses_tokio_test_attr: false,
                        has_assertion_macro: function.has_assertion_macro,
                        has_failure_enforcement: function.has_failure_enforcement,
                        call_paths: function.call_paths.clone(),
                        path_uses: function.path_uses.clone(),
                        method_receiver_paths: Vec::new(),
                        method_names: function.method_names.clone(),
                        local_call_aliases: function.local_call_aliases.clone(),
                        field_accesses: function.field_accesses.clone(),
                        shadowed_idents: function.shadowed_idents.clone(),
                        should_panic_line: None,
                        should_panic_has_expected: false,
                        tautological_assert_lines: Vec::new(),
                        weak_matches_lines: Vec::new(),
                    },
                    imports,
                    file_function_names,
                    assertions_package_name,
                    proof_bearing_assertion_functions,
                )
        })
        .map(|function| function.name.as_str())
        .collect::<std::collections::BTreeSet<_>>();

    loop {
        let mut changed = false;
        for function in functions.iter().filter(|function| !function.is_test) {
            if assertion_helpers.contains(function.name.as_str()) {
                continue;
            }
            if function.call_paths.iter().any(|path| {
                path.len() == 1
                    && assertion_helpers.contains(path[0].as_str())
                    && !function.shadowed_idents.contains(&path[0])
            }) {
                changed |= assertion_helpers.insert(function.name.as_str());
            }
        }
        if !changed {
            break;
        }
    }

    assertion_helpers
}

fn import_binds_name(imports: &[super::parse::UseBinding], name: &str) -> bool {
    imports.iter().any(|binding| {
        binding.local_name.as_deref() == Some(name)
            || (binding.local_name.is_none()
                && binding
                    .path_segments
                    .last()
                    .is_some_and(|segment| segment == name))
    })
}

#[cfg(test)]
pub(crate) fn run_family(root: &std::path::Path) -> Vec<CheckResult> {
    let tree = test_support::walk(root);
    super::check_test_tree(&tree, &test_support::StubToolChecker::default())
}
#[cfg(test)]
#[path = "rs_test_17_external_harnesses_use_assertions_tests/mod.rs"]
mod rs_test_17_external_harnesses_use_assertions_tests;
