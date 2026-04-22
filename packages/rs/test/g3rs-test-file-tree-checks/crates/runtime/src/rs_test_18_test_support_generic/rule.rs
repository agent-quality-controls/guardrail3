use std::collections::{BTreeMap, BTreeSet};

use crate::support::TestSupportFileInput;
use g3rs_test_types::ast::{FunctionInfo, PublicValueKind, ReturnKind, UseBinding};
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

fn calls_local_helper(
    function: &FunctionInfo,
    local_helpers: &BTreeSet<&str>,
    local_import_aliases: &BTreeMap<String, Vec<String>>,
) -> bool {
    function.body.call_paths.iter().any(|path| {
        call_path_uses_local_helper(
            path,
            local_helpers,
            &function.body.local_call_aliases,
            local_import_aliases,
            &function.body.shadowed_idents,
        )
    })
}

pub(crate) fn check(input: &TestSupportFileInput<'_>, results: &mut Vec<G3CheckResult>) {
    let mut reported = false;
    let local_import_aliases = local_import_aliases(&input.file.parsed.imports);
    let mut local_canned_helpers =
        canned_helper_names(&input.file.parsed.functions, &local_import_aliases);
    let mut local_semantic_helpers =
        semantic_helper_names_owned(&input.file.parsed.functions, &local_import_aliases);
    let (glob_canned_helpers, glob_semantic_helpers) = glob_imported_helper_names(input);
    local_canned_helpers.extend(glob_canned_helpers);
    local_semantic_helpers.extend(glob_semantic_helpers);
    let local_canned_helpers = helper_name_refs(&local_canned_helpers);
    let local_semantic_helpers = helper_name_refs(&local_semantic_helpers);
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
        let calls_local_canned_helper =
            calls_local_helper(function, &local_canned_helpers, &local_import_aliases);

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
        let calls_local_semantic_helper =
            calls_local_helper(function, &local_semantic_helpers, &local_import_aliases);
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

fn canned_helper_names(
    functions: &[FunctionInfo],
    local_import_aliases: &BTreeMap<String, Vec<String>>,
) -> BTreeSet<String> {
    let mut canned_helpers = functions
        .iter()
        .filter(|function| !function.is_test)
        .filter(|function| {
            function.signature.arg_count == 0
                && matches!(
                    function.signature.return_kind,
                    ReturnKind::StringLike | ReturnKind::PathLike
                )
        })
        .map(|function| function.name.clone())
        .collect::<BTreeSet<_>>();

    loop {
        let mut changed = false;
        for function in functions.iter().filter(|function| !function.is_test) {
            if canned_helpers.contains(&function.name) {
                continue;
            }
            let calls_helper = {
                let helper_refs = helper_name_refs(&canned_helpers);
                function.body.call_paths.iter().any(|path| {
                    call_path_uses_local_helper(
                        path,
                        &helper_refs,
                        &function.body.local_call_aliases,
                        local_import_aliases,
                        &function.body.shadowed_idents,
                    )
                })
            };
            if calls_helper {
                changed |= canned_helpers.insert(function.name.clone());
            }
        }
        if !changed {
            break;
        }
    }

    canned_helpers
}

fn semantic_helper_names_owned(
    functions: &[FunctionInfo],
    local_import_aliases: &BTreeMap<String, Vec<String>>,
) -> BTreeSet<String> {
    let mut semantic_helpers = functions
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
        .map(|function| function.name.clone())
        .collect::<BTreeSet<_>>();

    loop {
        let mut changed = false;
        for function in functions.iter().filter(|function| !function.is_test) {
            if semantic_helpers.contains(&function.name) {
                continue;
            }
            let calls_helper = {
                let helper_refs = helper_name_refs(&semantic_helpers);
                function.body.call_paths.iter().any(|path| {
                    call_path_uses_local_helper(
                        path,
                        &helper_refs,
                        &function.body.local_call_aliases,
                        local_import_aliases,
                        &function.body.shadowed_idents,
                    )
                })
            };
            if calls_helper {
                changed |= semantic_helpers.insert(function.name.clone());
            }
        }
        if !changed {
            break;
        }
    }

    semantic_helpers
}

fn glob_imported_helper_names(
    input: &TestSupportFileInput<'_>,
) -> (BTreeSet<String>, BTreeSet<String>) {
    let mut canned_helpers = BTreeSet::new();
    let mut semantic_helpers = BTreeSet::new();

    for binding in input
        .file
        .parsed
        .imports
        .iter()
        .filter(|binding| binding.local_name.is_none())
    {
        let Some(target_paths) =
            glob_import_target_paths(&input.file.rel_path, &binding.path_segments)
        else {
            continue;
        };
        for sibling in input
            .sibling_files
            .iter()
            .filter(|file| file.rel_path != input.file.rel_path)
        {
            if !target_paths.iter().any(|target| target == &sibling.rel_path) {
                continue;
            }
            let local_import_aliases = local_import_aliases(&sibling.parsed.imports);
            canned_helpers.extend(canned_helper_names(
                &sibling.parsed.functions,
                &local_import_aliases,
            ));
            semantic_helpers.extend(semantic_helper_names_owned(
                &sibling.parsed.functions,
                &local_import_aliases,
            ));
        }
    }

    (canned_helpers, semantic_helpers)
}

fn glob_import_target_paths(file_rel_path: &str, path_segments: &[String]) -> Option<Vec<String>> {
    let first = path_segments.first()?;
    let relative = path_segments.get(1..)?.join("/");
    if relative.is_empty() {
        return None;
    }
    let base_dir = match first.as_str() {
        "self" => parent_dir(file_rel_path)?.to_owned(),
        "super" => parent_dir(parent_dir(file_rel_path)?)?.to_owned(),
        "crate" => package_src_root(file_rel_path)?,
        _ => return None,
    };

    Some(vec![
        format!("{base_dir}/{relative}.rs"),
        format!("{base_dir}/{relative}/mod.rs"),
    ])
}

fn package_src_root(file_rel_path: &str) -> Option<String> {
    let (prefix, _) = file_rel_path.split_once("/src/")?;
    Some(format!("{prefix}/src"))
}

fn parent_dir(path: &str) -> Option<&str> {
    path.rsplit_once('/').map(|(parent, _)| parent)
}

fn helper_name_refs<'a>(names: &'a BTreeSet<String>) -> BTreeSet<&'a str> {
    names.iter().map(String::as_str).collect()
}

fn call_path_uses_local_helper(
    path: &[String],
    local_helpers: &BTreeSet<&str>,
    local_call_aliases: &std::collections::BTreeMap<String, Vec<String>>,
    local_import_aliases: &std::collections::BTreeMap<String, Vec<String>>,
    shadowed_idents: &BTreeSet<String>,
) -> bool {
    match path {
        [name] => {
            if local_helpers.contains(name.as_str()) && !shadowed_idents.contains(name) {
                return true;
            }
            if local_call_aliases.contains_key(name) {
                return local_call_alias_targets_local_helper(
                    name,
                    local_helpers,
                    local_call_aliases,
                );
            }
            local_import_aliases.contains_key(name)
                && local_call_alias_targets_local_helper(
                    name,
                    local_helpers,
                    local_import_aliases,
                )
        }
        [first, ..] if matches!(first.as_str(), "crate" | "self" | "super") => {
            path.last()
                .is_some_and(|name| local_helpers.contains(name.as_str()))
        }
        _ => false,
    }
}

fn local_call_alias_targets_local_helper(
    name: &str,
    local_helpers: &BTreeSet<&str>,
    local_call_aliases: &std::collections::BTreeMap<String, Vec<String>>,
) -> bool {
    let mut current = name;
    let mut seen = BTreeSet::new();

    loop {
        if local_helpers.contains(current) {
            return true;
        }
        if !seen.insert(current.to_owned()) {
            return false;
        }
        let Some(target) = local_call_aliases.get(current) else {
            return false;
        };
        let Some(next) = target.last() else {
            return false;
        };
        current = next;
    }
}

fn local_import_aliases(imports: &[UseBinding]) -> BTreeMap<String, Vec<String>> {
    let mut aliases = BTreeMap::new();
    loop {
        let mut changed = false;
        for binding in imports {
            let Some(first) = binding.path_segments.first() else {
                continue;
            };
            if !matches!(first.as_str(), "crate" | "self" | "super")
                && !aliases.contains_key(first)
            {
                continue;
            }
            let Some(local_name) = binding
                .local_name
                .clone()
                .or_else(|| binding.path_segments.last().cloned())
            else {
                continue;
            };
            let previous = aliases.insert(local_name, binding.path_segments.clone());
            changed |= previous
                .as_ref()
                .is_none_or(|existing| existing != &binding.path_segments);
        }
        if !changed {
            break;
        }
    }
    aliases
}
