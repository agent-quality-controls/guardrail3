use eslint_config_parser::types::{EslintProbeKind, EslintProbeTarget};
use g3_workspace_crawl::{
    G3RsWorkspaceCrawl as G3WorkspaceCrawl, G3RsWorkspaceEntry as G3WorkspaceEntry,
    G3RsWorkspaceEntryKind as G3WorkspaceEntryKind,
    G3RsWorkspaceIgnoreState as G3WorkspaceIgnoreState,
};
use std::collections::BTreeSet;
use std::path::Path;

const ROOT_ESLINT_CONFIGS: [&str; 6] = [
    "eslint.config.js",
    "eslint.config.mjs",
    "eslint.config.cjs",
    "eslint.config.ts",
    "eslint.config.mts",
    "eslint.config.cts",
];

const ROOT_ASTRO_CONFIGS: [&str; 6] = [
    "astro.config.js",
    "astro.config.mjs",
    "astro.config.cjs",
    "astro.config.ts",
    "astro.config.mts",
    "astro.config.cts",
];

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

pub(crate) fn select_astro_app_roots(crawl: &G3WorkspaceCrawl) -> Vec<String> {
    let mut roots = BTreeSet::new();

    for entry in &crawl.entries {
        if !is_included_file(entry) {
            continue;
        }

        let Some(file_name) = Path::new(&entry.path.rel_path).file_name().and_then(|name| name.to_str()) else {
            continue;
        };

        if ROOT_ASTRO_CONFIGS.contains(&file_name) {
            let _ = roots.insert(parent_rel_path(&entry.path.rel_path));
        }
    }

    roots.into_iter().collect()
}

pub(crate) fn select_package_json<'a>(
    crawl: &'a G3WorkspaceCrawl,
    app_root_rel_path: &str,
) -> Option<&'a G3WorkspaceEntry> {
    exact_file(crawl, &scoped_rel_path(app_root_rel_path, "package.json"))
        .filter(|entry| is_included_file(entry))
}

pub(crate) fn select_astro_config<'a>(
    crawl: &'a G3WorkspaceCrawl,
    app_root_rel_path: &str,
) -> Option<&'a G3WorkspaceEntry> {
    ROOT_ASTRO_CONFIGS
        .iter()
        .find_map(|file_name| {
            exact_file(crawl, &scoped_rel_path(app_root_rel_path, file_name))
                .filter(|entry| is_included_file(entry))
        })
}

pub(crate) fn select_content_config<'a>(
    crawl: &'a G3WorkspaceCrawl,
    app_root_rel_path: &str,
) -> Option<&'a G3WorkspaceEntry> {
    CONTENT_CONFIGS
        .iter()
        .find_map(|rel_path| {
            exact_file(crawl, &scoped_rel_path(app_root_rel_path, rel_path))
                .filter(|entry| is_included_file(entry))
        })
}

pub(crate) fn select_live_config<'a>(
    crawl: &'a G3WorkspaceCrawl,
    app_root_rel_path: &str,
) -> Option<&'a G3WorkspaceEntry> {
    LIVE_CONFIGS
        .iter()
        .find_map(|rel_path| {
            exact_file(crawl, &scoped_rel_path(app_root_rel_path, rel_path))
                .filter(|entry| is_included_file(entry))
        })
}

pub(crate) fn has_content_files(crawl: &G3WorkspaceCrawl, app_root_rel_path: &str) -> bool {
    let prefix = scoped_rel_path(app_root_rel_path, "src/content/");

    crawl.entries.iter().any(|entry| {
        is_included_file(entry)
            && entry.path.rel_path.starts_with(&prefix)
            && entry.kind == G3WorkspaceEntryKind::File
    })
}

pub(crate) fn route_markdown_pages(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
) -> Vec<String> {
    let pages_prefix = scoped_rel_path(app_root_rel_path, "src/pages/");

    crawl.entries
        .iter()
        .filter(|entry| {
            is_included_file(entry)
                && entry.kind == G3WorkspaceEntryKind::File
                && entry.path.rel_path.starts_with(&pages_prefix)
                && (entry.path.rel_path.ends_with(".md") || entry.path.rel_path.ends_with(".mdx"))
        })
        .map(|entry| entry.path.rel_path.clone())
        .collect()
}

pub(crate) fn select_active_eslint_config<'a>(
    crawl: &'a G3WorkspaceCrawl,
    app_root_rel_path: &str,
) -> Option<&'a G3WorkspaceEntry> {
    ancestor_rel_paths(app_root_rel_path).into_iter().find_map(|candidate_root| {
        ROOT_ESLINT_CONFIGS.iter().find_map(|file_name| {
            exact_file(crawl, &scoped_rel_path(&candidate_root, file_name))
                .filter(|entry| is_included_file(entry))
        })
    })
}

pub(crate) fn probe_targets(
    crawl: &G3WorkspaceCrawl,
    config_rel_path: &str,
) -> Vec<EslintProbeTarget> {
    let config_scope = config_scope_dir(config_rel_path);
    let mut probes = Vec::new();

    if let Some(rel_path) = first_astro_source_rel_path(crawl, config_scope) {
        probes.push(probe(EslintProbeKind::AstroSource, rel_path));
    }

    probes.push(probe(
        EslintProbeKind::TsSource,
        first_ts_source_rel_path(crawl, config_scope)
            .unwrap_or_else(|| scoped_default_rel_path(config_scope, "src/index.ts")),
    ));

    if let Some(rel_path) = first_tsx_source_rel_path(crawl, config_scope) {
        probes.push(probe(EslintProbeKind::TsxSource, rel_path));
    }

    probes.push(probe(
        EslintProbeKind::ConfigFile,
        config_rel_path.to_owned(),
    ));
    probes
}

fn exact_file<'a>(crawl: &'a G3WorkspaceCrawl, rel_path: &str) -> Option<&'a G3WorkspaceEntry> {
    crawl.entries
        .iter()
        .find(|entry| entry.path.rel_path == rel_path && entry.kind == G3WorkspaceEntryKind::File)
}

fn ancestor_rel_paths(app_root_rel_path: &str) -> Vec<String> {
    let mut ancestors = Vec::new();
    let mut current = app_root_rel_path.to_owned();

    loop {
        ancestors.push(current.clone());

        if current == "." {
            break;
        }

        current = parent_rel_path(&current);
    }

    ancestors
}

fn parent_rel_path(rel_path: &str) -> String {
    let Some(parent) = Path::new(rel_path).parent().and_then(|parent| parent.to_str()) else {
        return ".".to_owned();
    };

    if parent.is_empty() {
        ".".to_owned()
    } else {
        parent.to_owned()
    }
}

fn scoped_rel_path(app_root_rel_path: &str, rel_path: &str) -> String {
    if app_root_rel_path == "." {
        rel_path.to_owned()
    } else {
        format!("{app_root_rel_path}/{rel_path}")
    }
}

fn first_ts_source_rel_path(
    crawl: &G3WorkspaceCrawl,
    config_scope: Option<&str>,
) -> Option<String> {
    first_matching_rel_path(crawl, config_scope, is_primary_ts_source_rel_path)
        .or_else(|| first_matching_rel_path(crawl, config_scope, is_fallback_ts_source_rel_path))
        .or_else(|| first_tsx_source_rel_path(crawl, config_scope))
}

fn first_tsx_source_rel_path(
    crawl: &G3WorkspaceCrawl,
    config_scope: Option<&str>,
) -> Option<String> {
    first_matching_rel_path(crawl, config_scope, is_primary_tsx_source_rel_path)
        .or_else(|| first_matching_rel_path(crawl, config_scope, is_fallback_tsx_source_rel_path))
}

fn first_astro_source_rel_path(
    crawl: &G3WorkspaceCrawl,
    config_scope: Option<&str>,
) -> Option<String> {
    first_matching_rel_path(crawl, config_scope, is_primary_astro_source_rel_path)
        .or_else(|| first_matching_rel_path(crawl, config_scope, is_fallback_astro_source_rel_path))
}

fn first_matching_rel_path(
    crawl: &G3WorkspaceCrawl,
    config_scope: Option<&str>,
    predicate: impl Fn(&str) -> bool,
) -> Option<String> {
    crawl.entries
        .iter()
        .find(|entry| {
            is_included_file(entry)
                && in_config_scope(&entry.path.rel_path, config_scope)
                && predicate(&entry.path.rel_path)
        })
        .map(|entry| entry.path.rel_path.clone())
}

fn config_scope_dir(config_rel_path: &str) -> Option<&str> {
    let parent = std::path::Path::new(config_rel_path).parent()?;
    let parent = parent.to_str()?;
    if parent.is_empty() { None } else { Some(parent) }
}

fn in_config_scope(rel_path: &str, config_scope: Option<&str>) -> bool {
    let Some(config_scope) = config_scope else {
        return true;
    };
    rel_path == config_scope || rel_path.starts_with(&format!("{config_scope}/"))
}

fn scoped_default_rel_path(config_scope: Option<&str>, default_rel_path: &str) -> String {
    match config_scope {
        Some(config_scope) => format!("{config_scope}/{default_rel_path}"),
        None => default_rel_path.to_owned(),
    }
}

fn probe(probe: EslintProbeKind, rel_path: String) -> EslintProbeTarget {
    EslintProbeTarget { probe, rel_path }
}

fn is_included_file(entry: &G3WorkspaceEntry) -> bool {
    entry.kind == G3WorkspaceEntryKind::File
        && entry.ignore_state == G3WorkspaceIgnoreState::Included
}

fn is_primary_ts_source_rel_path(rel_path: &str) -> bool {
    rel_path.starts_with("src/")
        && rel_path.ends_with(".ts")
        && !rel_path.ends_with(".d.ts")
        && !rel_path.ends_with(".test.ts")
        && !rel_path.ends_with(".spec.ts")
        && !is_config_like_rel_path(rel_path)
}

fn is_primary_tsx_source_rel_path(rel_path: &str) -> bool {
    rel_path.starts_with("src/")
        && rel_path.ends_with(".tsx")
        && !rel_path.ends_with(".test.tsx")
        && !rel_path.ends_with(".spec.tsx")
        && !is_config_like_rel_path(rel_path)
}

fn is_fallback_ts_source_rel_path(rel_path: &str) -> bool {
    rel_path.ends_with(".ts")
        && !rel_path.ends_with(".d.ts")
        && !rel_path.contains("/node_modules/")
        && !is_config_like_rel_path(rel_path)
}

fn is_fallback_tsx_source_rel_path(rel_path: &str) -> bool {
    rel_path.ends_with(".tsx")
        && !rel_path.contains("/node_modules/")
        && !is_config_like_rel_path(rel_path)
}

fn is_primary_astro_source_rel_path(rel_path: &str) -> bool {
    rel_path.starts_with("src/") && rel_path.ends_with(".astro") && !is_config_like_rel_path(rel_path)
}

fn is_fallback_astro_source_rel_path(rel_path: &str) -> bool {
    rel_path.ends_with(".astro")
        && !rel_path.contains("/node_modules/")
        && !is_config_like_rel_path(rel_path)
}

fn is_config_like_rel_path(rel_path: &str) -> bool {
    std::path::Path::new(rel_path)
        .file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.contains(".config."))
}
