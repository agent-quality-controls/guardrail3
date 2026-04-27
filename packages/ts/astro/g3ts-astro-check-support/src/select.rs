use g3_workspace_crawl::{
    G3RsWorkspaceCrawl as G3WorkspaceCrawl, G3RsWorkspaceEntry as G3WorkspaceEntry,
    G3RsWorkspaceEntryKind as G3WorkspaceEntryKind,
    G3RsWorkspaceIgnoreState as G3WorkspaceIgnoreState,
};
use std::path::Path;

const ROOT_ESLINT_CONFIGS: [&str; 6] = [
    "eslint.config.js",
    "eslint.config.mjs",
    "eslint.config.cjs",
    "eslint.config.ts",
    "eslint.config.mts",
    "eslint.config.cts",
];

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

fn exact_file<'a>(crawl: &'a G3WorkspaceCrawl, rel_path: &str) -> Option<&'a G3WorkspaceEntry> {
    crawl
        .entries
        .iter()
        .find(|entry| entry.path.rel_path == rel_path && entry.kind == G3WorkspaceEntryKind::File)
}

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

fn scoped_rel_path(app_root_rel_path: &str, rel_path: &str) -> String {
    if app_root_rel_path == "." {
        rel_path.to_owned()
    } else {
        format!("{app_root_rel_path}/{rel_path}")
    }
}

fn is_included_file(entry: &G3WorkspaceEntry) -> bool {
    entry.kind == G3WorkspaceEntryKind::File
        && entry.ignore_state == G3WorkspaceIgnoreState::Included
}
