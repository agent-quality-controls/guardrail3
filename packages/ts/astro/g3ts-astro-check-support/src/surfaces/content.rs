use super::eslint_options::glob_set_from_strings;
use super::syncpack::exact_included_file;
use super::prelude::*;
use super::constants::*;
use super::roots::{
    app_relative_path, astro_app_roots, classify_content_mode, ingest_astro_policy_surface,
    is_under_app_root, nearest_app_root, scoped_rel_path,
};

pub(super) fn content_adapter_source_paths(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
    astro_policy: &G3TsAstroPolicySurfaceState,
) -> Vec<String> {
    let G3TsAstroPolicySurfaceState::Parsed { snapshot } = astro_policy else {
        return Vec::new();
    };
    let scoped_adapters: Vec<(String, String)> = snapshot
        .content_adapters
        .iter()
        .map(|adapter| {
            let scoped_adapter = scoped_rel_path(app_root_rel_path, adapter.trim_end_matches('/'));
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
            entry.kind == g3_workspace_crawl::G3RsWorkspaceEntryKind::File
                && entry.ignore_state == g3_workspace_crawl::G3RsWorkspaceIgnoreState::Included
                && is_adapter_source_file(&entry.path.rel_path)
                && scoped_adapters
                    .iter()
                    .any(|(scoped_adapter, scoped_adapter_prefix)| {
                        entry.path.rel_path == *scoped_adapter
                            || entry.path.rel_path.starts_with(scoped_adapter_prefix)
                    })
        })
        .map(|entry| app_relative_path(&entry.path.rel_path, app_root_rel_path))
        .collect()
}

pub(super) fn content_adapter_policy_paths(astro_policy: &G3TsAstroPolicySurfaceState) -> Vec<String> {
    let G3TsAstroPolicySurfaceState::Parsed { snapshot } = astro_policy else {
        return Vec::new();
    };

    snapshot
        .content_adapters
        .iter()
        .map(|path| path.to_owned())
        .collect()
}

pub(super) fn policy_configured_paths(
    astro_policy: &G3TsAstroPolicySurfaceState,
    select_paths: fn(&G3TsAstroPolicySnapshot) -> &Vec<String>,
) -> Vec<String> {
    let G3TsAstroPolicySurfaceState::Parsed { snapshot } = astro_policy else {
        return Vec::new();
    };

    select_paths(snapshot).clone()
}

fn policy_module_source_paths(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
    astro_policy: &G3TsAstroPolicySurfaceState,
    select_paths: fn(&G3TsAstroPolicySnapshot) -> &Vec<String>,
) -> G3TsAstroPolicyModuleSourcePaths {
    let G3TsAstroPolicySurfaceState::Parsed { snapshot } = astro_policy else {
        return G3TsAstroPolicyModuleSourcePaths {
            source_paths: Vec::new(),
            missing_policy_paths: Vec::new(),
        };
    };

    let mut source_paths = Vec::new();
    let mut missing_policy_paths = Vec::new();

    for policy_path in select_paths(snapshot) {
        let paths = source_paths_under_policy_path(crawl, app_root_rel_path, policy_path);
        if paths.is_empty() {
            missing_policy_paths.push(policy_path.clone());
        } else {
            source_paths.extend(paths);
        }
    }

    G3TsAstroPolicyModuleSourcePaths {
        source_paths,
        missing_policy_paths,
    }
}

#[must_use]
pub fn content_adapter_sources(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
    astro_policy: &G3TsAstroPolicySurfaceState,
) -> G3TsAstroContentAdapterSourcePaths {
    let content_adapter = content_adapter_source_paths(crawl, app_root_rel_path, astro_policy);
    let content_adapter_astro_content =
        content_adapter_astro_content_source_paths(crawl, app_root_rel_path, &content_adapter);

    G3TsAstroContentAdapterSourcePaths {
        content_adapter,
        content_adapter_astro_content,
    }
}

#[must_use]
pub fn mdx_component_map_sources(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
    astro_policy: &G3TsAstroPolicySurfaceState,
) -> G3TsAstroMdxApprovedSourcePaths {
    let mdx_component_maps = policy_module_source_paths(
        crawl,
        app_root_rel_path,
        astro_policy,
        |policy| &policy.mdx_component_maps,
    );

    G3TsAstroMdxApprovedSourcePaths {
        mdx_component_maps: mdx_component_maps.source_paths,
        missing_mdx_component_maps: mdx_component_maps.missing_policy_paths,
    }
}

#[must_use]
pub fn seo_helper_sources(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
    astro_policy: &G3TsAstroPolicySurfaceState,
) -> G3TsAstroSeoApprovedSourcePaths {
    let metadata_helpers = policy_module_source_paths(
        crawl,
        app_root_rel_path,
        astro_policy,
        |policy| &policy.metadata_helpers,
    );
    let json_ld_helpers = policy_module_source_paths(
        crawl,
        app_root_rel_path,
        astro_policy,
        |policy| &policy.json_ld_helpers,
    );

    G3TsAstroSeoApprovedSourcePaths {
        metadata_helpers: metadata_helpers.source_paths,
        missing_metadata_helpers: metadata_helpers.missing_policy_paths,
        json_ld_helpers: json_ld_helpers.source_paths,
        missing_json_ld_helpers: json_ld_helpers.missing_policy_paths,
    }
}

fn source_paths_under_policy_path(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
    policy_path: &str,
) -> Vec<String> {
    let scoped_path = scoped_rel_path(app_root_rel_path, policy_path.trim_end_matches('/'));
    let scoped_prefix = format!("{scoped_path}/");

    crawl
        .entries
        .iter()
        .filter(|entry| {
            entry.kind == g3_workspace_crawl::G3RsWorkspaceEntryKind::File
                && entry.ignore_state == g3_workspace_crawl::G3RsWorkspaceIgnoreState::Included
                && is_adapter_source_file(&entry.path.rel_path)
                && (entry.path.rel_path == scoped_path
                    || entry.path.rel_path.starts_with(&scoped_prefix))
        })
        .map(|entry| app_relative_path(&entry.path.rel_path, app_root_rel_path))
        .collect()
}

fn forbidden_state_paths(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
    app_root_rel_paths: &[String],
    astro_policy: &G3TsAstroPolicySurfaceState,
) -> Vec<String> {
    let G3TsAstroPolicySurfaceState::Parsed { snapshot } = astro_policy else {
        return Vec::new();
    };
    let Ok(globs) = glob_set_from_strings(&snapshot.forbidden_state) else {
        return Vec::new();
    };

    crawl
        .entries
        .iter()
        .filter(|entry| {
            entry.readable
                && matches!(
                    entry.kind,
                    g3_workspace_crawl::G3RsWorkspaceEntryKind::File
                        | g3_workspace_crawl::G3RsWorkspaceEntryKind::Directory
                )
                && is_under_app_root(&entry.path.rel_path, app_root_rel_path)
                && nearest_app_root(&entry.path.rel_path, app_root_rel_paths)
                    .is_some_and(|nearest| nearest == app_root_rel_path)
                && globs.is_match(app_relative_path(&entry.path.rel_path, app_root_rel_path))
        })
        .map(|entry| entry.path.rel_path.clone())
        .collect()
}

#[must_use]
pub fn app_root_input(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
    app_root_rel_paths: &[String],
) -> G3TsAstroAppRootInput {
    let astro_policy = ingest_astro_policy_surface(crawl, app_root_rel_path);
    G3TsAstroAppRootInput {
        app_root_rel_path: app_root_rel_path.to_owned(),
        astro_config_rel_path: crate::select::select_astro_config(crawl, app_root_rel_path)
            .map(|entry| entry.path.rel_path.clone()),
        content_config_rel_path: crate::select::select_content_config(
            crawl,
            app_root_rel_path,
        )
        .map(|entry| entry.path.rel_path.clone()),
        live_config_rel_path: crate::select::select_live_config(crawl, app_root_rel_path)
            .map(|entry| entry.path.rel_path.clone()),
        velite_config_rel_path: crate::select::select_velite_config(
            crawl,
            app_root_rel_path,
        )
        .map(|entry| entry.path.rel_path.clone()),
        velite_output_rel_paths: crate::select::velite_output_paths(
            crawl,
            app_root_rel_path,
            app_root_rel_paths,
        ),
        legacy_generated_state_rel_paths: crate::select::legacy_generated_state_paths(
            crawl,
            app_root_rel_path,
            app_root_rel_paths,
        ),
        forbidden_state_rel_paths: forbidden_state_paths(
            crawl,
            app_root_rel_path,
            app_root_rel_paths,
            &astro_policy,
        ),
    }
}

#[must_use]
pub fn app_root_inputs(crawl: &G3WorkspaceCrawl) -> Vec<G3TsAstroAppRootInput> {
    let app_root_rel_paths = astro_app_roots(crawl);
    app_root_rel_paths
        .iter()
        .map(|app_root_rel_path| app_root_input(crawl, app_root_rel_path, &app_root_rel_paths))
        .collect()
}

#[must_use]
pub fn build_collection_roots(
    crawl: &G3WorkspaceCrawl,
    roots: &[G3TsAstroAppRootInput],
) -> Vec<G3TsAstroAppRootInput> {
    roots
        .iter()
        .filter(|root| {
            classify_content_mode(crawl, &root.app_root_rel_path)
                == G3TsAstroContentMode::BuildCollections
        })
        .cloned()
        .collect()
}

#[must_use]
pub fn live_collection_roots(
    crawl: &G3WorkspaceCrawl,
    roots: &[G3TsAstroAppRootInput],
) -> Vec<G3TsAstroAppRootInput> {
    roots
        .iter()
        .filter(|root| {
            classify_content_mode(crawl, &root.app_root_rel_path)
                == G3TsAstroContentMode::LiveCollections
        })
        .cloned()
        .collect()
}

#[must_use]
pub fn route_markdown_page_inputs(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_paths: &[String],
) -> Vec<G3TsAstroRouteMarkdownPageInput> {
    app_root_rel_paths
        .iter()
        .flat_map(|app_root_rel_path| {
            crate::select::route_markdown_pages(crawl, app_root_rel_path)
        })
        .map(|rel_path| G3TsAstroRouteMarkdownPageInput { rel_path })
        .collect()
}

fn is_adapter_source_file(rel_path: &str) -> bool {
    is_source_module_file(rel_path)
}

pub(super) fn is_source_module_file(rel_path: &str) -> bool {
    SOURCE_MODULE_EXTENSIONS
        .iter()
        .any(|extension| rel_path.ends_with(extension))
}

pub(super) fn content_adapter_astro_content_source_paths(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
    content_adapter_source_paths: &[String],
) -> Vec<String> {
    content_adapter_source_paths
        .iter()
        .filter(|app_relative_path| {
            let rel_path = scoped_rel_path(app_root_rel_path, app_relative_path);
            exact_included_file(crawl, &rel_path).is_some_and(|entry| {
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
