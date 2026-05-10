use g3_workspace_crawl::{
    G3RsWorkspaceCrawl as G3WorkspaceCrawl, G3RsWorkspaceEntry as G3WorkspaceEntry,
    G3RsWorkspaceEntryKind as G3WorkspaceEntryKind,
    G3RsWorkspaceIgnoreState as G3WorkspaceIgnoreState,
};
use std::path::Path;

/// File names recognized as a root `ESLint` config (flat-config formats).
const ROOT_ESLINT_CONFIGS: [&str; 6] = [
    "eslint.config.js",
    "eslint.config.mjs",
    "eslint.config.cjs",
    "eslint.config.ts",
    "eslint.config.mts",
    "eslint.config.cts",
];

/// Walk from `app_root_rel_path` up to the repo root, returning the nearest active `ESLint` config entry.
pub(crate) fn select_active_eslint_config<'a>(
    crawl: &'a G3WorkspaceCrawl,
    app_root_rel_path: &str,
) -> Option<&'a G3WorkspaceEntry> {
    ancestor_rel_paths(app_root_rel_path)
        .into_iter()
        .find_map(|candidate_root| {
            ROOT_ESLINT_CONFIGS.iter().find_map(|file_name| {
                exact_file(crawl, &scoped_rel_path(&candidate_root, file_name))
                    .filter(|entry| is_included_file(entry))
            })
        })
}

/// Look up a file entry by exact `rel_path`.
fn exact_file<'a>(crawl: &'a G3WorkspaceCrawl, rel_path: &str) -> Option<&'a G3WorkspaceEntry> {
    crawl
        .entries
        .iter()
        .find(|entry| entry.path.rel_path == rel_path && entry.kind == G3WorkspaceEntryKind::File)
}

/// Return `app_root_rel_path` and each ancestor directory up to the repo root (".").
fn ancestor_rel_paths(app_root_rel_path: &str) -> Vec<String> {
    let mut ancestors = Vec::new();
    let mut current = app_root_rel_path.to_owned();

    loop {
        ancestors.push(current.clone());

        if current == "." {
            break;
        }

        current = parent_rel_path(&current);
    }

    ancestors
}

/// Return the parent directory of `rel_path`, mapping the root parent to ".".
fn parent_rel_path(rel_path: &str) -> String {
    let Some(parent) = Path::new(rel_path)
        .parent()
        .and_then(|parent| parent.to_str())
    else {
        return ".".to_owned();
    };

    if parent.is_empty() {
        ".".to_owned()
    } else {
        parent.to_owned()
    }
}

/// Join `app_root_rel_path` with `rel_path`, treating "." as the repo root.
fn scoped_rel_path(app_root_rel_path: &str, rel_path: &str) -> String {
    if app_root_rel_path == "." {
        rel_path.to_owned()
    } else {
        format!("{app_root_rel_path}/{rel_path}")
    }
}

/// Whether `entry` is a file that has not been ignored by the workspace crawl.
fn is_included_file(entry: &G3WorkspaceEntry) -> bool {
    entry.kind == G3WorkspaceEntryKind::File
        && entry.ignore_state == G3WorkspaceIgnoreState::Included
}
