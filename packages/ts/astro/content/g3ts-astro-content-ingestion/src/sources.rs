use g3_workspace_crawl::{
    G3RsWorkspaceCrawl as G3WorkspaceCrawl, G3RsWorkspaceEntryKind as G3WorkspaceEntryKind,
    G3RsWorkspaceIgnoreState as G3WorkspaceIgnoreState,
};
use g3ts_astro_content_types::{
    G3TsAstroContentAdapterSourcePaths, G3TsAstroContentPolicySurfaceState,
};

const SOURCE_MODULE_EXTENSIONS: [&str; 6] = [".ts", ".tsx", ".js", ".jsx", ".mts", ".mjs"];

pub(crate) fn content_adapter_sources(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
    astro_policy: &G3TsAstroContentPolicySurfaceState,
) -> G3TsAstroContentAdapterSourcePaths {
    let content_adapter = content_adapter_source_paths(crawl, app_root_rel_path, astro_policy);
    let content_adapter_astro_content =
        content_adapter_astro_content_source_paths(crawl, app_root_rel_path, &content_adapter);

    G3TsAstroContentAdapterSourcePaths {
        content_adapter,
        content_adapter_astro_content,
    }
}

fn content_adapter_source_paths(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
    astro_policy: &G3TsAstroContentPolicySurfaceState,
) -> Vec<String> {
    let G3TsAstroContentPolicySurfaceState::Parsed { snapshot } = astro_policy else {
        return Vec::new();
    };
    let scoped_adapters: Vec<(String, String)> = snapshot
        .content_adapters
        .iter()
        .map(|adapter| {
            let scoped_adapter = g3ts_astro_check_support::surfaces::scoped_rel_path(
                app_root_rel_path,
                adapter.trim_end_matches('/'),
            );
            let scoped_adapter_prefix = format!("{scoped_adapter}/");
            (scoped_adapter, scoped_adapter_prefix)
        })
        .collect();

    if scoped_adapters.is_empty() {
        return Vec::new();
    }

    crawl
        .entries
        .iter()
        .filter(|entry| {
            entry.kind == G3WorkspaceEntryKind::File
                && entry.ignore_state == G3WorkspaceIgnoreState::Included
                && is_source_module_file(&entry.path.rel_path)
                && scoped_adapters
                    .iter()
                    .any(|(scoped_adapter, scoped_adapter_prefix)| {
                        entry.path.rel_path == *scoped_adapter
                            || entry.path.rel_path.starts_with(scoped_adapter_prefix)
                    })
        })
        .map(|entry| {
            g3ts_astro_check_support::surfaces::app_relative_path(
                &entry.path.rel_path,
                app_root_rel_path,
            )
        })
        .collect()
}

fn content_adapter_astro_content_source_paths(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
    content_adapter_source_paths: &[String],
) -> Vec<String> {
    content_adapter_source_paths
        .iter()
        .filter(|app_relative_path| {
            let rel_path = g3ts_astro_check_support::surfaces::scoped_rel_path(
                app_root_rel_path,
                app_relative_path,
            );
            crawl
                .entries
                .iter()
                .find(|entry| {
                    entry.kind == G3WorkspaceEntryKind::File
                        && entry.ignore_state == G3WorkspaceIgnoreState::Included
                        && entry.path.rel_path == rel_path
                })
                .is_some_and(|entry| {
                    astro_config_parser::module_has_runtime_source_import(
                        &crawl.root_abs_path,
                        &entry.path.rel_path,
                        "astro:content",
                    )
                    .unwrap_or(false)
                })
        })
        .cloned()
        .collect()
}

fn is_source_module_file(rel_path: &str) -> bool {
    SOURCE_MODULE_EXTENSIONS
        .iter()
        .any(|extension| rel_path.ends_with(extension))
}
