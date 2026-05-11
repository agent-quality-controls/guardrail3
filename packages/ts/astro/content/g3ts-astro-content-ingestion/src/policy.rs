use g3_workspace_crawl::{G3WorkspaceCrawl, G3WorkspaceEntryKind, G3WorkspaceIgnoreState};
use g3ts_astro_content_types::{
    G3TsAstroContentMode, G3TsAstroContentPolicySnapshot, G3TsAstroContentPolicySurfaceState,
};
use std::path::Path;

/// Constant `GUARDRAIL_CONFIG_REL_PATH`.
const GUARDRAIL_CONFIG_REL_PATH: &str = "guardrail3-ts.toml";
/// Constant `CONTENT_CONFIGS`.
const CONTENT_CONFIGS: [&str; 6] = [
    "src/content.config.js",
    "src/content.config.mjs",
    "src/content.config.cjs",
    "src/content.config.ts",
    "src/content.config.mts",
    "src/content.config.cts",
];
/// Constant `VELITE_CONFIGS`.
const VELITE_CONFIGS: [&str; 6] = [
    "velite.config.js",
    "velite.config.mjs",
    "velite.config.cjs",
    "velite.config.ts",
    "velite.config.mts",
    "velite.config.cts",
];

/// Helper `ingest_content_policy_surface`.
pub(crate) fn ingest_content_policy_surface(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
) -> G3TsAstroContentPolicySurfaceState {
    let rel_path = g3ts_astro_check_support::surfaces::scoped_rel_path(
        app_root_rel_path,
        GUARDRAIL_CONFIG_REL_PATH,
    );
    let Some(entry) = crate::roots::exact_included_file(crawl, &rel_path) else {
        return G3TsAstroContentPolicySurfaceState::Missing { rel_path };
    };

    if !entry.readable {
        return G3TsAstroContentPolicySurfaceState::Unreadable {
            rel_path: entry.path.rel_path.clone(),
            reason: "workspace crawl marked the guardrail config unreadable".to_owned(),
        };
    }

    let config = match g3ts_toml_parser::from_path(&entry.path.abs_path) {
        Ok(config) => config,
        Err(error) => {
            return G3TsAstroContentPolicySurfaceState::ParseError {
                rel_path: entry.path.rel_path.clone(),
                reason: error.to_string(),
            };
        }
    };

    let Some(astro) = config.astro else {
        return G3TsAstroContentPolicySurfaceState::MissingAstroPolicy {
            rel_path: entry.path.rel_path.clone(),
        };
    };

    G3TsAstroContentPolicySurfaceState::Parsed {
        snapshot: G3TsAstroContentPolicySnapshot {
            rel_path: entry.path.rel_path.clone(),
            profile: astro.profile,
            content_routes: astro.routes.content,
            non_content_routes: astro.routes.non_content,
            endpoints: astro.routes.endpoints,
            content_root: astro.content.root,
            content_adapters: astro.content.adapters,
            required_collections: astro.content.required_collections,
            collection_fields: astro.content.collection_fields,
        },
    }
}

/// Helper `classify_content_mode`.
pub(crate) fn classify_content_mode(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
) -> G3TsAstroContentMode {
    if crate::roots::select_live_config(crawl, app_root_rel_path).is_some() {
        G3TsAstroContentMode::LiveCollections
    } else if select_content_config(crawl, app_root_rel_path).is_some()
        || has_content_files(crawl, app_root_rel_path)
    {
        G3TsAstroContentMode::BuildCollections
    } else {
        G3TsAstroContentMode::None
    }
}

/// Helper `select_content_config`.
pub(crate) fn select_content_config<'a>(
    crawl: &'a G3WorkspaceCrawl,
    app_root_rel_path: &str,
) -> Option<&'a g3_workspace_crawl::G3WorkspaceEntry> {
    crate::roots::find_first_scoped_included_file(crawl, app_root_rel_path, &CONTENT_CONFIGS)
}

/// Helper `select_velite_config`.
pub(crate) fn select_velite_config<'a>(
    crawl: &'a G3WorkspaceCrawl,
    app_root_rel_path: &str,
) -> Option<&'a g3_workspace_crawl::G3WorkspaceEntry> {
    crawl
        .entries
        .iter()
        .filter(|entry| {
            is_included_file(entry)
                && g3ts_astro_check_support::surfaces::is_under_app_root(
                    &entry.path.rel_path,
                    app_root_rel_path,
                )
                && !is_route_tree_path(&entry.path.rel_path, app_root_rel_path)
                && Path::new(&entry.path.rel_path)
                    .file_name()
                    .and_then(|name| name.to_str())
                    .is_some_and(|file_name| VELITE_CONFIGS.contains(&file_name))
        })
        .min_by(|left, right| left.path.rel_path.cmp(&right.path.rel_path))
}

/// Helper `route_page_paths`.
pub(crate) fn route_page_paths(crawl: &G3WorkspaceCrawl, app_root_rel_path: &str) -> Vec<String> {
    let pages_prefix =
        g3ts_astro_check_support::surfaces::scoped_rel_path(app_root_rel_path, "src/pages/");

    crawl
        .entries
        .iter()
        .filter(|entry| {
            is_included_file(entry)
                && entry.path.rel_path.starts_with(&pages_prefix)
                && is_route_page_file(&entry.path.rel_path)
        })
        .map(|entry| {
            g3ts_astro_check_support::surfaces::app_relative_path(
                &entry.path.rel_path,
                app_root_rel_path,
            )
        })
        .collect()
}

/// Helper `endpoint_paths`.
pub(crate) fn endpoint_paths(crawl: &G3WorkspaceCrawl, app_root_rel_path: &str) -> Vec<String> {
    let pages_prefix =
        g3ts_astro_check_support::surfaces::scoped_rel_path(app_root_rel_path, "src/pages/");

    crawl
        .entries
        .iter()
        .filter(|entry| {
            is_included_file(entry)
                && entry.path.rel_path.starts_with(&pages_prefix)
                && is_endpoint_file(&entry.path.rel_path)
        })
        .map(|entry| {
            g3ts_astro_check_support::surfaces::app_relative_path(
                &entry.path.rel_path,
                app_root_rel_path,
            )
        })
        .collect()
}

/// Helper `route_markdown_pages`.
pub(crate) fn route_markdown_pages(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
) -> Vec<String> {
    let pages_prefix =
        g3ts_astro_check_support::surfaces::scoped_rel_path(app_root_rel_path, "src/pages/");

    crawl
        .entries
        .iter()
        .filter(|entry| {
            is_included_file(entry)
                && entry.path.rel_path.starts_with(&pages_prefix)
                && (has_extension_ascii_ci(&entry.path.rel_path, "md")
                    || has_extension_ascii_ci(&entry.path.rel_path, "mdx"))
        })
        .map(|entry| entry.path.rel_path.clone())
        .collect()
}

/// Helper `velite_output_paths`.
pub(crate) fn velite_output_paths(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
    app_root_rel_paths: &[String],
) -> Vec<String> {
    crawl
        .entries
        .iter()
        .filter(|entry| {
            is_app_visible(entry)
                && g3ts_astro_check_support::surfaces::is_under_app_root(
                    &entry.path.rel_path,
                    app_root_rel_path,
                )
                && g3ts_astro_check_support::surfaces::nearest_app_root(
                    &entry.path.rel_path,
                    app_root_rel_paths,
                )
                .is_some_and(|nearest| nearest == app_root_rel_path)
                && path_has_segment(&entry.path.rel_path, ".velite")
        })
        .map(|entry| entry.path.rel_path.clone())
        .collect()
}

/// Helper `has_content_files`.
fn has_content_files(crawl: &G3WorkspaceCrawl, app_root_rel_path: &str) -> bool {
    let prefix =
        g3ts_astro_check_support::surfaces::scoped_rel_path(app_root_rel_path, "src/content/");

    crawl.entries.iter().any(|entry| {
        is_included_file(entry)
            && entry.path.rel_path.starts_with(&prefix)
            && entry.kind == G3WorkspaceEntryKind::File
    })
}

/// Helper `is_included_file`.
fn is_included_file(entry: &g3_workspace_crawl::G3WorkspaceEntry) -> bool {
    entry.kind == G3WorkspaceEntryKind::File
        && entry.ignore_state == G3WorkspaceIgnoreState::Included
}

/// Helper `is_app_visible`.
const fn is_app_visible(entry: &g3_workspace_crawl::G3WorkspaceEntry) -> bool {
    entry.readable
        && matches!(
            entry.kind,
            G3WorkspaceEntryKind::File | G3WorkspaceEntryKind::Directory
        )
}

/// Helper `is_route_tree_path`.
fn is_route_tree_path(rel_path: &str, app_root_rel_path: &str) -> bool {
    let pages_prefix =
        g3ts_astro_check_support::surfaces::scoped_rel_path(app_root_rel_path, "src/pages/");
    rel_path.starts_with(&pages_prefix)
}

/// Helper `is_route_page_file`.
fn is_route_page_file(rel_path: &str) -> bool {
    has_extension_ascii_ci(rel_path, "astro")
        || has_extension_ascii_ci(rel_path, "md")
        || has_extension_ascii_ci(rel_path, "mdx")
        || has_extension_ascii_ci(rel_path, "html")
}

/// Helper `is_endpoint_file`.
fn is_endpoint_file(rel_path: &str) -> bool {
    (has_extension_ascii_ci(rel_path, "js") || has_extension_ascii_ci(rel_path, "ts"))
        && !rel_path.to_ascii_lowercase().ends_with(".d.ts")
}

/// Returns `true` when `rel_path` has the given `extension`, comparing the
/// extension byte-wise in an ASCII-case-insensitive manner.
fn has_extension_ascii_ci(rel_path: &str, extension: &str) -> bool {
    Path::new(rel_path)
        .extension()
        .is_some_and(|actual| actual.eq_ignore_ascii_case(extension))
}

/// Helper `path_has_segment`.
fn path_has_segment(rel_path: &str, segment: &str) -> bool {
    Path::new(rel_path)
        .components()
        .any(|component| component.as_os_str() == segment)
}
