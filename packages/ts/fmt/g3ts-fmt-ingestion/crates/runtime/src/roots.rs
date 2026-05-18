use std::collections::BTreeSet;

use g3_workspace_crawl::G3WorkspaceCrawl;

/// Returns the sorted set of fmt scope roots discovered in `crawl`.
#[must_use]
pub(crate) fn fmt_roots(crawl: &G3WorkspaceCrawl) -> Vec<String> {
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

/// Joins `scope` and `local` into a single relative path.
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

/// Returns the file name when `rel_path` is a recognized Prettier config.
pub(crate) fn prettier_config_name(rel_path: &str) -> Option<&str> {
    let name = rel_path.rsplit('/').next()?;
    let normalized = name.to_ascii_lowercase();
    (normalized == ".prettierrc"
        || normalized.starts_with(".prettierrc.")
        || (normalized.starts_with("prettier.config.")
            && normalized.len() > "prettier.config.".len()))
    .then_some(name)
}

/// Returns true when `entry` is an included `package.json` file.
fn package_manifest(entry: &g3_workspace_crawl::G3WorkspaceEntry) -> bool {
    entry.kind == g3_workspace_crawl::G3WorkspaceEntryKind::File
        && entry.ignore_state == g3_workspace_crawl::G3WorkspaceIgnoreState::Included
        && entry.path.rel_path.rsplit('/').next() == Some("package.json")
}

/// Returns true when `entry` is an included `guardrail3-ts.toml` file.
fn guardrail_config(entry: &g3_workspace_crawl::G3WorkspaceEntry) -> bool {
    entry.kind == g3_workspace_crawl::G3WorkspaceEntryKind::File
        && entry.ignore_state == g3_workspace_crawl::G3WorkspaceIgnoreState::Included
        && entry.path.rel_path.rsplit('/').next() == Some("guardrail3-ts.toml")
}

/// Returns the parent directory of `rel_path` as a relative path, or `.` for top-level files.
fn parent_rel_path(rel_path: &str) -> String {
    std::path::Path::new(rel_path)
        .parent()
        .and_then(std::path::Path::to_str)
        .filter(|parent| !parent.is_empty())
        .map_or_else(|| ".".to_owned(), str::to_owned)
}
