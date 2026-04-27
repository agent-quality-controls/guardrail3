use g3_workspace_crawl::{
    G3RsWorkspaceCrawl as G3WorkspaceCrawl, G3RsWorkspaceEntryKind as G3WorkspaceEntryKind,
    G3RsWorkspaceIgnoreState as G3WorkspaceIgnoreState,
};
use g3ts_astro_state_types::{G3TsAstroStatePolicySnapshot, G3TsAstroStatePolicySurfaceState};
use std::path::Path;

const GUARDRAIL_CONFIG_REL_PATH: &str = "guardrail3-ts.toml";
const CONTENT_CONFIGS: [&str; 6] = [
    "src/content.config.js",
    "src/content.config.mjs",
    "src/content.config.cjs",
    "src/content.config.ts",
    "src/content.config.mts",
    "src/content.config.cts",
];
const LIVE_CONFIGS: [&str; 6] = [
    "src/live.config.js",
    "src/live.config.mjs",
    "src/live.config.cjs",
    "src/live.config.ts",
    "src/live.config.mts",
    "src/live.config.cts",
];
const CONTENTLAYER_CONFIGS: [&str; 6] = [
    "contentlayer.config.js",
    "contentlayer.config.mjs",
    "contentlayer.config.cjs",
    "contentlayer.config.ts",
    "contentlayer.config.mts",
    "contentlayer.config.cts",
];

pub(crate) fn ingest_state_policy_surface(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
) -> G3TsAstroStatePolicySurfaceState {
    let rel_path = g3ts_astro_check_support::surfaces::scoped_rel_path(
        app_root_rel_path,
        GUARDRAIL_CONFIG_REL_PATH,
    );
    let Some(entry) = exact_included_file(crawl, &rel_path) else {
        return G3TsAstroStatePolicySurfaceState::Missing { rel_path };
    };

    if !entry.readable {
        return G3TsAstroStatePolicySurfaceState::Unreadable {
            rel_path: entry.path.rel_path.clone(),
            reason: "workspace crawl marked the guardrail config unreadable".to_owned(),
        };
    }

    let config = match guardrail3_rs_toml_parser::from_path(&entry.path.abs_path) {
        Ok(config) => config,
        Err(error) => {
            return G3TsAstroStatePolicySurfaceState::ParseError {
                rel_path: entry.path.rel_path.clone(),
                reason: error.to_string(),
            };
        }
    };

    let Some(ts) = config.ts else {
        return G3TsAstroStatePolicySurfaceState::MissingAstroPolicy {
            rel_path: entry.path.rel_path.clone(),
        };
    };
    let Some(astro) = ts.astro else {
        return G3TsAstroStatePolicySurfaceState::MissingAstroPolicy {
            rel_path: entry.path.rel_path.clone(),
        };
    };

    G3TsAstroStatePolicySurfaceState::Parsed {
        snapshot: G3TsAstroStatePolicySnapshot {
            rel_path: entry.path.rel_path.clone(),
            forbidden_state: astro.state.forbidden,
        },
    }
}

pub(crate) fn has_strict_astro_state_boundary(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
) -> bool {
    select_live_config(crawl, app_root_rel_path).is_some()
        || select_content_config(crawl, app_root_rel_path).is_some()
        || has_content_files(crawl, app_root_rel_path)
}

pub(crate) fn legacy_generated_state_paths(
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
                && is_legacy_generated_state_path(&entry.path.rel_path, app_root_rel_path)
        })
        .map(|entry| entry.path.rel_path.clone())
        .collect()
}

fn select_content_config<'a>(
    crawl: &'a G3WorkspaceCrawl,
    app_root_rel_path: &str,
) -> Option<&'a g3_workspace_crawl::G3RsWorkspaceEntry> {
    CONTENT_CONFIGS.iter().find_map(|rel_path| {
        exact_included_file(
            crawl,
            &g3ts_astro_check_support::surfaces::scoped_rel_path(app_root_rel_path, rel_path),
        )
    })
}

fn select_live_config<'a>(
    crawl: &'a G3WorkspaceCrawl,
    app_root_rel_path: &str,
) -> Option<&'a g3_workspace_crawl::G3RsWorkspaceEntry> {
    LIVE_CONFIGS.iter().find_map(|rel_path| {
        exact_included_file(
            crawl,
            &g3ts_astro_check_support::surfaces::scoped_rel_path(app_root_rel_path, rel_path),
        )
    })
}

fn has_content_files(crawl: &G3WorkspaceCrawl, app_root_rel_path: &str) -> bool {
    let prefix =
        g3ts_astro_check_support::surfaces::scoped_rel_path(app_root_rel_path, "src/content/");

    crawl.entries.iter().any(|entry| {
        is_included_file(entry)
            && entry.path.rel_path.starts_with(&prefix)
            && entry.kind == G3WorkspaceEntryKind::File
    })
}

fn exact_included_file<'crawl>(
    crawl: &'crawl G3WorkspaceCrawl,
    rel_path: &str,
) -> Option<&'crawl g3_workspace_crawl::G3RsWorkspaceEntry> {
    crawl.entries.iter().find(|entry| {
        entry.kind == G3WorkspaceEntryKind::File
            && entry.ignore_state == G3WorkspaceIgnoreState::Included
            && entry.path.rel_path == rel_path
    })
}

fn is_included_file(entry: &g3_workspace_crawl::G3RsWorkspaceEntry) -> bool {
    entry.kind == G3WorkspaceEntryKind::File
        && entry.ignore_state == G3WorkspaceIgnoreState::Included
}

fn is_app_visible(entry: &g3_workspace_crawl::G3RsWorkspaceEntry) -> bool {
    entry.readable
        && matches!(
            entry.kind,
            G3WorkspaceEntryKind::File | G3WorkspaceEntryKind::Directory
        )
}

fn is_route_tree_path(rel_path: &str, app_root_rel_path: &str) -> bool {
    let pages_prefix =
        g3ts_astro_check_support::surfaces::scoped_rel_path(app_root_rel_path, "src/pages/");
    rel_path.starts_with(&pages_prefix)
}

fn is_legacy_generated_state_path(rel_path: &str, app_root_rel_path: &str) -> bool {
    path_has_segment(rel_path, ".next")
        || path_has_segment(rel_path, ".contentlayer")
        || (!is_route_tree_path(rel_path, app_root_rel_path)
            && Path::new(rel_path)
                .file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|file_name| CONTENTLAYER_CONFIGS.contains(&file_name)))
}

fn path_has_segment(rel_path: &str, segment: &str) -> bool {
    Path::new(rel_path)
        .components()
        .any(|component| component.as_os_str() == segment)
}
