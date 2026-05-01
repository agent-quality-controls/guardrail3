use std::collections::BTreeSet;

use g3_workspace_crawl::G3RsWorkspaceCrawl as G3WorkspaceCrawl;

#[must_use]
pub(crate) fn fmt_roots(crawl: &G3WorkspaceCrawl) -> Vec<String> {
    let mut roots = BTreeSet::new();
    for entry in &crawl.entries {
        if package_manifest(entry) || prettier_config_name(&entry.path.rel_path).is_some() {
            let _inserted = roots.insert(parent_rel_path(&entry.path.rel_path));
        }
    }
    roots.into_iter().collect()
}

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

pub(crate) fn prettier_config_name(rel_path: &str) -> Option<&str> {
    let name = rel_path.rsplit('/').next()?;
    matches!(
        name,
        "prettier.config.js"
            | "prettier.config.cjs"
            | "prettier.config.mjs"
            | ".prettierrc"
            | ".prettierrc.json"
            | ".prettierrc.yaml"
            | ".prettierrc.yml"
            | ".prettierrc.toml"
    )
    .then_some(name)
}

fn package_manifest(entry: &g3_workspace_crawl::G3RsWorkspaceEntry) -> bool {
    entry.kind == g3_workspace_crawl::G3RsWorkspaceEntryKind::File
        && entry.ignore_state == g3_workspace_crawl::G3RsWorkspaceIgnoreState::Included
        && entry.path.rel_path.ends_with("package.json")
}

fn parent_rel_path(rel_path: &str) -> String {
    std::path::Path::new(rel_path)
        .parent()
        .and_then(std::path::Path::to_str)
        .filter(|parent| !parent.is_empty())
        .map_or_else(|| ".".to_owned(), str::to_owned)
}
