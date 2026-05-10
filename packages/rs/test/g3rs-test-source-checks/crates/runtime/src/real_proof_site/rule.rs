#![expect(
    clippy::excessive_nesting,
    reason = "structural code pattern (parser/assertion helper) where lint conflicts with module architecture"
)]
#![expect(
    clippy::indexing_slicing,
    reason = "structural code pattern (parser/assertion helper) where lint conflicts with module architecture"
)]
#![expect(
    clippy::too_many_lines,
    reason = "structural code pattern (parser/assertion helper) where lint conflicts with module architecture"
)]
#![expect(
    clippy::type_complexity,
    reason = "structural code pattern (parser/assertion helper) where lint conflicts with module architecture"
)]
use std::collections::{BTreeMap, BTreeSet};

use g3rs_test_types::ast::{TestFunctionInfo, UseBinding};
use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::support::{TestFunctionInput, normalized_owned_assertion_relative_segments};

/// `ID` constant.
const ID: &str = "g3rs-test/real-proof-site";

/// `check` function.
pub(crate) fn check(input: &TestFunctionInput<'_>, results: &mut Vec<G3CheckResult>) {
    if input.function.assertions.has_assertion_macro
        || has_owned_assertion_proof(
            input.function,
            &input.file.parsed.imports,
            &input.file.parsed.file_function_names,
            input.file.assertions_package_name.as_deref(),
            input.proof_bearing_assertion_functions,
        )
    {
        results.push(
            G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Info,
                "test uses shared proof".to_owned(),
                format!(
                    "Test `{}` in `{}` ends with a real assertion or calls the shared assertions crate, so the test actually proves the behavior it exercises.",
                    input.function.name,
                    input.file.rel_path
                ),
                Some(input.file.rel_path.clone()),
                Some(input.function.line),
            )
            .into_inventory(),
        );
        return;
    }

    if let Some(local_path) = local_proof_path(
        input.function,
        &input.file.parsed.file_function_names,
        &input.file.parsed.imports,
        &input.file.local_proof_helper_functions,
    ) {
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "test checks results through local path".to_owned(),
            format!(
                "Test `{}` in `{}` checks results through local path `{}`. Move those result assertions into the shared assertions crate and call that from the test instead, so internal and external tests use the same proof.",
                input.function.name,
                input.file.rel_path,
                local_path,
            ),
            Some(input.file.rel_path.clone()),
            Some(input.function.line),
        ));
        return;
    }

    results.push(G3CheckResult::new(
        ID.to_owned(),
        G3Severity::Error,
        "test has no shared proof step".to_owned(),
        format!(
            "Test `{}` in `{}` does not call the shared assertions crate. Move the result assertions into the shared assertions crate and call that from the test, so internal and external tests use the same proof.",
            input.function.name,
            input.file.rel_path,
        ),
        Some(input.file.rel_path.clone()),
        Some(input.function.line),
    ));
}

/// `local_proof_path` function.
fn local_proof_path(
    function: &TestFunctionInfo,
    file_function_names: &BTreeSet<String>,
    imports: &[UseBinding],
    local_proof_helpers: &BTreeSet<String>,
) -> Option<String> {
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

    first_local_assertion_path(
        &function.body.call_paths,
        &function.body.local_call_aliases,
        &function.body.shadowed_idents,
        file_function_names,
        &local_imports,
        local_proof_helpers,
    )
    .or_else(|| {
        function
            .body
            .method_names
            .iter()
            .zip(function.harness.method_receiver_paths.iter())
            .find(|(method, receiver)| {
                local_proof_helpers.contains(method.as_str())
                    && receiver
                        .first()
                        .is_some_and(|name| function.body.shadowed_idents.contains(name))
            })
            .map(|(method, receiver)| format!("{}::{method}", receiver.join("::")))
    })
}

/// `first_local_assertion_path` function.
fn first_local_assertion_path(
    call_paths: &[Vec<String>],
    local_call_aliases: &BTreeMap<String, Vec<String>>,
    shadowed_idents: &BTreeSet<String>,
    file_function_names: &BTreeSet<String>,
    local_imports: &BTreeMap<String, String>,
    local_proof_helpers: &BTreeSet<String>,
) -> Option<String> {
    for path in call_paths {
        let Some(first) = path.first() else {
            continue;
        };
        if matches!(first.as_str(), "crate" | "self" | "super") {
            if path.last().is_some_and(|name| {
                local_proof_helpers.contains(name) || looks_like_proof_helper_name(name)
            }) {
                return Some(path.join("::"));
            }
            continue;
        }
        if let Some(import_path) = local_imports.get(first) {
            if path.len() == 1 {
                if local_proof_helpers.contains(first)
                    || import_path.rsplit("::").next().is_some_and(|name| {
                        local_proof_helpers.contains(name) || looks_like_proof_helper_name(name)
                    })
                {
                    return Some(import_path.clone());
                }
                continue;
            }
            if path[1..].last().is_some_and(|name| {
                local_proof_helpers.contains(name) || looks_like_proof_helper_name(name)
            }) {
                return Some(format!("{import_path}::{}", path[1..].join("::")));
            }
            continue;
        }
        if path.len() == 1
            && !shadowed_idents.contains(first)
            && file_function_names.contains(first)
            && local_proof_helpers.contains(first)
        {
            return Some(format!("local function `{first}`"));
        }
        if let Some(alias_path) = local_call_aliases.get(first) {
            if alias_path.last().is_some_and(|name| {
                local_proof_helpers.contains(name) || looks_like_proof_helper_name(name)
            }) {
                return Some(alias_path.join("::"));
            }
        }
    }
    None
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

/// `has_owned_assertion_proof` function.
pub(crate) fn has_owned_assertion_proof(
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

    loop {
        let mut changed = false;
        for binding in imports {
            let Some(relative_segments) = normalized_owned_assertion_relative_segments(
                binding,
                assertions_package_name,
                &root_prefixes,
            ) else {
                continue;
            };

            if let Some(local_name) = binding.local_name.as_ref() {
                changed |= root_prefixes
                    .insert(local_name.clone(), relative_segments.clone())
                    .as_ref()
                    .is_none_or(|existing| existing != &relative_segments);
                if !relative_segments.is_empty() {
                    let qualified = relative_segments.join("::");
                    changed |= bare_imports
                        .insert(local_name.clone(), qualified.clone())
                        .as_ref()
                        .is_none_or(|existing| existing != &qualified);
                }
            } else if let Some(last) = relative_segments.last().cloned() {
                changed |= root_prefixes
                    .insert(last.clone(), relative_segments.clone())
                    .as_ref()
                    .is_none_or(|existing| existing != &relative_segments);
                let qualified = relative_segments.join("::");
                changed |= bare_imports
                    .insert(last, qualified.clone())
                    .as_ref()
                    .is_none_or(|existing| existing != &qualified);
            } else if !glob_prefixes.contains(&relative_segments) {
                glob_prefixes.push(relative_segments);
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }

    let bare_call_is_owned = |name: &str| {
        if function.body.shadowed_idents.contains(name) {
            return function
                .body
                .local_call_aliases
                .get(name)
                .is_some_and(|path| {
                    path_is_owned(
                        path,
                        &root_prefixes,
                        &bare_imports,
                        &glob_prefixes,
                        proof_bearing_assertion_functions,
                    )
                });
        }
        !file_function_names.contains(name)
            && (bare_imports
                .get(name)
                .is_some_and(|qualified| proof_bearing_assertion_functions.contains(qualified))
                || glob_prefixes.iter().any(|prefix| {
                    proof_bearing_assertion_functions
                        .contains(&qualified_assertion_name(prefix, name))
                }))
    };

    function
        .body
        .call_paths
        .iter()
        .any(|path| match path.first() {
            Some(first) if path.len() == 1 => bare_call_is_owned(first),
            Some(_) => path_is_owned(
                path,
                &root_prefixes,
                &bare_imports,
                &glob_prefixes,
                proof_bearing_assertion_functions,
            ),
            None => false,
        })
        || function
            .harness
            .method_receiver_paths
            .iter()
            .any(|path| match path.first() {
                Some(first) if path.len() == 1 => bare_call_is_owned(first),
                Some(_) => path_is_owned(
                    path,
                    &root_prefixes,
                    &bare_imports,
                    &glob_prefixes,
                    proof_bearing_assertion_functions,
                ),
                None => false,
            })
}

/// `qualified_assertion_name` function.
fn qualified_assertion_name(module_prefix: &[String], tail: &str) -> String {
    if module_prefix.is_empty() {
        tail.to_owned()
    } else if tail.is_empty() {
        module_prefix.join("::")
    } else {
        format!("{}::{tail}", module_prefix.join("::"))
    }
}

/// `path_is_owned` function.
fn path_is_owned(
    path: &[String],
    root_prefixes: &BTreeMap<String, Vec<String>>,
    bare_imports: &BTreeMap<String, String>,
    glob_prefixes: &[Vec<String>],
    proof_bearing_assertion_functions: &BTreeSet<String>,
) -> bool {
    match path.first() {
        Some(first) if path.len() == 1 => {
            bare_imports
                .get(first)
                .is_some_and(|qualified| proof_bearing_assertion_functions.contains(qualified))
                || glob_prefixes.iter().any(|prefix| {
                    proof_bearing_assertion_functions
                        .contains(&qualified_assertion_name(prefix, first))
                })
        }
        Some(first) => root_prefixes.get(first).is_some_and(|prefix| {
            proof_bearing_assertion_functions
                .contains(&qualified_assertion_name(prefix, &path[1..].join("::")))
        }),
        None => false,
    }
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rule_tests;
