use g3_workspace_crawl::{G3WorkspaceCrawl, G3WorkspaceEntryKind, G3WorkspaceIgnoreState};
use g3ts_astro_mdx_types::{G3TsAstroMdxApprovedSourcePaths, G3TsAstroMdxPolicySurfaceState};

/// `SOURCE_MODULE_EXTENSIONS` constant.
const SOURCE_MODULE_EXTENSIONS: [&str; 9] = [
    ".ts", ".tsx", ".js", ".jsx", ".mts", ".cts", ".mjs", ".cjs", ".astro",
];

/// `mdx_component_map_sources` helper.
pub(crate) fn mdx_component_map_sources(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
    astro_policy: &G3TsAstroMdxPolicySurfaceState,
) -> G3TsAstroMdxApprovedSourcePaths {
    let policy_paths = match astro_policy {
        G3TsAstroMdxPolicySurfaceState::Parsed { snapshot } => snapshot.mdx_component_maps.clone(),
        G3TsAstroMdxPolicySurfaceState::Missing { .. }
        | G3TsAstroMdxPolicySurfaceState::Unreadable { .. }
        | G3TsAstroMdxPolicySurfaceState::ParseError { .. }
        | G3TsAstroMdxPolicySurfaceState::MissingAstroPolicy { .. } => Vec::new(),
    };
    let (mdx_component_maps, missing_mdx_component_maps) =
        policy_module_source_paths(crawl, app_root_rel_path, &policy_paths);

    G3TsAstroMdxApprovedSourcePaths {
        mdx_component_maps,
        missing_mdx_component_maps,
    }
}

/// Pair of (resolved-source-paths, unresolved-policy-paths) returned by `policy_module_source_paths`.
type PolicyModulePaths = (Vec<String>, Vec<String>);

/// `policy_module_source_paths` helper.
fn policy_module_source_paths(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
    policy_paths: &[String],
) -> PolicyModulePaths {
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

/// `source_paths_under_policy_path` helper.
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

/// `is_source_module_file` helper.
fn is_source_module_file(rel_path: &str) -> bool {
    SOURCE_MODULE_EXTENSIONS
        .iter()
        .any(|extension| rel_path.ends_with(extension))
}
