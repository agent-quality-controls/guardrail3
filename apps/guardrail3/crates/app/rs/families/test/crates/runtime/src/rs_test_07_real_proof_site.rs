use std::collections::{BTreeMap, BTreeSet};

use crate::{CheckResult, Severity};

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
    let mut root_prefixes = BTreeMap::from([(assertions_package_name.to_owned(), Vec::new())]);
    let mut bare_imports = BTreeMap::new();
    let mut glob_prefixes = Vec::new();

    for binding in imports {
        if binding
            .path_segments
            .first()
            .is_some_and(|segment| segment == assertions_package_name)
        {
            let relative_segments = binding.path_segments[1..].to_vec();
            if let Some(local_name) = binding.local_name.as_ref() {
                let _ = root_prefixes.insert(local_name.clone(), relative_segments.clone());
                let _ = bare_imports.insert(local_name.clone(), relative_segments.join("::"));
            } else if let Some(last) = relative_segments.last().cloned() {
                let _ = root_prefixes.insert(last.clone(), relative_segments.clone());
                let _ = bare_imports.insert(last, relative_segments.join("::"));
            } else {
                glob_prefixes.push(relative_segments);
            }
        }
    }

    let bare_call_is_owned = |name: &str| {
        !function.shadowed_idents.contains(name)
            && !file_function_names.contains(name)
            && (bare_imports
                .get(name)
                .is_some_and(|qualified| proof_bearing_assertion_functions.contains(qualified))
                || glob_prefixes.iter().any(|prefix| {
                    proof_bearing_assertion_functions
                        .contains(&qualified_assertion_name(prefix, name))
                }))
    };

    function.call_paths.iter().any(|path| match path.first() {
        Some(first) if path.len() == 1 => bare_call_is_owned(first),
        Some(first) => root_prefixes.get(first).is_some_and(|prefix| {
            proof_bearing_assertion_functions
                .contains(&qualified_assertion_name(prefix, &path[1..].join("::")))
        }),
        None => false,
    }) || function
        .method_receiver_paths
        .iter()
        .any(|path| match path.first() {
            Some(first) if path.len() == 1 => bare_call_is_owned(first),
            Some(first) => root_prefixes.get(first).is_some_and(|prefix| {
                proof_bearing_assertion_functions
                    .contains(&qualified_assertion_name(prefix, &path[1..].join("::")))
            }),
            None => false,
        })
}

fn qualified_assertion_name(module_prefix: &[String], tail: &str) -> String {
    if module_prefix.is_empty() {
        tail.to_owned()
    } else if tail.is_empty() {
        module_prefix.join("::")
    } else {
        format!("{}::{tail}", module_prefix.join("::"))
    }
}

#[cfg(test)]
#[allow(dead_code)]
pub(crate) fn run_family(root: &std::path::Path) -> Vec<CheckResult> {
    let tree = test_support::walk(root);
    super::check_test_tree(&tree, &test_support::StubToolChecker::default())
}

#[cfg(test)]
#[allow(dead_code)]
#[allow(dead_code)]
pub(crate) fn run_family_with_tool(
    root: &std::path::Path,
    cargo_mutants_installed: bool,
) -> Vec<CheckResult> {
    let tree = test_support::walk(root);
    let checker = if cargo_mutants_installed {
        test_support::StubToolChecker::with_tools(["cargo-mutants"])
    } else {
        test_support::StubToolChecker::default()
    };
    super::check_test_tree(&tree, &checker)
}

#[cfg(test)]
#[path = "rs_test_07_real_proof_site_tests/mod.rs"]
mod rs_test_07_real_proof_site_tests;
