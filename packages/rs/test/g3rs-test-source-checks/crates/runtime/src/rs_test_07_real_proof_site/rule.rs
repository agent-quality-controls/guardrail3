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
        &input.parsed.functions,
        &input.parsed.imports,
        &input.parsed.file_function_names,
        input.file.assertions_package_name.as_deref(),
        input.proof_bearing_assertion_functions,
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
    functions: &[crate::parse::FunctionInfo],
    imports: &[UseBinding],
    file_function_names: &BTreeSet<String>,
    assertions_package_name: Option<&str>,
    proof_bearing_assertion_functions: Option<&BTreeSet<String>>,
) -> Option<String> {
    let local_assertion_helpers = local_assertion_helper_names(
        functions,
        imports,
        file_function_names,
        assertions_package_name,
        proof_bearing_assertion_functions,
    );
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
        &function.call_paths,
        &function.local_call_aliases,
        &function.shadowed_idents,
        file_function_names,
        &local_imports,
        &local_assertion_helpers,
    )
    .or_else(|| {
        function
            .method_names
            .iter()
            .zip(function.method_receiver_paths.iter())
            .find(|(method, receiver)| {
                local_assertion_helpers.contains(method.as_str())
                    && receiver
                        .first()
                        .is_some_and(|name| function.shadowed_idents.contains(name))
            })
            .map(|(method, receiver)| format!("{}::{method}", receiver.join("::")))
    })
}

fn first_local_assertion_path(
    call_paths: &[Vec<String>],
    local_call_aliases: &BTreeMap<String, Vec<String>>,
    shadowed_idents: &BTreeSet<String>,
    file_function_names: &BTreeSet<String>,
    local_imports: &BTreeMap<String, String>,
    local_assertion_helpers: &BTreeSet<&str>,
) -> Option<String> {
    for path in call_paths {
        let Some(first) = path.first() else {
            continue;
        };
        if matches!(first.as_str(), "crate" | "self" | "super") {
            if path_uses_local_assertions_module(path)
                || path
                    .last()
                    .is_some_and(|name| local_assertion_helpers.contains(name.as_str()))
            {
                return Some(path.join("::"));
            }
            continue;
        }
        if let Some(import_path) = local_imports.get(first) {
            if path.len() == 1 {
                if local_assertion_helpers.contains(first.as_str())
                    || import_path
                        .split("::")
                        .any(|segment| segment == "assertions")
                {
                    return Some(import_path.clone());
                }
                continue;
            }
            if import_path
                .split("::")
                .any(|segment| segment == "assertions")
                || path[1..]
                    .last()
                    .is_some_and(|name| local_assertion_helpers.contains(name.as_str()))
            {
                return Some(format!("{import_path}::{}", path[1..].join("::")));
            }
            continue;
        }
        if path.len() == 1
            && !shadowed_idents.contains(first)
            && file_function_names.contains(first)
            && local_assertion_helpers.contains(first.as_str())
        {
            return Some(format!("local function `{first}`"));
        }
        if let Some(alias_path) = local_call_aliases.get(first) {
            if path_uses_local_assertions_module(alias_path)
                || alias_path
                    .last()
                    .is_some_and(|name| local_assertion_helpers.contains(name.as_str()))
            {
                return Some(alias_path.join("::"));
            }
        }
    }
    None
}

fn path_uses_local_assertions_module(path: &[String]) -> bool {
    path.iter().any(|segment| segment == "assertions")
}

fn local_assertion_helper_names<'a>(
    functions: &'a [crate::parse::FunctionInfo],
    imports: &[UseBinding],
    file_function_names: &BTreeSet<String>,
    assertions_package_name: Option<&str>,
    proof_bearing_assertion_functions: Option<&BTreeSet<String>>,
) -> BTreeSet<&'a str> {
    let mut assertion_helpers = functions
        .iter()
        .filter(|function| !function.is_test)
        .filter(|function| {
            function.has_assertion_macro
                || has_owned_assertion_proof(
                    &crate::parse::TestFunctionInfo {
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
        .collect::<BTreeSet<_>>();

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
