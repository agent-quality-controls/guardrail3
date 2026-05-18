use std::collections::BTreeSet;

use g3_workspace_crawl::G3WorkspaceCrawl;

/// Enumerate the relative directory paths within `crawl` that should be
/// inspected for spelling configuration (each `package.json` or cspell
/// config marks a candidate root).
#[must_use]
pub(crate) fn spelling_roots(crawl: &G3WorkspaceCrawl) -> Vec<String> {
    let mut package_roots = BTreeSet::new();
    let mut guardrail_roots = BTreeSet::new();
    for entry in &crawl.entries {
        if package_manifest(entry) {
            let _inserted = package_roots.insert(parent_rel_path(&entry.path.rel_path));
        }
        if guardrail_config(entry) {
            let _inserted = guardrail_roots.insert(parent_rel_path(&entry.path.rel_path));
        }
    }
    package_roots
        .intersection(&guardrail_roots)
        .cloned()
        .collect()
}

/// Join `local` onto `scope` to produce a workspace-relative path, treating
/// an empty or `"."` scope as the workspace root.
pub(crate) fn scoped_rel_path(scope: &str, local: &str) -> String {
    if scope.is_empty() || scope == "." {
        return local.to_owned();
    }
    format!(
        "{}/{}",
        scope.trim_end_matches('/'),
        local.trim_start_matches('/')
    )
}

/// Return the cspell config filename of `rel_path` if it matches any of the
/// supported cspell config filenames (JSON or YAML variants).
pub(crate) fn cspell_config_name(rel_path: &str) -> Option<&str> {
    let name = rel_path.rsplit('/').next()?;
    let normalized = name.to_ascii_lowercase();
    (matches!(
        normalized.as_str(),
        "cspell.json" | ".cspell.json" | "cspell.yaml" | "cspell.yml"
    ) || (normalized.starts_with("cspell.config.") && normalized.len() > "cspell.config.".len()))
    .then_some(name)
}

/// Return the cspell config filename of `rel_path` if it matches a
/// JSON-style cspell config name (used when JSON-only logic applies).
pub(crate) fn cspell_json_config_name(rel_path: &str) -> Option<&str> {
    let name = rel_path.rsplit('/').next()?;
    let normalized = name.to_ascii_lowercase();
    matches!(
        normalized.as_str(),
        "cspell.json" | "cspell.config.json" | ".cspell.json"
    )
    .then_some(name)
}

/// Whether `entry` is an included file named `package.json`.
fn package_manifest(entry: &g3_workspace_crawl::G3WorkspaceEntry) -> bool {
    entry.kind == g3_workspace_crawl::G3WorkspaceEntryKind::File
        && entry.ignore_state == g3_workspace_crawl::G3WorkspaceIgnoreState::Included
        && entry.path.rel_path.rsplit('/').next() == Some("package.json")
}

/// Whether `entry` is an included `guardrail3-ts.toml` file.
fn guardrail_config(entry: &g3_workspace_crawl::G3WorkspaceEntry) -> bool {
    entry.kind == g3_workspace_crawl::G3WorkspaceEntryKind::File
        && entry.ignore_state == g3_workspace_crawl::G3WorkspaceIgnoreState::Included
        && entry.path.rel_path.rsplit('/').next() == Some("guardrail3-ts.toml")
}

/// Return the parent directory of `rel_path` as a workspace-relative path,
/// using `.` for the workspace root.
fn parent_rel_path(rel_path: &str) -> String {
    std::path::Path::new(rel_path)
        .parent()
        .and_then(std::path::Path::to_str)
        .filter(|parent| !parent.is_empty())
        .map_or_else(|| ".".to_owned(), str::to_owned)
}
