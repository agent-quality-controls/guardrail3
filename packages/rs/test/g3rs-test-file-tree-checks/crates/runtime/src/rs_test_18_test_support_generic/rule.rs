use std::collections::BTreeSet;

use crate::support::TestSupportFileInput;
use g3rs_test_types::ast::{FunctionInfo, PublicValueKind, ReturnKind};
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "RS-TEST-FILETREE-18";
const REPORT_FIELDS: &[&str] = &[
    "file",
    "id",
    "inventory",
    "line",
    "message",
    "severity",
    "title",
];
const REPORT_METHODS: &[&str] = &[
    "file",
    "id",
    "inventory",
    "line",
    "message",
    "severity",
    "title",
];

#[cfg(test)]
#[path = "rule_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rule_tests;

fn path_mentions_route_construction(path: &[String]) -> bool {
    path.iter().any(|segment| {
        matches!(
            segment.as_str(),
            "FamilyMapper" | "guardrail3_app_rs_placement"
        )
    })
}

pub(crate) fn check(input: &TestSupportFileInput<'_>, results: &mut Vec<G3CheckResult>) {
    let mut reported = false;
    let disallowed_packages = input
        .local_runtime_packages
        .iter()
        .chain(input.local_assertions_packages.iter())
        .cloned()
        .collect::<BTreeSet<_>>();

    if !disallowed_packages.is_empty() {
        for binding in &input.file.parsed.imports {
            let Some(first) = binding.path_segments.first() else {
                continue;
            };
            if !disallowed_packages.contains(first) {
                continue;
            }
            results.push(G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Error,
                "test_support imports local component crate".to_owned(),
                format!(
                    "Shared `test_support` must stay generic and must not import local runtime/assertions crate `{first}`."
                ),
                Some(input.file.rel_path.clone()),
                Some(binding.line),
            ));
            reported = true;
        }
    }

    let mut reported_route_infra_imports = BTreeSet::new();
    for binding in &input.file.parsed.imports {
        if !path_mentions_route_construction(&binding.path_segments)
            || !reported_route_infra_imports.insert(binding.line)
        {
            continue;
        }
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "test_support imports route construction infrastructure".to_owned(),
            "Shared `test_support` must stay generic and must not import FamilyMapper or placement wiring. Keep test_support independent of how families are routed."
                .to_owned(),
            Some(input.file.rel_path.clone()),
            Some(binding.line),
        ));
        reported = true;
    }

    if !disallowed_packages.is_empty() {
        let mut called_packages = BTreeSet::new();
        for call_path in &input.file.parsed.file_call_paths {
            let Some(first) = call_path.first() else {
                continue;
            };
            if !disallowed_packages.contains(first) || !called_packages.insert(first.clone()) {
                continue;
            }
            results.push(G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Error,
                "test_support calls local component crate".to_owned(),
                format!(
                    "Shared `test_support` must stay generic and must not call local runtime/assertions crate `{first}` directly."
                ),
                Some(input.file.rel_path.clone()),
                None,
            ));
            reported = true;
        }
    }

    if input
        .file
        .parsed
        .file_call_paths
        .iter()
        .any(|call_path| path_mentions_route_construction(call_path))
        || input
            .file
            .parsed
            .functions
            .iter()
            .flat_map(|function| function.body.path_uses.iter())
            .any(|path| path_mentions_route_construction(path))
    {
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "test_support builds routed family input".to_owned(),
            "Shared `test_support` must stay generic and must not construct routed family inputs through FamilyMapper or placement wiring. Keep test_support independent of how families are routed.".to_owned(),
            Some(input.file.rel_path.clone()),
            None,
        ));
        reported = true;
    }

    for value in &input.file.parsed.public_values {
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "test_support exports public semantic constant".to_owned(),
            format!(
                "Shared `test_support` must stay generic and must not expose public {} `{}`.",
                match value.kind {
                    PublicValueKind::Const => "const",
                    PublicValueKind::Static => "static",
                },
                value.name
            ),
            Some(input.file.rel_path.clone()),
            Some(value.line),
        ));
        reported = true;
    }

    let local_canned_helpers = input
        .file
        .parsed
        .functions
        .iter()
        .filter(|function| {
            !function.is_public
                && !function.is_test
                && function.signature.arg_count == 0
                && matches!(
                    function.signature.return_kind,
                    ReturnKind::StringLike | ReturnKind::PathLike
                )
        })
        .map(|function| function.name.as_str())
        .collect::<BTreeSet<_>>();
    let local_semantic_helpers = semantic_helper_names(&input.file.parsed.functions);

    for function in input
        .file
        .parsed
        .functions
        .iter()
        .filter(|function| function.is_public && !function.is_test)
    {
        let references_file_value = function.body.path_uses.iter().any(|path| {
            path.first()
                .is_some_and(|first| input.file.parsed.file_value_names.contains(first))
        });
        let calls_local_canned_helper = function.body.call_paths.iter().any(|path| {
            path.len() == 1
                && local_canned_helpers.contains(path[0].as_str())
                && !function.body.shadowed_idents.contains(&path[0])
        });

        if matches!(
            function.signature.return_kind,
            ReturnKind::StringLike | ReturnKind::PathLike
        ) && (function.signature.arg_count == 0
            || references_file_value
            || calls_local_canned_helper)
        {
            results.push(G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Error,
                "test_support exports canned path or string helper".to_owned(),
                format!(
                    "Shared `test_support` must stay generic and must not expose public helper `{}` returning hardcoded path/string data. Helpers should accept parameters instead.",
                    function.name
                ),
                Some(input.file.rel_path.clone()),
                Some(function.line),
            ));
            reported = true;
            continue;
        }

        if function.signature.arg_count == 0
            && matches!(function.signature.return_kind, ReturnKind::Other)
            && calls_local_canned_helper
        {
            results.push(G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Error,
                "test_support exports canned fixture helper".to_owned(),
                format!(
                    "Shared `test_support` must stay generic and must not expose zero-argument public helper `{}` that wraps hardcoded fixture path/string data. Helpers should accept parameters instead.",
                    function.name
                ),
                Some(input.file.rel_path.clone()),
                Some(function.line),
            ));
            reported = true;
        }

        let selects_report_semantics = function.signature.has_check_result_arg
            && (function.signature.arg_names.contains("rule_id")
                || function.signature.arg_names.contains("id")
                || function
                    .body
                    .field_accesses
                    .iter()
                    .any(|field| REPORT_FIELDS.contains(&field.name.as_str()))
                || function
                    .body
                    .method_names
                    .iter()
                    .any(|method| REPORT_METHODS.contains(&method.as_str()))
                || function
                    .body
                    .path_uses
                    .iter()
                    .any(|path| path.last().is_some_and(|segment| segment == "CheckResult"))
                || function
                    .string_literals
                    .iter()
                    .any(|value| value.starts_with("RS-")));
        let calls_local_semantic_helper = function.body.call_paths.iter().any(|path| {
            path.len() == 1
                && local_semantic_helpers.contains(path[0].as_str())
                && !function.body.shadowed_idents.contains(&path[0])
        });
        if selects_report_semantics || calls_local_semantic_helper {
            results.push(G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Error,
                "test_support exports semantic finding helper".to_owned(),
                format!(
                    "Shared `test_support` must stay generic and must not expose public helper `{}` that filters or inspects CheckResult fields by rule ID or severity. Move rule-specific assertions into the assertions crate.",
                    function.name
                ),
                Some(input.file.rel_path.clone()),
                Some(function.line),
            ));
            reported = true;
        }
    }

    if !reported {
        results.push(
            G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Info,
                "test_support stays generic".to_owned(),
                "Shared `test_support` contains only generic fixture helpers and no local-component or semantic result-surface coupling.".to_owned(),
                Some(input.file.rel_path.clone()),
                None,
            )
            .into_inventory(),
        );
    }
}

fn semantic_helper_names<'a>(functions: &'a [FunctionInfo]) -> BTreeSet<&'a str> {
    let mut semantic_helpers =
        functions
            .iter()
            .filter(|function| !function.is_public && !function.is_test)
            .filter(|function| {
                function.signature.has_check_result_arg
                    && (function.signature.arg_names.contains("rule_id")
                        || function.signature.arg_names.contains("id")
                        || function
                            .body
                            .field_accesses
                            .iter()
                            .any(|field| REPORT_FIELDS.contains(&field.name.as_str()))
                        || function
                            .body
                            .method_names
                            .iter()
                            .any(|method| REPORT_METHODS.contains(&method.as_str()))
                        || function.body.path_uses.iter().any(|path| {
                            path.last().is_some_and(|segment| segment == "CheckResult")
                        })
                        || function
                            .string_literals
                            .iter()
                            .any(|value| value.starts_with("RS-")))
            })
            .map(|function| function.name.as_str())
            .collect::<BTreeSet<_>>();

    loop {
        let mut changed = false;
        for function in functions
            .iter()
            .filter(|function| !function.is_public && !function.is_test)
        {
            if semantic_helpers.contains(function.name.as_str()) {
                continue;
            }
            if function.body.call_paths.iter().any(|path| {
                path.len() == 1
                    && semantic_helpers.contains(path[0].as_str())
                    && !function.body.shadowed_idents.contains(&path[0])
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
