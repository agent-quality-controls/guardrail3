use std::collections::{BTreeMap, BTreeSet};

use crate::support::TestSupportFileInput;
use g3rs_test_types::ast::{FunctionInfo, ReturnKind, UseBinding};

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

pub(super) fn path_mentions_route_construction(path: &[String]) -> bool {
    path.iter().any(|segment| {
        matches!(
            segment.as_str(),
            "FamilyMapper" | "guardrail3_app_rs_placement"
        )
    })
}

pub(super) fn calls_local_helper(
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

pub(super) fn canned_helper_names(
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

pub(super) fn semantic_helper_names_owned(
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

pub(super) fn glob_imported_helper_names(
    input: &TestSupportFileInput<'_>,
) -> (BTreeSet<String>, BTreeSet<String>) {
    let mut canned_helpers = BTreeSet::new();
    let mut semantic_helpers = BTreeSet::new();

    for binding in &input.file.parsed.imports {
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

pub(super) fn local_import_aliases(imports: &[UseBinding]) -> BTreeMap<String, Vec<String>> {
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

pub(super) fn helper_name_refs_owned<'a>(names: &'a BTreeSet<String>) -> BTreeSet<&'a str> {
    helper_name_refs(names)
}

fn call_path_uses_local_helper(
    path: &[String],
    local_helpers: &BTreeSet<&str>,
    local_call_aliases: &BTreeMap<String, Vec<String>>,
    local_import_aliases: &BTreeMap<String, Vec<String>>,
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
        [first, ..]
            if local_import_aliases.contains_key(first) && !shadowed_idents.contains(first) =>
        {
            path.last()
                .is_some_and(|name| local_helpers.contains(name.as_str()))
        }
        _ => false,
    }
}

fn local_call_alias_targets_local_helper(
    name: &str,
    local_helpers: &BTreeSet<&str>,
    local_call_aliases: &BTreeMap<String, Vec<String>>,
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
