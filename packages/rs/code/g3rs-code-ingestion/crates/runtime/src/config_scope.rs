use std::collections::BTreeSet;

use cargo_toml_parser::{types::CargoToml, parse};
use g3rs_workspace_crawl::{G3RsWorkspaceCrawl, G3RsWorkspaceEntry, G3RsWorkspaceEntryKind};
use glob::Pattern;

use crate::run::IngestionError;

pub(crate) fn select_owned_config_entries<'a>(
    crawl: &'a G3RsWorkspaceCrawl,
    file_names: &[&str],
) -> Result<Vec<&'a G3RsWorkspaceEntry>, IngestionError> {
    let owned_roots = owned_root_dirs(crawl)?;
    let mut entries = crawl
        .entries
        .iter()
        .filter(|entry| entry.kind == G3RsWorkspaceEntryKind::File)
        .filter(|entry| {
            std::path::Path::new(entry.path.rel_path.as_str())
                .file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| file_names.contains(&name))
                && file_is_within_owned_root(entry.path.rel_path.as_str(), &owned_roots)
        })
        .collect::<Vec<_>>();

    entries.sort_by(|left, right| left.path.rel_path.cmp(&right.path.rel_path));
    Ok(entries)
}

pub(crate) fn owned_root_dirs(
    crawl: &G3RsWorkspaceCrawl,
) -> Result<BTreeSet<String>, IngestionError> {
    let mut roots = BTreeSet::from([String::new()]);

    let Some(root_cargo_entry) = g3rs_workspace_crawl::root_file(crawl, "Cargo.toml") else {
        return Ok(roots);
    };

    if !root_cargo_entry.readable {
        return Err(IngestionError::Unreadable {
            path: root_cargo_entry.path.abs_path.clone(),
            reason: "file is not readable".to_owned(),
        });
    }

    let content = crate::fs::read_to_string(&root_cargo_entry.path.abs_path).map_err(|err| {
        IngestionError::Unreadable {
            path: root_cargo_entry.path.abs_path.clone(),
            reason: err.to_string(),
        }
    })?;
    let root_cargo = parse(&content).map_err(|err| IngestionError::ParseFailed {
        path: root_cargo_entry.path.abs_path.clone(),
        reason: err.to_string(),
    })?;

    if root_cargo.workspace.is_some() {
        roots.extend(select_workspace_member_dirs(crawl, &root_cargo)?);
    }

    Ok(roots)
}

fn select_workspace_member_dirs(
    crawl: &G3RsWorkspaceCrawl,
    workspace_cargo: &CargoToml,
) -> Result<BTreeSet<String>, IngestionError> {
    let workspace =
        workspace_cargo
            .workspace
            .as_ref()
            .ok_or_else(|| IngestionError::ParseFailed {
                path: crawl.root_abs_path.join("Cargo.toml"),
                reason: "workspace Cargo.toml has no [workspace] section".to_owned(),
            })?;

    let member_patterns = workspace
        .members
        .iter()
        .map(|pattern| {
            Pattern::new(pattern).map_err(|err| IngestionError::ParseFailed {
                path: crawl.root_abs_path.join("Cargo.toml"),
                reason: format!("invalid workspace member pattern `{pattern}`: {err}"),
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    let exclude_patterns = workspace
        .exclude
        .iter()
        .map(|pattern| {
            Pattern::new(pattern).map_err(|err| IngestionError::ParseFailed {
                path: crawl.root_abs_path.join("Cargo.toml"),
                reason: format!("invalid workspace exclude pattern `{pattern}`: {err}"),
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    let member_dirs = crawl
        .entries
        .iter()
        .filter(|entry| entry.kind == G3RsWorkspaceEntryKind::File)
        .filter_map(|entry| manifest_dir_from_manifest_path(entry.path.rel_path.as_str()))
        .filter(|member_dir| {
            !member_dir.is_empty()
                && member_patterns
                    .iter()
                    .any(|pattern| pattern.matches(member_dir))
                && !exclude_patterns
                    .iter()
                    .any(|pattern| pattern.matches(member_dir))
        })
        .map(str::to_owned)
        .collect::<BTreeSet<_>>();

    for (raw_pattern, pattern) in workspace.members.iter().zip(member_patterns.iter()) {
        let matched = member_dirs
            .iter()
            .any(|member_dir| pattern.matches(member_dir));
        if !matched {
            return Err(IngestionError::ParseFailed {
                path: crawl.root_abs_path.join("Cargo.toml"),
                reason: format!(
                    "workspace member pattern `{raw_pattern}` did not resolve to any Cargo.toml"
                ),
            });
        }
    }

    Ok(member_dirs)
}

fn manifest_dir_from_manifest_path(rel_path: &str) -> Option<&str> {
    if rel_path == "Cargo.toml" {
        return Some("");
    }

    rel_path.strip_suffix("/Cargo.toml")
}

fn file_is_within_owned_root(rel_path: &str, owned_roots: &BTreeSet<String>) -> bool {
    let parent = rel_path.rsplit_once('/').map_or("", |(dir, _)| dir);

    owned_roots.iter().any(|root| {
        if root.is_empty() {
            parent.is_empty()
        } else {
            parent == root
                || parent
                    .strip_prefix(root.as_str())
                    .is_some_and(|rest| rest.starts_with('/'))
        }
    })
}
