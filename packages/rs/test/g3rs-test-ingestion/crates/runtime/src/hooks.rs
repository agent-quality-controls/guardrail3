use std::collections::BTreeSet;

use g3rs_workspace_crawl::{G3RsWorkspaceCrawl, G3RsWorkspaceEntryKind};

use crate::hook_shell::{ExecutableLine, parse_script};
use crate::roots::{OwnedTestRoot, TestRootDiscovery, join_under_root};
use crate::ingest::IngestionError;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct MutationHookState {
    pub(crate) active: bool,
    pub(crate) files: Vec<String>,
}

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
    Ok(parse_script(&content)
        .executable_lines()
        .iter()
        .any(executable_line_has_mutation_hook))
}

fn executable_line_has_mutation_hook(line: &ExecutableLine<'_>) -> bool {
    is_cargo_mutants_command(line.command_text())
}

fn is_cargo_mutants_command(command_text: &str) -> bool {
    let tokens = shell_words(command_text);
    let mut parts = tokens.iter().map(String::as_str).peekable();

    while matches!(parts.peek(), Some(token) if looks_like_env_assignment(token)) {
        let _ = parts.next();
    }

    let Some(first) = parts.next() else {
        return false;
    };

    let first = normalize_command_token(first);
    if matches!(first, "exec" | "command") {
        while matches!(parts.peek(), Some(token) if token.starts_with('-')) {
            let _ = parts.next();
        }
        let rest = parts.collect::<Vec<_>>();
        return is_cargo_mutants_from_tokens(&rest);
    }
    if matches!(first, "bash" | "sh" | "zsh") {
        while let Some(flag) = parts.next() {
            if matches!(flag, "-c" | "-lc") {
                return parts.next().is_some_and(is_cargo_mutants_command);
            }
        }
        return false;
    }
    if first == "env" {
        while matches!(parts.peek(), Some(token) if token.starts_with('-')) {
            let _ = parts.next();
        }
        while matches!(parts.peek(), Some(token) if looks_like_env_assignment(token)) {
            let _ = parts.next();
        }
        let rest = parts.collect::<Vec<_>>();
        return is_cargo_mutants_from_tokens(&rest);
    }

    let rest = parts.collect::<Vec<_>>();
    is_cargo_mutants_from_tokens_with_first(first, &rest)
}

fn is_cargo_mutants_from_tokens(tokens: &[&str]) -> bool {
    let Some((first, rest)) = tokens.split_first() else {
        return false;
    };
    is_cargo_mutants_from_tokens_with_first(normalize_command_token(first), rest)
}

fn is_cargo_mutants_from_tokens_with_first(first: &str, rest: &[&str]) -> bool {
    let mut parts = rest.iter().copied().peekable();
    match first {
        "cargo" => is_cargo_mutants_invocation(&mut parts),
        "cargo-mutants" => !parts.any(is_help_or_version_flag),
        _ => false,
    }
}

fn is_cargo_mutants_invocation<'a, I>(parts: &mut std::iter::Peekable<I>) -> bool
where
    I: Iterator<Item = &'a str>,
{
    if matches!(parts.peek(), Some(token) if token.starts_with('+')) {
        let _ = parts.next();
    }

    while let Some(token) = parts.peek().copied() {
        if !token.starts_with('-') {
            break;
        }

        let flag = parts.next().unwrap_or_default();
        if is_help_or_version_flag(flag) {
            return false;
        }
        if cargo_global_flag_takes_value(flag) {
            let _ = parts.next();
        }
    }

    parts.next() == Some("mutants") && !parts.any(is_help_or_version_flag)
}

fn cargo_global_flag_takes_value(flag: &str) -> bool {
    matches!(
        flag,
        "--config" | "-Z" | "--manifest-path" | "--color" | "--target" | "--target-dir" | "--jobs"
    )
}

fn is_help_or_version_flag(token: &str) -> bool {
    matches!(token, "-h" | "--help" | "-V" | "--version")
}

fn normalize_command_token(token: &str) -> &str {
    token.rsplit('/').next().unwrap_or(token)
}

fn looks_like_env_assignment(token: &str) -> bool {
    let Some((name, _value)) = token.split_once('=') else {
        return false;
    };
    let mut chars = name.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    (first.is_ascii_alphabetic() || first == '_')
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn shell_words(command_text: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut current = String::new();
    let mut chars = command_text.chars().peekable();
    let mut single_quoted = false;
    let mut double_quoted = false;

    while let Some(ch) = chars.next() {
        match ch {
            '\'' if !double_quoted => {
                single_quoted = !single_quoted;
            }
            '"' if !single_quoted => {
                double_quoted = !double_quoted;
            }
            '\\' if double_quoted => {
                if let Some(next) = chars.next() {
                    current.push(next);
                }
            }
            ch if ch.is_whitespace() && !single_quoted && !double_quoted => {
                if !current.is_empty() {
                    words.push(std::mem::take(&mut current));
                }
            }
            _ => current.push(ch),
        }
    }

    if !current.is_empty() {
        words.push(current);
    }

    words
}
