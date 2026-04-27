use g3_workspace_crawl::{
    G3RsWorkspaceCrawl as G3WorkspaceCrawl, G3RsWorkspaceEntry as G3WorkspaceEntry,
    G3RsWorkspaceEntryKind as G3WorkspaceEntryKind,
    G3RsWorkspaceIgnoreState as G3WorkspaceIgnoreState,
};
use std::collections::BTreeSet;
use std::path::Path;

const ROOT_ASTRO_CONFIGS: [&str; 6] = [
    "astro.config.js",
    "astro.config.mjs",
    "astro.config.cjs",
    "astro.config.ts",
    "astro.config.mts",
    "astro.config.cts",
];
#[must_use]
pub(crate) fn astro_app_roots(crawl: &G3WorkspaceCrawl) -> Vec<String> {
    let mut roots = BTreeSet::new();

    for entry in crawl.entries.iter().filter(|entry| is_included_file(entry)) {
        if root_astro_config_file(&entry.path.rel_path) {
            let _ = roots.insert(parent_rel_path(&entry.path.rel_path));
        }

        if package_json_file(&entry.path.rel_path) {
            let app_root_rel_path = parent_rel_path(&entry.path.rel_path);
            if package_has_astro_dependency(entry) {
                let _ = roots.insert(app_root_rel_path);
            }
        }
    }

    roots.into_iter().collect()
}

fn is_included_file(entry: &G3WorkspaceEntry) -> bool {
    entry.kind == G3WorkspaceEntryKind::File
        && entry.ignore_state == G3WorkspaceIgnoreState::Included
}

fn root_astro_config_file(rel_path: &str) -> bool {
    Path::new(rel_path)
        .file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|file_name| ROOT_ASTRO_CONFIGS.contains(&file_name))
}

fn package_json_file(rel_path: &str) -> bool {
    Path::new(rel_path)
        .file_name()
        .and_then(|name| name.to_str())
        == Some("package.json")
}

fn parent_rel_path(rel_path: &str) -> String {
    Path::new(rel_path)
        .parent()
        .and_then(|parent| parent.to_str())
        .filter(|parent| !parent.is_empty())
        .unwrap_or(".")
        .to_owned()
}

fn package_has_astro_dependency(entry: &G3WorkspaceEntry) -> bool {
    if !entry.readable {
        return false;
    }

    let Ok(document) = package_json_parser::from_path_document(&entry.path.abs_path) else {
        return false;
    };
    let Some(typed) = package_json_parser::typed(&document) else {
        return false;
    };

    typed
        .dependencies
        .iter()
        .chain(typed.dev_dependencies.iter())
        .any(|dependency| dependency == "astro")
}
