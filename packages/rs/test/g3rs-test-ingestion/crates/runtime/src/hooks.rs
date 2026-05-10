#![expect(
    clippy::arithmetic_side_effects,
    reason = "structural code pattern (parser/assertion helper) where lint conflicts with module architecture"
)]
#![expect(
    clippy::excessive_nesting,
    reason = "structural code pattern (parser/assertion helper) where lint conflicts with module architecture"
)]
use std::collections::BTreeSet;

use g3rs_workspace_crawl::{G3RsWorkspaceCrawl, G3RsWorkspaceEntryKind};
use hook_shell_parser::command_query::{ResolvedCommand, any_resolved_command};

use crate::ingest::IngestionError;
use crate::roots::{OwnedTestRoot, TestRootDiscovery, join_under_root};

/// `MutationHookState` struct.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct MutationHookState {
    /// `active` item.
    pub(crate) active: bool,
    /// `files` item.
    pub(crate) files: Vec<String>,
}

/// `collect_mutation_hook_state` function.
pub(crate) fn collect_mutation_hook_state(
    crawl: &G3RsWorkspaceCrawl,
    discovery: &TestRootDiscovery,
    root: &OwnedTestRoot,
) -> Result<MutationHookState, IngestionError> {
    let hook_root_rels = active_hook_root_dirs(discovery, root);
    let mut active = false;
    let mut files = Vec::new();

    for hook_root_rel in &hook_root_rels {
        for rel_path in [
            join_under_root(hook_root_rel, ".githooks/pre-commit"),
            join_under_root(hook_root_rel, "hooks/pre-commit"),
        ] {
            if script_contains_mutation_step(crawl, &rel_path)? {
                active = true;
                if hook_root_rel == &root.root_rel_dir {
                    files.push(rel_path);
                }
            }
        }

        let hook_dir_rel = join_under_root(hook_root_rel, ".githooks/pre-commit.d");
        for entry in crawl.entries.iter().filter(|entry| {
            entry.kind == G3RsWorkspaceEntryKind::File
                && entry
                    .path
                    .rel_path
                    .starts_with(&(hook_dir_rel.clone() + "/"))
        }) {
            if script_contains_mutation_step(crawl, &entry.path.rel_path)? {
                active = true;
                if hook_root_rel == &root.root_rel_dir {
                    files.push(entry.path.rel_path.clone());
                }
            }
        }
    }

    files.sort();
    files.dedup();
    Ok(MutationHookState { active, files })
}

/// `active_hook_root_dirs` function.
fn active_hook_root_dirs(discovery: &TestRootDiscovery, root: &OwnedTestRoot) -> Vec<String> {
    let mut roots = BTreeSet::new();
    if root.root_rel_dir.is_empty() {
        let _ = roots.insert(String::new());
    }
    if discovery
        .workspace_manifest
        .as_ref()
        .and_then(|manifest| manifest.workspace.as_ref())
        .is_some()
        && (root.root_rel_dir.is_empty()
            || discovery.workspace_members.contains(&root.root_rel_dir))
    {
        let _ = roots.insert(String::new());
    }
    if root
        .root_manifest
        .as_ref()
        .and_then(|manifest| manifest.workspace.as_ref())
        .is_some()
    {
        let _ = roots.insert(root.root_rel_dir.clone());
    }
    roots.into_iter().collect()
}

/// `script_contains_mutation_step` function.
fn script_contains_mutation_step(
    crawl: &G3RsWorkspaceCrawl,
    rel_path: &str,
) -> Result<bool, IngestionError> {
    let Some(entry) = g3rs_workspace_crawl::entry(crawl, rel_path) else {
        return Ok(false);
    };
    if !entry.readable {
        return Err(IngestionError::Unreadable {
            path: entry.path.abs_path.clone(),
            reason: "file is not readable".to_owned(),
        });
    }
    let content = crate::fs::read_to_string(&entry.path.abs_path).map_err(|err| {
        IngestionError::Unreadable {
            path: entry.path.abs_path.clone(),
            reason: err.to_string(),
        }
    })?;
    let parsed = hook_shell_parser::parse_script(&content);
    Ok(any_resolved_command(&parsed, is_cargo_mutants_command))
}

/// `is_cargo_mutants_command` function.
fn is_cargo_mutants_command(command: &ResolvedCommand) -> bool {
    match command.command_name() {
        "cargo" => cargo_mutants_subcommand(command.args()),
        "cargo-mutants" => !args_have_help_or_version(command.args()),
        _ => false,
    }
}

/// `cargo_mutants_subcommand` function.
fn cargo_mutants_subcommand(args: &[String]) -> bool {
    let mut index = 0usize;

    if args.get(index).is_some_and(|token| token.starts_with('+')) {
        index += 1;
    }

    while let Some(token) = args.get(index).map(String::as_str) {
        if !token.starts_with('-') {
            break;
        }

        if is_help_or_version_flag(token) {
            return false;
        }
        if let Some((flag_name, _)) = token.split_once('=')
            && cargo_global_flag_takes_value(flag_name)
        {
            index += 1;
            continue;
        }
        if cargo_global_flag_takes_value(token) {
            index += 2;
            continue;
        }
        index += 1;
    }

    args.get(index).map(String::as_str) == Some("mutants")
        && !args_have_help_or_version(args.get(index + 1..).unwrap_or(&[]))
}

/// `cargo_global_flag_takes_value` function.
fn cargo_global_flag_takes_value(flag: &str) -> bool {
    matches!(
        flag,
        "--config"
            | "-Z"
            | "--manifest-path"
            | "--color"
            | "--target"
            | "--target-dir"
            | "--jobs"
            | "-j"
            | "-C"
    )
}

/// `args_have_help_or_version` function.
fn args_have_help_or_version(args: &[String]) -> bool {
    args.iter().any(|arg| is_help_or_version_flag(arg))
}

/// `is_help_or_version_flag` function.
fn is_help_or_version_flag(token: &str) -> bool {
    matches!(token, "-h" | "--help" | "-V" | "--version")
}
