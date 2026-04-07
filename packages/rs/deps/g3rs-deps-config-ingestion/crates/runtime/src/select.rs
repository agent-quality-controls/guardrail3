/// Select config entries from a workspace crawl.
use cargo_toml_parser::CargoToml;
use g3rs_workspace_crawl::{G3RsWorkspaceCrawl, G3RsWorkspaceEntry, G3RsWorkspaceEntryKind};
use glob::Pattern;

/// Find `Cargo.toml` at the workspace root.
pub(crate) fn select_workspace_cargo_toml(
    crawl: &G3RsWorkspaceCrawl,
) -> Option<&G3RsWorkspaceEntry> {
    crawl.root_file("Cargo.toml")
}

/// Find `guardrail3-rs.toml` at the workspace root.
pub(crate) fn select_workspace_guardrail3_rs_toml(
    crawl: &G3RsWorkspaceCrawl,
) -> Option<&G3RsWorkspaceEntry> {
    crawl.root_file("guardrail3-rs.toml")
}

/// Find member `Cargo.toml` files declared by `[workspace].members`.
pub(crate) fn select_member_cargo_tomls<'a>(
    crawl: &'a G3RsWorkspaceCrawl,
    workspace_cargo: &CargoToml,
) -> Result<Vec<&'a G3RsWorkspaceEntry>, String> {
    let workspace = workspace_cargo
        .workspace
        .as_ref()
        .ok_or_else(|| "workspace Cargo.toml has no [workspace] section".to_owned())?;
    let member_patterns = workspace
        .members
        .iter()
        .map(|pattern| {
            Pattern::new(pattern)
                .map(|compiled| (pattern.as_str(), compiled))
                .map_err(|err| format!("invalid workspace member pattern `{pattern}`: {err}"))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let exclude_patterns = workspace
        .exclude
        .iter()
        .map(|pattern| {
            Pattern::new(pattern)
                .map(|compiled| (pattern.as_str(), compiled))
                .map_err(|err| format!("invalid workspace exclude pattern `{pattern}`: {err}"))
        })
        .collect::<Result<Vec<_>, _>>()?;

    let mut members = crawl
        .entries
        .iter()
        .filter(|entry| {
            entry.kind == G3RsWorkspaceEntryKind::File
                && is_member_manifest_path(entry.path.rel_path.as_str())
        })
        .filter(|entry| {
            let Some(member_dir) = member_dir_from_manifest_path(entry.path.rel_path.as_str())
            else {
                return false;
            };
            member_patterns
                .iter()
                .any(|(_, pattern)| pattern.matches(member_dir))
                && !exclude_patterns
                    .iter()
                    .any(|(_, pattern)| pattern.matches(member_dir))
        })
        .collect::<Vec<_>>();

    if workspace_cargo.package.is_some() && !workspace_root_is_excluded(&exclude_patterns) {
        let root_entry = crawl
            .root_file("Cargo.toml")
            .ok_or_else(|| "workspace root Cargo.toml missing from crawl".to_owned())?;
        members.push(root_entry);
    }

    for (raw_pattern, pattern) in &member_patterns {
        let matched = members.iter().any(|entry| {
            member_dir_from_manifest_path(entry.path.rel_path.as_str())
                .is_some_and(|member_dir| pattern.matches(member_dir))
        });
        if !matched {
            return Err(format!(
                "workspace member pattern `{raw_pattern}` did not resolve to any Cargo.toml"
            ));
        }
    }

    members.sort_by(|left, right| left.path.rel_path.cmp(&right.path.rel_path));
    members.dedup_by(|left, right| left.path.rel_path == right.path.rel_path);
    Ok(members)
}

fn is_member_manifest_path(rel_path: &str) -> bool {
    rel_path == "Cargo.toml" || rel_path.ends_with("/Cargo.toml")
}

fn member_dir_from_manifest_path(rel_path: &str) -> Option<&str> {
    if rel_path == "Cargo.toml" {
        return Some("");
    }

    rel_path.strip_suffix("/Cargo.toml")
}

fn workspace_root_is_excluded(exclude_patterns: &[(&str, Pattern)]) -> bool {
    exclude_patterns
        .iter()
        .any(|(_, pattern)| pattern.matches(""))
}
