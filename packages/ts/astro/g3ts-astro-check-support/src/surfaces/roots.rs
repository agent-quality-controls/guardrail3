use super::package::package_surface_has_astro_dependency;
use super::prelude::*;
use super::constants::*;
use super::syncpack::exact_included_file;
use super::package::ingest_package_surface;

pub fn astro_app_roots(crawl: &G3WorkspaceCrawl) -> Vec<String> {
    let mut roots: BTreeSet<String> = crate::select::select_astro_app_roots(crawl)
        .into_iter()
        .collect();

    for entry in crawl.entries.iter().filter(|entry| {
        entry.kind == g3_workspace_crawl::G3RsWorkspaceEntryKind::File
            && entry.ignore_state == g3_workspace_crawl::G3RsWorkspaceIgnoreState::Included
            && (entry.path.rel_path.ends_with("/package.json")
                || entry.path.rel_path == PACKAGE_JSON_REL_PATH)
    }) {
        let app_root_rel_path = if entry.path.rel_path == PACKAGE_JSON_REL_PATH {
            ".".to_owned()
        } else {
            std::path::Path::new(&entry.path.rel_path)
                .parent()
                .and_then(|parent| parent.to_str())
                .unwrap_or(".")
                .to_owned()
        };

        if package_surface_has_astro_dependency(&ingest_package_surface(crawl, &app_root_rel_path))
        {
            let _ = roots.insert(app_root_rel_path);
        }
    }

    roots.into_iter().collect()
}

pub fn classify_content_mode(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
) -> G3TsAstroContentMode {
    if crate::select::select_live_config(crawl, app_root_rel_path).is_some() {
        G3TsAstroContentMode::LiveCollections
    } else if crate::select::select_content_config(crawl, app_root_rel_path).is_some()
        || crate::select::has_content_files(crawl, app_root_rel_path)
    {
        G3TsAstroContentMode::BuildCollections
    } else {
        G3TsAstroContentMode::None
    }
}

pub fn ingest_astro_policy_surface(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
) -> G3TsAstroPolicySurfaceState {
    let rel_path = scoped_rel_path(app_root_rel_path, GUARDRAIL_CONFIG_REL_PATH);
    let Some(entry) = exact_included_file(crawl, &rel_path) else {
        return G3TsAstroPolicySurfaceState::Missing { rel_path };
    };

    if !entry.readable {
        return G3TsAstroPolicySurfaceState::Unreadable {
            rel_path: entry.path.rel_path.clone(),
            reason: "workspace crawl marked the guardrail config unreadable".to_owned(),
        };
    }

    let config = match guardrail3_rs_toml_parser::from_path(&entry.path.abs_path) {
        Ok(config) => config,
        Err(error) => {
            return G3TsAstroPolicySurfaceState::ParseError {
                rel_path: entry.path.rel_path.clone(),
                reason: error.to_string(),
            };
        }
    };

    let Some(ts) = config.ts else {
        return G3TsAstroPolicySurfaceState::MissingAstroPolicy {
            rel_path: entry.path.rel_path.clone(),
        };
    };
    let Some(astro) = ts.astro else {
        return G3TsAstroPolicySurfaceState::MissingAstroPolicy {
            rel_path: entry.path.rel_path.clone(),
        };
    };

    G3TsAstroPolicySurfaceState::Parsed {
        snapshot: G3TsAstroPolicySnapshot {
            rel_path: entry.path.rel_path.clone(),
            profile: astro.profile,
            content_routes: astro.routes.content,
            non_content_routes: astro.routes.non_content,
            endpoints: astro.routes.endpoints,
            content_root: astro.content.root,
            content_adapters: astro.content.adapters,
            required_collections: astro.content.required_collections,
            collection_fields: astro.content.collection_fields,
            mdx_component_maps: astro.mdx.component_maps,
            metadata_helpers: astro.seo.metadata_helpers,
            json_ld_helpers: astro.seo.json_ld_helpers,
            forbidden_state: astro.state.forbidden,
        },
    }
}

pub(crate) fn scoped_rel_path(app_root_rel_path: &str, rel_path: &str) -> String {
    if app_root_rel_path == "." {
        rel_path.to_owned()
    } else {
        format!("{app_root_rel_path}/{rel_path}")
    }
}

pub(crate) fn app_relative_path(rel_path: &str, app_root_rel_path: &str) -> String {
    if app_root_rel_path == "." {
        rel_path.to_owned()
    } else {
        rel_path
            .strip_prefix(&format!("{app_root_rel_path}/"))
            .unwrap_or(rel_path)
            .to_owned()
    }
}

pub(crate) fn is_under_app_root(rel_path: &str, app_root_rel_path: &str) -> bool {
    app_root_rel_path == "."
        || rel_path == app_root_rel_path
        || rel_path.starts_with(&format!("{app_root_rel_path}/"))
}

pub(crate) fn nearest_app_root<'a>(
    rel_path: &str,
    app_root_rel_paths: &'a [String],
) -> Option<&'a str> {
    app_root_rel_paths
        .iter()
        .filter(|root| is_under_app_root(rel_path, root))
        .max_by_key(|root| root.len())
        .map(String::as_str)
}

pub fn select_llms_txt(crawl: &G3WorkspaceCrawl, app_root_rel_path: &str) -> Option<String> {
    let rel_path = scoped_rel_path(app_root_rel_path, "public/llms.txt");
    exact_included_file(crawl, &rel_path).map(|entry| entry.path.rel_path.clone())
}
