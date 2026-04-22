use g3rs_test_types::G3RsTestFileKind;
use g3rs_test_types::ast::{FunctionInfo, TestFunctionInfo, TestHarnessFacts, UseBinding};
use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::rs_test_07_real_proof_site::has_owned_assertion_proof;
use crate::support::TestFunctionInput;

const ID: &str = "RS-TEST-SOURCE-17";

pub(crate) fn check(input: &TestFunctionInput<'_>, results: &mut Vec<G3CheckResult>) {
    if !matches!(input.file.kind, G3RsTestFileKind::ExternalHarness) {
        return;
    }

    if input.function.assertions.has_assertion_macro || calls_local_assertion_helper(input) {
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

    if has_owned_assertion_proof(
        input.function,
        &input.file.parsed.imports,
        &input.file.parsed.file_function_names,
        input.file.assertions_package_name.as_deref(),
        input.proof_bearing_assertion_functions,
    ) || qualified_owned_assertion_call(
        &input.function.body.call_paths,
        input.file.assertions_package_name.as_deref(),
        input.proof_bearing_assertion_functions,
    ) {
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

fn calls_local_assertion_helper(input: &TestFunctionInput<'_>) -> bool {
    let local_assertion_helpers = local_assertion_helper_names(
        &input.file.parsed.functions,
        &input.file.parsed.imports,
        &input.file.parsed.file_function_names,
        input.file.assertions_package_name.as_deref(),
        input.proof_bearing_assertion_functions,
    );
    let imported_local_helpers = imported_local_helper_names(&input.file.parsed.imports);

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
            return direct_local_helper || aliased_local_helper || imported_local_helper;
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
        let Some(local_name) = binding.local_name.as_ref().or_else(|| binding.path_segments.last())
        else {
            continue;
        };
        let _ = imported_local_helpers.insert(local_name.clone(), binding.path_segments.clone());
    }

    imported_local_helpers
}

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
