use cargo_toml_parser::types::CargoToml;
use g3rs_workspace_crawl::{G3RsWorkspaceCrawl, G3RsWorkspaceEntry, G3RsWorkspaceEntryKind};
use glob::Pattern;

/// Result type returned by member-rel and pattern-expansion helpers; `Err` carries a human-readable reason.
type MemberRelsResult = Result<Vec<String>, String>;

/// `select_cargo_toml` function.
pub(crate) fn select_cargo_toml(crawl: &G3RsWorkspaceCrawl) -> Option<&G3RsWorkspaceEntry> {
    g3rs_workspace_crawl::root_file(crawl, "Cargo.toml")
}

/// `select_release_plz_toml` function.
pub(crate) fn select_release_plz_toml(crawl: &G3RsWorkspaceCrawl) -> Option<&G3RsWorkspaceEntry> {
    g3rs_workspace_crawl::root_file(crawl, "release-plz.toml")
}

/// `select_cliff_toml` function.
pub(crate) fn select_cliff_toml(crawl: &G3RsWorkspaceCrawl) -> Option<&G3RsWorkspaceEntry> {
    g3rs_workspace_crawl::root_file(crawl, "cliff.toml")
}

/// `select_member_manifest` function.
pub(crate) fn select_member_manifest<'a>(
    crawl: &'a G3RsWorkspaceCrawl,
    member_rel: &str,
) -> Option<&'a G3RsWorkspaceEntry> {
    let rel_path = member_manifest_rel_path(member_rel);
    g3rs_workspace_crawl::entry(crawl, &rel_path)
        .filter(|entry| entry.kind == G3RsWorkspaceEntryKind::File)
}

/// `select_workflow_entries` function.
pub(crate) fn select_workflow_entries(crawl: &G3RsWorkspaceCrawl) -> Vec<&G3RsWorkspaceEntry> {
    let mut entries = crawl
        .entries
        .iter()
        .filter(|entry| {
            entry.kind == G3RsWorkspaceEntryKind::File
                && entry.path.rel_path.starts_with(".github/workflows/")
                && std::path::Path::new(&entry.path.rel_path)
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .is_some_and(|ext| {
                        ext.eq_ignore_ascii_case("yml") || ext.eq_ignore_ascii_case("yaml")
                    })
        })
        .collect::<Vec<_>>();
    entries.sort_by(|left, right| left.path.rel_path.cmp(&right.path.rel_path));
    entries
}

/// `collect_member_rels` function.
pub(crate) fn collect_member_rels(
    crawl: &G3RsWorkspaceCrawl,
    root_cargo: &CargoToml,
) -> MemberRelsResult {
    let workspace = root_cargo
        .workspace
        .as_ref()
        .ok_or_else(|| "root Cargo.toml is not a workspace".to_owned())?;

    let exclude_patterns = workspace
        .exclude
        .iter()
        .map(|pattern| {
            Pattern::new(pattern)
                .map(|compiled| (pattern.as_str(), compiled))
                .map_err(|err| format!("invalid workspace exclude pattern `{pattern}`: {err}"))
        })
        .collect::<Result<Vec<_>, _>>()?;

    let mut members = std::collections::BTreeSet::new();
    for pattern in &workspace.members {
        for member_rel in expand_member_pattern(crawl, pattern)? {
            if exclude_patterns
                .iter()
                .any(|(_, exclude)| exclude.matches(&member_rel))
            {
                continue;
            }
            let _ = members.insert(member_rel);
        }
    }

    Ok(members.into_iter().collect())
}

/// `expand_member_pattern` function.
fn expand_member_pattern(crawl: &G3RsWorkspaceCrawl, pattern: &str) -> MemberRelsResult {
    let normalized = normalize_member_rel(pattern);
    if looks_like_glob(&normalized) {
        let compiled = Pattern::new(&normalized)
            .map_err(|err| format!("invalid workspace member pattern `{pattern}`: {err}"))?;
        Ok(crawl
            .entries
            .iter()
            .filter(|entry| entry.kind == G3RsWorkspaceEntryKind::Directory)
            .map(|entry| entry.path.rel_path.as_str())
            .filter(|rel| compiled.matches(rel))
            .map(normalize_member_rel)
            .collect())
    } else {
        Ok(vec![normalized])
    }
}

/// `looks_like_glob` function.
fn looks_like_glob(pattern: &str) -> bool {
    pattern.contains('*') || pattern.contains('?') || pattern.contains('[')
}

/// `normalize_member_rel` function.
pub(crate) fn normalize_member_rel(pattern: &str) -> String {
    let trimmed = pattern.trim_matches('/');
    let stripped = trimmed
        .strip_prefix("./")
        .unwrap_or(trimmed)
        .trim_matches('/');

    if stripped == "." {
        String::new()
    } else {
        stripped.to_owned()
    }
}

/// `member_manifest_rel_path` function.
pub(crate) fn member_manifest_rel_path(member_rel: &str) -> String {
    if member_rel.is_empty() {
        "Cargo.toml".to_owned()
    } else {
        format!("{member_rel}/Cargo.toml")
    }
}
