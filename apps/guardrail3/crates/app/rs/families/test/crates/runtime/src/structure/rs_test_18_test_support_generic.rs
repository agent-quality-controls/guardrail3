use std::collections::BTreeSet;

use crate::{CheckResult, Severity};

use super::inputs::TestSupportFileInput;
use super::parse::{PublicValueKind, ReturnKind};

const ID: &str = "RS-TEST-18";
const REPORT_FIELDS: &[&str] = &[
    "file",
    "id",
    "inventory",
    "line",
    "message",
    "severity",
    "title",
];

pub fn check(input: &TestSupportFileInput<'_>, results: &mut Vec<CheckResult>) {
    let mut reported = false;
    let disallowed_packages = input
        .local_runtime_packages
        .iter()
        .chain(input.local_assertions_packages.iter())
        .cloned()
        .collect::<BTreeSet<_>>();

    if !disallowed_packages.is_empty() {
        for binding in &input.parsed.imports {
            let Some(first) = binding.path_segments.first() else {
                continue;
            };
            if !disallowed_packages.contains(first) {
                continue;
            }
            results.push(CheckResult::from_parts(
                ID.to_owned(),
                Severity::Error,
                "test_support imports local component crate".to_owned(),
                format!(
                    "Shared `test_support` must stay generic and must not import local runtime/assertions crate `{first}`."
                ),
                Some(input.file.rel_path.clone()),
                Some(binding.line),
                false,
            ));
            reported = true;
        }
    }

    let mut reported_route_infra_imports = BTreeSet::new();
    for binding in &input.parsed.imports {
        if !binding
            .path_segments
            .iter()
            .any(|segment| segment == "FamilyMapper")
            || !reported_route_infra_imports.insert(binding.line)
        {
            continue;
        }
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "test_support imports route construction infrastructure".to_owned(),
            "Shared `test_support` must stay generic and must not import route-construction infrastructure."
                .to_owned(),
            Some(input.file.rel_path.clone()),
            Some(binding.line),
            false,
        ));
        reported = true;
    }

    if !disallowed_packages.is_empty() {
        let mut called_packages = BTreeSet::new();
        for call_path in &input.parsed.file_call_paths {
            let Some(first) = call_path.first() else {
                continue;
            };
            if !disallowed_packages.contains(first) || !called_packages.insert(first.clone()) {
                continue;
            }
            results.push(CheckResult::from_parts(
                ID.to_owned(),
                Severity::Error,
                "test_support calls local component crate".to_owned(),
                format!(
                    "Shared `test_support` must stay generic and must not call local runtime/assertions crate `{first}` directly."
                ),
                Some(input.file.rel_path.clone()),
                None,
                false,
            ));
            reported = true;
        }
    }

    if input
        .parsed
        .file_call_paths
        .iter()
        .any(|call_path| call_path.iter().any(|segment| segment == "FamilyMapper"))
    {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "test_support builds routed family input".to_owned(),
            "Shared `test_support` must stay generic and must not construct routed family inputs through `FamilyMapper`.".to_owned(),
            Some(input.file.rel_path.clone()),
            None,
            false,
        ));
        reported = true;
    }

    for value in &input.parsed.public_values {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: "test_support exports public semantic constant".to_owned(),
            message: format!(
                "Shared `test_support` must stay generic and must not expose public {} `{}`.",
                match value.kind {
                    PublicValueKind::Const => "const",
                    PublicValueKind::Static => "static",
                },
                value.name
            ),
            file: Some(input.file.rel_path.clone()),
            line: Some(value.line),
            inventory: false,
        });
        reported = true;
    }

    let local_canned_helpers = input
        .parsed
        .functions
        .iter()
        .filter(|function| {
            !function.is_public
                && !function.is_test
                && function.arg_count == 0
                && matches!(
                    function.return_kind,
                    ReturnKind::StringLike | ReturnKind::PathLike
                )
        })
        .map(|function| function.name.as_str())
        .collect::<BTreeSet<_>>();
    let local_semantic_helpers = semantic_helper_names(&input.parsed.functions);

    for function in input
        .parsed
        .functions
        .iter()
        .filter(|function| function.is_public && !function.is_test)
    {
        let references_file_value = function.path_uses.iter().any(|path| {
            path.first()
                .is_some_and(|first| input.parsed.file_value_names.contains(first))
        });
        let calls_local_canned_helper = function.call_paths.iter().any(|path| {
            path.len() == 1
                && local_canned_helpers.contains(path[0].as_str())
                && !function.shadowed_idents.contains(&path[0])
        });

        if matches!(
            function.return_kind,
            ReturnKind::StringLike | ReturnKind::PathLike
        ) && (function.arg_count == 0 || references_file_value || calls_local_canned_helper)
        {
            results.push(CheckResult {
                id: ID.to_owned(),
                severity: Severity::Error,
                title: "test_support exports canned path or string helper".to_owned(),
                message: format!(
                    "Shared `test_support` must stay generic and must not expose public helper `{}` returning canned path/string data.",
                    function.name
                ),
                file: Some(input.file.rel_path.clone()),
                line: Some(function.line),
                inventory: false,
            });
            reported = true;
            continue;
        }

        if function.arg_count == 0
            && matches!(function.return_kind, ReturnKind::Other)
            && calls_local_canned_helper
        {
            results.push(CheckResult {
                id: ID.to_owned(),
                severity: Severity::Error,
                title: "test_support exports canned fixture helper".to_owned(),
                message: format!(
                    "Shared `test_support` must stay generic and must not expose zero-argument public helper `{}` that wraps canned fixture path/string data.",
                    function.name
                ),
                file: Some(input.file.rel_path.clone()),
                line: Some(function.line),
                inventory: false,
            });
            reported = true;
        }

        let selects_report_semantics = function.has_check_result_arg
            && (function.arg_names.contains("rule_id")
                || function.arg_names.contains("id")
                || function
                    .field_accesses
                    .iter()
                    .any(|field| REPORT_FIELDS.contains(&field.name.as_str()))
                || function
                    .path_uses
                    .iter()
                    .any(|path| path.last().is_some_and(|segment| segment == "CheckResult"))
                || function
                    .string_literals
                    .iter()
                    .any(|value| value.starts_with("RS-")));
        let calls_local_semantic_helper = function.call_paths.iter().any(|path| {
            path.len() == 1
                && local_semantic_helpers.contains(path[0].as_str())
                && !function.shadowed_idents.contains(&path[0])
        });
        if selects_report_semantics || calls_local_semantic_helper {
            results.push(CheckResult {
                id: ID.to_owned(),
                severity: Severity::Error,
                title: "test_support exports semantic finding helper".to_owned(),
                message: format!(
                    "Shared `test_support` must stay generic and must not expose public helper `{}` that selects or inspects guardrail findings/results by rule semantics.",
                    function.name
                ),
                file: Some(input.file.rel_path.clone()),
                line: Some(function.line),
                inventory: false,
            });
            reported = true;
        }
    }

    if !reported {
        results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: "test_support stays generic".to_owned(),
                message: "Shared `test_support` contains only generic fixture helpers and no local-component or semantic result-surface coupling.".to_owned(),
                file: Some(input.file.rel_path.clone()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    }
}

fn semantic_helper_names<'a>(functions: &'a [super::parse::FunctionInfo]) -> BTreeSet<&'a str> {
    let mut semantic_helpers = functions
        .iter()
        .filter(|function| !function.is_public && !function.is_test)
        .filter(|function| {
            function.has_check_result_arg
                && (function.arg_names.contains("rule_id")
                    || function.arg_names.contains("id")
                    || function
                        .field_accesses
                        .iter()
                        .any(|field| REPORT_FIELDS.contains(&field.name.as_str()))
                    || function
                        .path_uses
                        .iter()
                        .any(|path| path.last().is_some_and(|segment| segment == "CheckResult"))
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
            if function.call_paths.iter().any(|path| {
                path.len() == 1
                    && semantic_helpers.contains(path[0].as_str())
                    && !function.shadowed_idents.contains(&path[0])
            }) {
                changed |= semantic_helpers.insert(function.name.as_str());
            }
        }
        if !changed {
            break;
        }
    }

    semantic_helpers,
)

#[cfg(test)]
pub(crate) fn run_family(root: &std::path::Path) -> Vec<CheckResult> {
    let tree = test_support::walk(root);
    super::check_test_tree(&tree, &test_support::StubToolChecker::default()),
)
#[cfg(test)]
#[path = "rs_test_18_test_support_generic_tests/mod.rs"]
mod rs_test_18_test_support_generic_tests;
