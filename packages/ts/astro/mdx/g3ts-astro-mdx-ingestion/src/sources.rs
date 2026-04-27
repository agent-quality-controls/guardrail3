use g3_workspace_crawl::{
    G3RsWorkspaceCrawl as G3WorkspaceCrawl, G3RsWorkspaceEntryKind as G3WorkspaceEntryKind,
    G3RsWorkspaceIgnoreState as G3WorkspaceIgnoreState,
};
use g3ts_astro_mdx_types::{G3TsAstroMdxApprovedSourcePaths, G3TsAstroMdxPolicySurfaceState};

const SOURCE_MODULE_EXTENSIONS: [&str; 9] = [
    ".ts", ".tsx", ".js", ".jsx", ".mts", ".cts", ".mjs", ".cjs", ".astro",
];

pub(crate) fn mdx_component_map_sources(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
    astro_policy: &G3TsAstroMdxPolicySurfaceState,
) -> G3TsAstroMdxApprovedSourcePaths {
    let policy_paths = match astro_policy {
        G3TsAstroMdxPolicySurfaceState::Parsed { snapshot } => snapshot.mdx_component_maps.clone(),
        _ => Vec::new(),
    };
    let (mdx_component_maps, missing_mdx_component_maps) =
        policy_module_source_paths(crawl, app_root_rel_path, &policy_paths);

    G3TsAstroMdxApprovedSourcePaths {
        mdx_component_maps,
        missing_mdx_component_maps,
    }
}

fn policy_module_source_paths(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
    policy_paths: &[String],
) -> (Vec<String>, Vec<String>) {
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

fn is_source_module_file(rel_path: &str) -> bool {
    SOURCE_MODULE_EXTENSIONS
        .iter()
        .any(|extension| rel_path.ends_with(extension))
}
