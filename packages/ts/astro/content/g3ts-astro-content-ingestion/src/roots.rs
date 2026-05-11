use g3_workspace_crawl::{
    G3WorkspaceCrawl, G3WorkspaceEntry, G3WorkspaceEntryKind, G3WorkspaceIgnoreState,
};
use g3ts_astro_content_types::G3TsAstroPackageSurfaceState;
use std::collections::BTreeSet;
use std::path::Path;

/// Constant `ROOT_ASTRO_CONFIGS`.
const ROOT_ASTRO_CONFIGS: [&str; 6] = [
    "astro.config.js",
    "astro.config.mjs",
    "astro.config.cjs",
    "astro.config.ts",
    "astro.config.mts",
    "astro.config.cts",
];
/// Constant `LIVE_CONFIGS`.
const LIVE_CONFIGS: [&str; 6] = [
    "src/live.config.js",
    "src/live.config.mjs",
    "src/live.config.cjs",
    "src/live.config.ts",
    "src/live.config.mts",
    "src/live.config.cts",
];

/// Discover the set of Astro app roots present in `crawl`, deduplicating by
/// path and returning them in stable order.
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

/// Select the live config file for the app rooted at `app_root_rel_path`.
pub(crate) fn select_live_config<'a>(
    crawl: &'a G3WorkspaceCrawl,
    app_root_rel_path: &str,
) -> Option<&'a G3WorkspaceEntry> {
    find_first_scoped_included_file(crawl, app_root_rel_path, &LIVE_CONFIGS)
}

/// Look up the workspace entry that exactly matches `rel_path` and whose
/// crawl ignore state is `Included`.
pub(crate) fn exact_included_file<'crawl>(
    crawl: &'crawl G3WorkspaceCrawl,
    rel_path: &str,
) -> Option<&'crawl G3WorkspaceEntry> {
    crawl.entries.iter().find(|entry| {
        entry.kind == G3WorkspaceEntryKind::File
            && entry.ignore_state == G3WorkspaceIgnoreState::Included
            && entry.path.rel_path == rel_path
    })
}

/// Find the first entry whose path equals `app_root_rel_path` joined to one
/// of the candidate `rel_paths`, returning the matching workspace entry.
pub(crate) fn find_first_scoped_included_file<'a>(
    crawl: &'a G3WorkspaceCrawl,
    app_root_rel_path: &str,
    rel_paths: &[&str],
) -> Option<&'a G3WorkspaceEntry> {
    rel_paths.iter().find_map(|rel_path| {
        exact_included_file(
            crawl,
            &g3ts_astro_check_support::surfaces::scoped_rel_path(app_root_rel_path, rel_path),
        )
    })
}

/// Helper `is_included_file`.
fn is_included_file(entry: &G3WorkspaceEntry) -> bool {
    entry.kind == G3WorkspaceEntryKind::File
        && entry.ignore_state == G3WorkspaceIgnoreState::Included
}

/// Helper `root_astro_config_file`.
fn root_astro_config_file(rel_path: &str) -> bool {
    Path::new(rel_path)
        .file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|file_name| ROOT_ASTRO_CONFIGS.contains(&file_name))
}

/// Helper `package_json_file`.
fn package_json_file(rel_path: &str) -> bool {
    Path::new(rel_path)
        .file_name()
        .and_then(|name| name.to_str())
        == Some("package.json")
}

/// Helper `parent_rel_path`.
fn parent_rel_path(rel_path: &str) -> String {
    Path::new(rel_path)
        .parent()
        .and_then(|parent| parent.to_str())
        .filter(|parent| !parent.is_empty())
        .unwrap_or(".")
        .to_owned()
}

/// Helper `package_has_astro_dependency`.
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
