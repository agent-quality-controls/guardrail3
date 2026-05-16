#![expect(
    clippy::indexing_slicing,
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
use std::collections::BTreeMap;

use g3rs_test_types::G3RsTestFileKind;
use g3rs_test_types::ast::{FieldAccessInfo, FunctionInfo, UseBinding};
use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::support::{AssertionsModuleInput, TestFunctionInput};

/// `ID` constant.
const ID: &str = "g3rs-test/assertions-modules-prove";
/// `REPORT_FIELDS` constant.
const REPORT_FIELDS: &[&str] = &[
    "file",
    "id",
    "inventory",
    "line",
    "message",
    "severity",
    "title",
];
/// `REPORT_METHODS` constant.
const REPORT_METHODS: &[&str] = &[
    "file",
    "id",
    "inventory",
    "line",
    "message",
    "severity",
    "title",
];

/// `check` function.
pub(crate) fn check(input: &AssertionsModuleInput<'_>, results: &mut Vec<G3CheckResult>) {
    let first_exported_function = input
        .file
        .parsed
        .functions
        .iter()
        .find(|function| function.is_public && !function.is_test);
    let Some(first_exported_function) = first_exported_function else {
        return;
    };
    if !input.proof_bearing_exported_functions.is_empty() {
        results.push(
            G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Info,
                "assertions module proves runtime".to_owned(),
                "Assertions modules expose public helpers that ultimately prove through real assertions.".to_owned(),
                Some(input.file.rel_path.clone()),
                Some(first_exported_function.line),
            )
            .into_inventory(),
        );
        return;
    }

    results.push(G3CheckResult::new(
    ID.to_owned(),
    G3Severity::Error,
    "assertions module lacks proof-bearing export".to_owned(),
    "The assertions crate exports public functions but none of them access CheckResult fields or call assertion macros. At least one exported function must verify CheckResult output.".to_owned(),
    Some(input.file.rel_path.clone()),
    Some(first_exported_function.line),
    ));
}

/// `check_sidecar_semantic_proof` function.
pub(crate) fn check_sidecar_semantic_proof(
    input: &TestFunctionInput<'_>,
    results: &mut Vec<G3CheckResult>,
) {
    if !matches!(input.file.kind, G3RsTestFileKind::InternalSidecarSupport)
        && !matches!(input.file.kind, G3RsTestFileKind::InternalSidecarMod)
    {
        return;
    }
    if input.file.assertions_package_name.is_none() {
        return;
    }
    if !owns_sidecar_semantic_proof(input) {
        if !input.function.assertions.has_failure_enforcement {
            return;
        }
        results.push(
            G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Info,
                "sidecar delegates semantic proof".to_owned(),
                "Internal sidecars keep setup local and delegate result-shape assertions to the owned sibling assertions module.".to_owned(),
                Some(input.file.rel_path.clone()),
                Some(input.function.line),
            )
            .into_inventory(),
        );
        return;
    }

    results.push(G3CheckResult::new(
    ID.to_owned(),
    G3Severity::Error,
    "sidecar owns semantic result assertion".to_owned(),
    format!("Test function `{}` in sidecar directly accesses CheckResult fields (.id(), .severity(), .title(), etc.). Move these result-shape assertions into the sibling assertions crate.", input.function.name),
    Some(input.file.rel_path.clone()),
    Some(input.function.line),
    ));
}

/// `owns_sidecar_semantic_proof` function.
fn owns_sidecar_semantic_proof(input: &TestFunctionInput<'_>) -> bool {
    let local_semantic_helpers =
        local_semantic_helper_names(&input.file.parsed.functions, &input.file.parsed.imports);
    let imported_local_helpers = imported_local_helper_names(&input.file.parsed.imports);

    owns_result_shape_assertion(
        &input.function.body.field_accesses,
        &input.function.body.method_names,
        &input.function.body.path_uses,
    ) || input.function.body.call_paths.iter().any(|path| {
        if path.len() == 1 {
            let direct_local_helper = local_semantic_helpers.contains(path[0].as_str())
                && !input.function.body.shadowed_idents.contains(&path[0]);
            let local_alias_helper = input
                .function
                .body
                .local_call_aliases
                .get(&path[0])
                .and_then(|alias| alias.last())
                .is_some_and(|name| local_semantic_helpers.contains(name.as_str()));
            let imported_local_helper = !input.function.body.shadowed_idents.contains(&path[0])
                && import_alias_targets_local_helper(
                    &path[0],
                    &local_semantic_helpers,
                    &imported_local_helpers,
                );
            return direct_local_helper || local_alias_helper || imported_local_helper;
        }

        matches!(
            path.first().map(String::as_str),
            Some("crate" | "self" | "super")
        ) && path
            .last()
            .is_some_and(|name| local_semantic_helpers.contains(name.as_str()))
    })
}

/// `local_semantic_helper_names` function.
fn local_semantic_helper_names<'a>(
    functions: &'a [FunctionInfo],
    imports: &[UseBinding],
) -> std::collections::BTreeSet<&'a str> {
    let imported_local_helpers = imported_local_helper_names(imports);
    let mut semantic_helpers = functions
        .iter()
        .filter(|function| !function.is_test)
        .filter(|function| {
            owns_result_shape_assertion(
                &function.body.field_accesses,
                &function.body.method_names,
                &function.body.path_uses,
            )
        })
        .map(|function| function.name.as_str())
        .collect::<std::collections::BTreeSet<_>>();

    loop {
        let mut changed = false;
        for function in functions.iter().filter(|function| !function.is_test) {
            if semantic_helpers.contains(function.name.as_str()) {
                continue;
            }
            if function.body.call_paths.iter().any(|path| {
                (path.len() == 1
                    && semantic_helpers.contains(path[0].as_str())
                    && !function.body.shadowed_idents.contains(&path[0]))
                    || (path.len() == 1
                        && function
                            .body
                            .local_call_aliases
                            .get(&path[0])
                            .and_then(|alias| alias.last())
                            .is_some_and(|name| semantic_helpers.contains(name.as_str())))
                    || (path.len() == 1
                        && !function.body.shadowed_idents.contains(&path[0])
                        && import_alias_targets_local_helper(
                            &path[0],
                            &semantic_helpers,
                            &imported_local_helpers,
                        ))
                    || (path.len() > 1
                        && matches!(
                            path.first().map(String::as_str),
                            Some("crate" | "self" | "super")
                        )
                        && path
                            .last()
                            .is_some_and(|name| semantic_helpers.contains(name.as_str())))
            }) {
                changed |= semantic_helpers.insert(function.name.as_str());
            }
        }
        if !changed {
            break;
        }
    }

    semantic_helpers
}

/// `imported_local_helper_names` function.
fn imported_local_helper_names(imports: &[UseBinding]) -> BTreeMap<String, Vec<String>> {
    let mut imported_local_helpers = BTreeMap::new();

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
    imported_local_helpers: &BTreeMap<String, Vec<String>>,
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

/// `owns_result_shape_assertion` function.
fn owns_result_shape_assertion(
    field_accesses: &[FieldAccessInfo],
    method_names: &[String],
    path_uses: &[Vec<String>],
) -> bool {
    field_accesses
        .iter()
        .any(|field| REPORT_FIELDS.contains(&field.name.as_str()))
        || method_names
            .iter()
            .any(|method| REPORT_METHODS.contains(&method.as_str()))
        || path_uses.iter().any(|path| {
            path.last()
                .is_some_and(|segment| matches!(segment.as_str(), "CheckResult" | "Severity"))
        })
}
