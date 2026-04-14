use std::collections::{BTreeMap, BTreeSet};

use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::support::TestFunctionInput;
use crate::parse::{TestFunctionInfo, UseBinding};

const ID: &str = "RS-TEST-SOURCE-07";

pub(crate) fn check(input: &TestFunctionInput<'_>, results: &mut Vec<G3CheckResult>) {
    if input.function.has_assertion_macro
        || has_owned_assertion_proof(
            input.function,
            &input.parsed.imports,
            &input.parsed.file_function_names,
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
        &input.parsed.imports,
        &input.parsed.file_function_names,
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

fn local_proof_path(
    function: &TestFunctionInfo,
    imports: &[UseBinding],
    file_function_names: &BTreeSet<String>,
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

    first_local_path(
        &function.call_paths,
        &function.local_call_aliases,
        &function.shadowed_idents,
        file_function_names,
        &local_imports,
    )
    .or_else(|| {
        first_local_path(
            &function.method_receiver_paths,
            &function.local_call_aliases,
            &function.shadowed_idents,
            file_function_names,
            &local_imports,
        )
    })
}

fn first_local_path(
    call_paths: &[Vec<String>],
    local_call_aliases: &BTreeMap<String, Vec<String>>,
    shadowed_idents: &BTreeSet<String>,
    file_function_names: &BTreeSet<String>,
    local_imports: &BTreeMap<String, String>,
) -> Option<String> {
    for path in call_paths {
        let Some(first) = path.first() else {
            continue;
        };
        if matches!(first.as_str(), "crate" | "self" | "super") {
            return Some(path.join("::"));
        }
        if let Some(import_path) = local_imports.get(first) {
            if path.len() == 1 {
                return Some(import_path.clone());
            }
            return Some(format!("{import_path}::{}", path[1..].join("::")));
        }
        if path.len() == 1 && !shadowed_idents.contains(first) && file_function_names.contains(first) {
            return Some(format!("local function `{first}`"));
        }
        if let Some(alias_path) = local_call_aliases.get(first) {
            if alias_path
                .first()
                .is_some_and(|segment| matches!(segment.as_str(), "crate" | "self" | "super"))
            {
                return Some(alias_path.join("::"));
            }
        }
    }
    None
}

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
        if function.shadowed_idents.contains(name) {
            return function.local_call_aliases.get(name).is_some_and(|path| {
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

    function.call_paths.iter().any(|path| match path.first() {
        Some(first) if path.len() == 1 => bare_call_is_owned(first),
        Some(_) => path_is_owned(
            path,
            &root_prefixes,
            &bare_imports,
            &glob_prefixes,
            proof_bearing_assertion_functions,
        ),
        None => false,
    }) || function
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

fn qualified_assertion_name(module_prefix: &[String], tail: &str) -> String {
    if module_prefix.is_empty() {
        tail.to_owned()
    } else if tail.is_empty() {
        module_prefix.join("::")
    } else {
        format!("{}::{tail}", module_prefix.join("::"))
    }
}

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
