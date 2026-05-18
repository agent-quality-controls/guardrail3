use std::collections::BTreeSet;

use g3_workspace_crawl::G3WorkspaceCrawl;

#[must_use]
/// `typecov_roots`: typecov roots.
pub(crate) fn typecov_roots(crawl: &G3WorkspaceCrawl) -> Vec<String> {
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

/// Returns the included file at `rel_path`, if it exists in the crawl.
pub(crate) fn exact_included_file<'a>(
    crawl: &'a G3WorkspaceCrawl,
    rel_path: &str,
) -> Option<&'a g3_workspace_crawl::G3WorkspaceEntry> {
    crawl.entries.iter().find(|entry| {
        entry.kind == g3_workspace_crawl::G3WorkspaceEntryKind::File
            && entry.ignore_state == g3_workspace_crawl::G3WorkspaceIgnoreState::Included
            && entry.path.rel_path == rel_path
    })
}

/// `scoped_rel_path`: scoped rel path.
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

/// `package_manifest`: package manifest.
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

/// `parent_rel_path`: parent rel path.
fn parent_rel_path(rel_path: &str) -> String {
    std::path::Path::new(rel_path)
        .parent()
        .and_then(std::path::Path::to_str)
        .filter(|parent| !parent.is_empty())
        .map_or_else(|| ".".to_owned(), str::to_owned)
}
