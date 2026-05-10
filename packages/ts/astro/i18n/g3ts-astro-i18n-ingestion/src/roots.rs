use g3_workspace_crawl::{
    G3RsWorkspaceCrawl as G3WorkspaceCrawl, G3RsWorkspaceEntry as G3WorkspaceEntry,
    G3RsWorkspaceEntryKind as G3WorkspaceEntryKind,
    G3RsWorkspaceIgnoreState as G3WorkspaceIgnoreState,
};
use g3ts_astro_i18n_types::G3TsAstroPackageSurfaceState;
use std::collections::BTreeSet;
use std::path::Path;

/// File names recognised as a root Astro config in any supported source extension.
const ROOT_ASTRO_CONFIGS: [&str; 6] = [
    "astro.config.js",
    "astro.config.mjs",
    "astro.config.cjs",
    "astro.config.ts",
    "astro.config.mts",
    "astro.config.cts",
];

/// Returns the relative paths of Astro app roots discovered in the workspace crawl.
#[must_use]
pub(crate) fn astro_app_roots(crawl: &G3WorkspaceCrawl) -> Vec<String> {
    let mut roots = BTreeSet::new();

    for entry in crawl.entries.iter().filter(|entry| is_included_file(entry)) {
        if root_astro_config_file(&entry.path.rel_path) {
            let _ = roots.insert(parent_rel_path(&entry.path.rel_path));
        }

        if package_json_file(&entry.path.rel_path) {
            let app_root_rel_path = parent_rel_path(&entry.path.rel_path);
            let package = crate::package::ingest_package_surface(crawl, &app_root_rel_path);
            if package_has_astro_dependency(&package) {
                let _ = roots.insert(app_root_rel_path);
            }
        }
    }

    roots.into_iter().collect()
}

/// Returns true when the entry is a regular file included by ignore rules.
pub(crate) fn is_included_file(entry: &G3WorkspaceEntry) -> bool {
    entry.kind == G3WorkspaceEntryKind::File
        && entry.ignore_state == G3WorkspaceIgnoreState::Included
}

/// Returns true when `rel_path`'s file name is a recognised root Astro config.
fn root_astro_config_file(rel_path: &str) -> bool {
    Path::new(rel_path)
        .file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|file_name| ROOT_ASTRO_CONFIGS.contains(&file_name))
}

/// Returns true when `rel_path` ends in a `package.json` file name.
fn package_json_file(rel_path: &str) -> bool {
    Path::new(rel_path)
        .file_name()
        .and_then(|name| name.to_str())
        == Some("package.json")
}

/// Returns the parent directory rel path (or "." when at the workspace root).
fn parent_rel_path(rel_path: &str) -> String {
    Path::new(rel_path)
        .parent()
        .and_then(|parent| parent.to_str())
        .filter(|parent| !parent.is_empty())
        .unwrap_or(".")
        .to_owned()
}

/// Returns true when the parsed `package.json` declares an `astro` dependency.
fn package_has_astro_dependency(package: &G3TsAstroPackageSurfaceState) -> bool {
    match package {
        G3TsAstroPackageSurfaceState::Parsed { snapshot } => snapshot
            .dependencies
            .iter()
            .chain(snapshot.dev_dependencies.iter())
            .any(|dependency| dependency == "astro"),
        G3TsAstroPackageSurfaceState::Missing { .. }
        | G3TsAstroPackageSurfaceState::Unreadable { .. }
        | G3TsAstroPackageSurfaceState::ParseError { .. } => false,
    }
}
