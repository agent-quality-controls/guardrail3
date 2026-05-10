#![expect(
    clippy::indexing_slicing,
    reason = "structural code pattern (parser/assertion helper) where lint conflicts with module architecture"
)]
#![expect(
    clippy::needless_pass_by_value,
    reason = "structural code pattern (parser/assertion helper) where lint conflicts with module architecture"
)]
#![expect(
    clippy::shadow_unrelated,
    reason = "structural code pattern (parser/assertion helper) where lint conflicts with module architecture"
)]
#![expect(
    clippy::type_complexity,
    reason = "structural code pattern (parser/assertion helper) where lint conflicts with module architecture"
)]
use g3rs_test_types::G3RsTestFileKind;
use g3rs_test_types::ast::{FunctionInfo, TestFunctionInfo, TestHarnessFacts, UseBinding};
use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::real_proof_site::has_owned_assertion_proof;
use crate::support::{TestFunctionInput, normalized_owned_assertion_relative_segments};

/// `ID` constant.
const ID: &str = "g3rs-test/external-harnesses-use-assertions";

/// `check` function.
pub(crate) fn check(input: &TestFunctionInput<'_>, results: &mut Vec<G3CheckResult>) {
    if !matches!(input.file.kind, G3RsTestFileKind::ExternalHarness) {
        return;
    }

    let owned_assertion_aliases = owned_assertion_alias_names(
        &input.file.parsed.imports,
        input.file.assertions_package_name.as_deref(),
        input.proof_bearing_assertion_functions,
    );
    let uses_owned_assertion_alias = input
        .function
        .body
        .call_paths
        .iter()
        .any(|path| path_uses_owned_assertion_alias(path, &owned_assertion_aliases));
    let uses_owned_assertions = uses_owned_assertion_alias
        || has_owned_assertion_proof(
            input.function,
            &input.file.parsed.imports,
            &input.file.parsed.file_function_names,
            input.file.assertions_package_name.as_deref(),
            input.proof_bearing_assertion_functions,
        )
        || qualified_owned_assertion_call(
            &input.function.body.call_paths,
            input.file.assertions_package_name.as_deref(),
            input.proof_bearing_assertion_functions,
        );

    if input.function.assertions.has_assertion_macro
        || (calls_local_assertion_helper(input) && !uses_owned_assertions)
    {
        results.push(G3CheckResult::new(
        ID.to_owned(),
        G3Severity::Error,
        "external harness asserts directly".to_owned(),
        format!("Test function `{}` in `{}` uses assertion macros directly. External harnesses in `runtime/tests/` must not assert directly — call functions from the sibling assertions crate instead.", input.function.name, input.file.rel_path),
        Some(input.file.rel_path.clone()),
        Some(input.function.line),
            ));
        return;
    }

    if uses_owned_assertions {
        results.push(
            G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Info,
                "external harness uses owned assertions".to_owned(),
                "External harnesses stay black-box and prove through the owned assertions crate rather than direct assertion macros.".to_owned(),
                Some(input.file.rel_path.clone()),
                Some(input.function.line),
            )
            .into_inventory(),
        );
    }
}

/// `calls_local_assertion_helper` function.
fn calls_local_assertion_helper(input: &TestFunctionInput<'_>) -> bool {
    let local_assertion_helpers = local_assertion_helper_names(
        &input.file.parsed.functions,
        &input.file.parsed.imports,
        &input.file.parsed.file_function_names,
        input.file.assertions_package_name.as_deref(),
        input.proof_bearing_assertion_functions,
    );
    let imported_local_helpers = imported_local_helper_names(&input.file.parsed.imports);
    let owned_assertion_aliases = owned_assertion_alias_names(
        &input.file.parsed.imports,
        input.file.assertions_package_name.as_deref(),
        input.proof_bearing_assertion_functions,
    );

    input.function.body.call_paths.iter().any(|path| {
        if path.len() == 1 {
            let direct_local_helper = local_assertion_helpers.contains(path[0].as_str())
                && !input.function.body.shadowed_idents.contains(&path[0]);
            let aliased_local_helper = input
                .function
                .body
                .local_call_aliases
                .get(&path[0])
                .and_then(|alias| alias.last())
                .is_some_and(|name| local_assertion_helpers.contains(name.as_str()));
            let imported_local_helper = !input.function.body.shadowed_idents.contains(&path[0])
                && import_alias_targets_local_helper(
                    &path[0],
                    &local_assertion_helpers,
                    &imported_local_helpers,
                );
            let owned_assertion_alias =
                path_uses_owned_assertion_alias(path, &owned_assertion_aliases);
            return (direct_local_helper || aliased_local_helper || imported_local_helper)
                && !owned_assertion_alias;
        }
        if path_is_qualified_owned_assertion_call(
            path,
            input.file.assertions_package_name.as_deref(),
            input.proof_bearing_assertion_functions,
        ) {
            return false;
        }
        path.last()
            .is_some_and(|name| local_assertion_helpers.contains(name.as_str()))
            && path
                .first()
                .is_none_or(|first| !import_binds_name(&input.file.parsed.imports, first))
    }) || input
        .function
        .body
        .method_names
        .iter()
        .zip(input.function.harness.method_receiver_paths.iter())
        .any(|(method, receiver)| {
            local_assertion_helpers.contains(method.as_str())
                && receiver
                    .first()
                    .is_some_and(|name| input.function.body.shadowed_idents.contains(name))
        })
}

/// `local_assertion_helper_names` function.
fn local_assertion_helper_names<'a>(
    functions: &'a [FunctionInfo],
    imports: &[UseBinding],
    file_function_names: &std::collections::BTreeSet<String>,
    assertions_package_name: Option<&str>,
    proof_bearing_assertion_functions: Option<&std::collections::BTreeSet<String>>,
) -> std::collections::BTreeSet<&'a str> {
    let imported_local_helpers = imported_local_helper_names(imports);
    let mut assertion_helpers = functions
        .iter()
        .filter(|function| !function.is_test)
        .filter(|function| {
            function.assertions.has_assertion_macro
                || has_owned_assertion_proof(
                    &TestFunctionInfo {
                        line: function.line,
                        name: function.name.clone(),
                        assertions: function.assertions.clone(),
                        body: function.body.clone(),
                        harness: TestHarnessFacts::default(),
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
            if function.body.call_paths.iter().any(|path| {
                (path.len() == 1
                    && assertion_helpers.contains(path[0].as_str())
                    && !function.body.shadowed_idents.contains(&path[0]))
                    || (path.len() == 1
                        && !function.body.shadowed_idents.contains(&path[0])
                        && import_alias_targets_local_helper(
                            &path[0],
                            &assertion_helpers,
                            &imported_local_helpers,
                        ))
                    || (path.len() > 1
                        && matches!(
                            path.first().map(String::as_str),
                            Some("crate" | "self" | "super")
                        )
                        && path
                            .last()
                            .is_some_and(|name| assertion_helpers.contains(name.as_str())))
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

/// `owned_assertion_alias_names` function.
fn owned_assertion_alias_names(
    imports: &[UseBinding],
    assertions_package_name: Option<&str>,
    proof_bearing_assertion_functions: Option<&std::collections::BTreeSet<String>>,
) -> std::collections::BTreeMap<String, String> {
    let Some(assertions_package_name) = assertions_package_name else {
        return std::collections::BTreeMap::new();
    };
    let Some(proof_bearing_assertion_functions) = proof_bearing_assertion_functions else {
        return std::collections::BTreeMap::new();
    };

    let mut owned_assertion_aliases = std::collections::BTreeMap::new();
    let mut root_prefixes =
        std::collections::BTreeMap::from([(assertions_package_name.to_owned(), Vec::new())]);

    loop {
        let mut changed = false;
        for binding in imports {
            let Some(local_name) = binding
                .local_name
                .as_ref()
                .or_else(|| binding.path_segments.last())
            else {
                continue;
            };
            let Some(relative_segments) = normalized_owned_assertion_relative_segments(
                binding,
                assertions_package_name,
                &root_prefixes,
            ) else {
                continue;
            };

            changed |= root_prefixes
                .insert(local_name.clone(), relative_segments.clone())
                .as_ref()
                .is_none_or(|existing| existing != &relative_segments);
            let qualified = relative_segments.join("::");
            if !qualified.is_empty() && proof_bearing_assertion_functions.contains(&qualified) {
                changed |= insert_owned_assertion_alias(
                    local_name.clone(),
                    qualified,
                    &mut owned_assertion_aliases,
                );
            }
        }
        if !changed {
            break;
        }
    }

    owned_assertion_aliases
}

/// `insert_owned_assertion_alias` function.
fn insert_owned_assertion_alias(
    local_name: String,
    qualified: String,
    owned_assertion_aliases: &mut std::collections::BTreeMap<String, String>,
) -> bool {
    owned_assertion_aliases
        .insert(local_name, qualified.clone())
        .is_none_or(|existing| existing != qualified)
}

/// `path_uses_owned_assertion_alias` function.
fn path_uses_owned_assertion_alias(
    path: &[String],
    owned_assertion_aliases: &std::collections::BTreeMap<String, String>,
) -> bool {
    match path {
        [name] => owned_assertion_aliases.contains_key(name),
        [first, name] if matches!(first.as_str(), "crate" | "self" | "super") => {
            owned_assertion_aliases.contains_key(name)
        }
        _ => false,
    }
}

/// `import_binds_name` function.
fn import_binds_name(imports: &[UseBinding], name: &str) -> bool {
    imports.iter().any(|binding| {
        binding.local_name.as_deref() == Some(name)
            || (binding.local_name.is_none()
                && binding
                    .path_segments
                    .last()
                    .is_some_and(|segment| segment == name))
    })
}

/// `imported_local_helper_names` function.
fn imported_local_helper_names(
    imports: &[UseBinding],
) -> std::collections::BTreeMap<String, Vec<String>> {
    let mut imported_local_helpers = std::collections::BTreeMap::new();

    for binding in imports {
        let Some(first) = binding.path_segments.first() else {
            continue;
        };
        if !matches!(first.as_str(), "crate" | "self" | "super") {
            continue;
        }
        let Some(local_name) = binding
            .local_name
            .as_ref()
            .or_else(|| binding.path_segments.last())
        else {
            continue;
        };
        let _ = imported_local_helpers.insert(local_name.clone(), binding.path_segments.clone());
    }

    imported_local_helpers
}

/// `import_alias_targets_local_helper` function.
fn import_alias_targets_local_helper(
    name: &str,
    local_helpers: &std::collections::BTreeSet<&str>,
    imported_local_helpers: &std::collections::BTreeMap<String, Vec<String>>,
) -> bool {
    let mut current = name;
    let mut seen = std::collections::BTreeSet::new();

    loop {
        if local_helpers.contains(current) {
            return true;
        }
        if !seen.insert(current.to_owned()) {
            return false;
        }
        let Some(target) = imported_local_helpers.get(current) else {
            return false;
        };
        if target
            .first()
            .is_some_and(|segment| matches!(segment.as_str(), "crate" | "self" | "super"))
        {
            return target
                .last()
                .is_some_and(|name| local_helpers.contains(name.as_str()));
        }
        let Some(next) = target.last() else {
            return false;
        };
        current = next;
    }
}

/// `qualified_owned_assertion_call` function.
fn qualified_owned_assertion_call(
    call_paths: &[Vec<String>],
    assertions_package_name: Option<&str>,
    proof_bearing_assertion_functions: Option<&std::collections::BTreeSet<String>>,
) -> bool {
    call_paths.iter().any(|path| {
        path_is_qualified_owned_assertion_call(
            path,
            assertions_package_name,
            proof_bearing_assertion_functions,
        )
    })
}

/// `path_is_qualified_owned_assertion_call` function.
fn path_is_qualified_owned_assertion_call(
    path: &[String],
    assertions_package_name: Option<&str>,
    proof_bearing_assertion_functions: Option<&std::collections::BTreeSet<String>>,
) -> bool {
    let Some(assertions_package_name) = assertions_package_name else {
        return false;
    };
    let Some(proof_bearing_assertion_functions) = proof_bearing_assertion_functions else {
        return false;
    };
    let package_root = assertions_package_name.replace('-', "_");

    let Some(first) = path.first() else {
        return false;
    };
    if first != assertions_package_name && first != &package_root {
        return false;
    }
    proof_bearing_assertion_functions.contains(&path[1..].join("::"))
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rule_tests;
