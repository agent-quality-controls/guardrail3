#![expect(
    clippy::indexing_slicing,
    reason = "structural code pattern (parser/assertion helper) where lint conflicts with module architecture"
)]
#![expect(
    clippy::type_complexity,
    reason = "structural code pattern (parser/assertion helper) where lint conflicts with module architecture"
)]
use std::collections::{BTreeMap, BTreeSet};

use g3rs_test_types::G3RsTestAnalyzedSourceFile;
use g3rs_test_types::ast::{FunctionInfo, UseBinding};

use super::pipeline::qualified_assertion_name;

/// `collect_local_proof_helper_functions` function.
pub(super) fn collect_local_proof_helper_functions(
    file: &G3RsTestAnalyzedSourceFile,
) -> BTreeSet<String> {
    let local_imports = collect_local_import_paths(&file.parsed.imports);
    let mut helpers = file
        .parsed
        .functions
        .iter()
        .filter(|function| !function.is_test)
        .filter(|function| {
            function_calls_owned_assertion_proof(
                function,
                &file.parsed.imports,
                &file.parsed.file_function_names,
                file.assertions_package_name.as_deref(),
                &file.proof_bearing_assertion_functions,
            ) || (function.assertions.has_assertion_macro
                && looks_like_proof_helper_name(&function.name))
        })
        .map(|function| function.name.clone())
        .collect::<BTreeSet<_>>();

    loop {
        let mut changed = false;
        for function in file
            .parsed
            .functions
            .iter()
            .filter(|function| !function.is_test)
        {
            if helpers.contains(&function.name) {
                continue;
            }
            if function_calls_local_proof_helper(
                function,
                &file.parsed.file_function_names,
                &local_imports,
                &helpers,
            ) {
                changed |= helpers.insert(function.name.clone());
            }
        }
        if !changed {
            break;
        }
    }

    helpers
}

/// `looks_like_proof_helper_name` function.
fn looks_like_proof_helper_name(name: &str) -> bool {
    name.split('_').any(|segment| {
        matches!(
            segment,
            "assert"
                | "asserts"
                | "assertion"
                | "assertions"
                | "expect"
                | "expected"
                | "prove"
                | "proof"
                | "require"
                | "verify"
                | "result"
                | "results"
                | "outcome"
                | "outcomes"
        )
    })
}

/// `collect_local_import_paths` function.
fn collect_local_import_paths(imports: &[UseBinding]) -> BTreeMap<String, String> {
    let mut local_imports = BTreeMap::new();
    for binding in imports {
        let Some(first) = binding.path_segments.first() else {
            continue;
        };
        if !matches!(first.as_str(), "crate" | "self" | "super") {
            continue;
        }
        let local_name = binding
            .local_name
            .clone()
            .or_else(|| binding.path_segments.last().cloned());
        if let Some(local_name) = local_name {
            let _ = local_imports.insert(local_name, binding.path_segments.join("::"));
        }
    }
    local_imports
}

/// `function_calls_local_proof_helper` function.
fn function_calls_local_proof_helper(
    function: &FunctionInfo,
    file_function_names: &BTreeSet<String>,
    local_imports: &BTreeMap<String, String>,
    local_proof_helpers: &BTreeSet<String>,
) -> bool {
    function.body.call_paths.iter().any(|path| {
        let Some(first) = path.first() else {
            return false;
        };
        if matches!(first.as_str(), "crate" | "self" | "super") {
            return path
                .last()
                .is_some_and(|name| local_proof_helpers.contains(name));
        }
        if let Some(import_path) = local_imports.get(first) {
            if path.len() == 1 {
                return local_proof_helpers.contains(first)
                    || import_path
                        .rsplit("::")
                        .next()
                        .is_some_and(|name| local_proof_helpers.contains(name));
            }
            return path[1..]
                .last()
                .is_some_and(|name| local_proof_helpers.contains(name));
        }
        if path.len() == 1
            && !function.body.shadowed_idents.contains(first)
            && file_function_names.contains(first)
        {
            return local_proof_helpers.contains(first);
        }
        function
            .body
            .local_call_aliases
            .get(first)
            .and_then(|alias_path| alias_path.last())
            .is_some_and(|name| local_proof_helpers.contains(name))
    })
}

/// `function_calls_owned_assertion_proof` function.
fn function_calls_owned_assertion_proof(
    function: &FunctionInfo,
    imports: &[UseBinding],
    file_function_names: &BTreeSet<String>,
    assertions_package_name: Option<&str>,
    proof_bearing_assertion_functions: &BTreeSet<String>,
) -> bool {
    let Some(assertions_package_name) = assertions_package_name else {
        return false;
    };
    if proof_bearing_assertion_functions.is_empty() {
        return false;
    }
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

    function
        .body
        .call_paths
        .iter()
        .any(|path| match path.first() {
            Some(first) if path.len() == 1 => {
                !file_function_names.contains(first)
                    && (bare_imports.get(first).is_some_and(|qualified| {
                        proof_bearing_assertion_functions.contains(qualified)
                    }) || glob_prefixes.iter().any(|prefix| {
                        proof_bearing_assertion_functions
                            .contains(&qualified_assertion_name(prefix, first))
                    }))
            }
            Some(_) => path_is_owned_assertion_call(
                path,
                &root_prefixes,
                proof_bearing_assertion_functions,
            ),
            None => false,
        })
}

/// `path_is_owned_assertion_call` function.
fn path_is_owned_assertion_call(
    path: &[String],
    root_prefixes: &BTreeMap<String, Vec<String>>,
    proof_bearing_assertion_functions: &BTreeSet<String>,
) -> bool {
    match path.first() {
        Some(first) if path.len() > 1 => root_prefixes.get(first).is_some_and(|prefix| {
            proof_bearing_assertion_functions
                .contains(&qualified_assertion_name(prefix, &path[1..].join("::")))
        }),
        _ => false,
    }
}
