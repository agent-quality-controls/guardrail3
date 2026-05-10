use g3_workspace_crawl::{
    G3RsWorkspaceCrawl as G3WorkspaceCrawl, G3RsWorkspaceEntryKind as G3WorkspaceEntryKind,
    G3RsWorkspaceIgnoreState as G3WorkspaceIgnoreState,
};
use g3ts_astro_seo_types::{G3TsAstroSeoApprovedSourcePaths, G3TsAstroSeoPolicySurfaceState};

/// `SOURCE_MODULE_EXTENSIONS` constant.
const SOURCE_MODULE_EXTENSIONS: [&str; 9] = [
    ".ts", ".tsx", ".js", ".jsx", ".mts", ".cts", ".mjs", ".cjs", ".astro",
];

/// `seo_helper_sources`: seo helper sources.
pub(crate) fn seo_helper_sources(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
    astro_policy: &G3TsAstroSeoPolicySurfaceState,
) -> G3TsAstroSeoApprovedSourcePaths {
    let (metadata_policy_paths, json_ld_policy_paths) = match astro_policy {
        G3TsAstroSeoPolicySurfaceState::Parsed { snapshot } => (
            snapshot.metadata_helpers.clone(),
            snapshot.json_ld_helpers.clone(),
        ),
        G3TsAstroSeoPolicySurfaceState::Missing { .. }
        | G3TsAstroSeoPolicySurfaceState::Unreadable { .. }
        | G3TsAstroSeoPolicySurfaceState::ParseError { .. }
        | G3TsAstroSeoPolicySurfaceState::MissingAstroPolicy { .. } => (Vec::new(), Vec::new()),
    };
    let (metadata_helpers, missing_metadata_helpers) =
        policy_module_source_paths(crawl, app_root_rel_path, &metadata_policy_paths);
    let (json_ld_helpers, missing_json_ld_helpers) =
        policy_module_source_paths(crawl, app_root_rel_path, &json_ld_policy_paths);

    G3TsAstroSeoApprovedSourcePaths {
        metadata_helpers,
        missing_metadata_helpers,
        json_ld_helpers,
        missing_json_ld_helpers,
    }
}

/// Pair of (found-source-paths, missing-policy-paths) returned by `policy_module_source_paths`.
type FoundAndMissingPaths = (Vec<String>, Vec<String>);

/// `policy_module_source_paths`: policy module source paths.
fn policy_module_source_paths(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
    policy_paths: &[String],
) -> FoundAndMissingPaths {
    let mut source_paths = Vec::new();
    let mut missing_policy_paths = Vec::new();

    for policy_path in policy_paths {
        let paths = source_paths_under_policy_path(crawl, app_root_rel_path, policy_path);
        if paths.is_empty() {
            missing_policy_paths.push(policy_path.clone());
        } else {
            source_paths.extend(paths);
        }
    }

    (source_paths, missing_policy_paths)
}

/// `source_paths_under_policy_path`: source paths under policy path.
fn source_paths_under_policy_path(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
    policy_path: &str,
) -> Vec<String> {
    let scoped_path = g3ts_astro_check_support::surfaces::scoped_rel_path(
        app_root_rel_path,
        policy_path.trim_end_matches('/'),
    );
    let scoped_prefix = format!("{scoped_path}/");

    crawl
        .entries
        .iter()
        .filter(|entry| {
            entry.kind == G3WorkspaceEntryKind::File
                && entry.ignore_state == G3WorkspaceIgnoreState::Included
                && is_source_module_file(&entry.path.rel_path)
                && (entry.path.rel_path == scoped_path
                    || entry.path.rel_path.starts_with(&scoped_prefix))
        })
        .map(|entry| {
            g3ts_astro_check_support::surfaces::app_relative_path(
                &entry.path.rel_path,
                app_root_rel_path,
            )
        })
        .collect()
}

/// `is_source_module_file`: is source module file.
fn is_source_module_file(rel_path: &str) -> bool {
    SOURCE_MODULE_EXTENSIONS
        .iter()
        .any(|extension| rel_path.ends_with(extension))
}
