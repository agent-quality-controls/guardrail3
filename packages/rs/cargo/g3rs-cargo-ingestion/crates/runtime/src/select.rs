use g3_workspace_crawl::{G3WorkspaceCrawl, G3WorkspaceEntry, G3WorkspaceEntryKind};
use glob::Pattern;

/// Alias for fallible normalization helpers returning member-rel strings.
type StringVecResult = Result<Vec<String>, String>;

/// select root cargo toml fn.
pub(crate) fn select_root_cargo_toml(crawl: &G3WorkspaceCrawl) -> Option<&G3WorkspaceEntry> {
    g3_workspace_crawl::root_file(crawl, "Cargo.toml")
}

/// select root rust policy toml fn.
pub(crate) fn select_root_rust_policy_toml(crawl: &G3WorkspaceCrawl) -> Option<&G3WorkspaceEntry> {
    g3_workspace_crawl::root_file(crawl, "guardrail3-rs.toml")
}

/// select member manifest fn.
pub(crate) fn select_member_manifest<'a>(
    crawl: &'a G3WorkspaceCrawl,
    member_rel: &str,
) -> Option<&'a G3WorkspaceEntry> {
    let rel_path = member_manifest_rel_path(member_rel);
    g3_workspace_crawl::entry(crawl, &rel_path)
        .filter(|entry| entry.kind == G3WorkspaceEntryKind::File)
}

/// collect declared member rels fn.
pub(crate) fn collect_declared_member_rels(
    crawl: &G3WorkspaceCrawl,
    root_raw: &toml::Value,
) -> StringVecResult {
    let member_patterns = parse_string_array(
        root_raw
            .get("workspace")
            .and_then(|value| value.get("members")),
        "[workspace].members",
    )?;
    let exclude_patterns = parse_string_array(
        root_raw
            .get("workspace")
            .and_then(|value| value.get("exclude")),
        "[workspace].exclude",
    )?;
    let exclude_patterns = exclude_patterns
        .iter()
        .map(|pattern| {
            Pattern::new(pattern)
                .map(|compiled| (pattern.as_str(), compiled))
                .map_err(|err| format!("invalid workspace exclude pattern `{pattern}`: {err}"))
        })
        .collect::<Result<Vec<_>, _>>()?;

    let mut members = std::collections::BTreeSet::new();
    for pattern in member_patterns {
        for member_rel in expand_member_pattern(crawl, &pattern)? {
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

/// workspace root kind fn.
pub(crate) fn workspace_root_kind(
    root_raw: &toml::Value,
) -> g3rs_cargo_types::G3RsCargoPolicyRootKind {
    if root_raw.get("workspace").is_some() {
        g3rs_cargo_types::G3RsCargoPolicyRootKind::WorkspaceRoot
    } else if root_raw.get("package").is_some() {
        g3rs_cargo_types::G3RsCargoPolicyRootKind::StandalonePackageRoot
    } else {
        g3rs_cargo_types::G3RsCargoPolicyRootKind::Other
    }
}

/// parse string array fn.
fn parse_string_array(value: Option<&toml::Value>, label: &str) -> StringVecResult {
    let Some(value) = value else {
        return Ok(Vec::new());
    };
    let Some(array) = value.as_array() else {
        return Err(format!("{label} must be an array of strings."));
    };
    array
        .iter()
        .map(|item| {
            item.as_str()
                .map(str::to_owned)
                .ok_or_else(|| format!("{label} must contain only string entries."))
        })
        .collect()
}

/// expand member pattern fn.
fn expand_member_pattern(crawl: &G3WorkspaceCrawl, pattern: &str) -> StringVecResult {
    let normalized = normalize_member_rel(pattern);
    if looks_like_glob(&normalized) {
        let compiled = Pattern::new(&normalized)
            .map_err(|err| format!("invalid workspace member pattern `{pattern}`: {err}"))?;
        Ok(crawl
            .entries
            .iter()
            .filter(|entry| entry.kind == G3WorkspaceEntryKind::Directory)
            .map(|entry| entry.path.rel_path.as_str())
            .filter(|rel| compiled.matches(rel))
            .map(normalize_member_rel)
            .collect())
    } else {
        Ok(vec![normalized])
    }
}

/// looks like glob fn.
fn looks_like_glob(pattern: &str) -> bool {
    pattern.contains('*') || pattern.contains('?') || pattern.contains('[')
}

/// normalize member rel fn.
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

/// member manifest rel path fn.
pub(crate) fn member_manifest_rel_path(member_rel: &str) -> String {
    if member_rel.is_empty() {
        "Cargo.toml".to_owned()
    } else {
        format!("{member_rel}/Cargo.toml")
    }
}
