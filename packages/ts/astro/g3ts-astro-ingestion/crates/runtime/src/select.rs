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

const VELITE_CONFIGS: [&str; 6] = [
    "velite.config.js",
    "velite.config.mjs",
    "velite.config.cjs",
    "velite.config.ts",
    "velite.config.mts",
    "velite.config.cts",
];

pub(crate) fn select_astro_app_roots(crawl: &G3WorkspaceCrawl) -> Vec<String> {
    let mut roots = BTreeSet::new();

    for entry in &crawl.entries {
        if !is_included_file(entry) {
            continue;
        }

        let Some(file_name) = Path::new(&entry.path.rel_path)
            .file_name()
            .and_then(|name| name.to_str())
        else {
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
    ROOT_ASTRO_CONFIGS.iter().find_map(|file_name| {
        exact_file(crawl, &scoped_rel_path(app_root_rel_path, file_name))
            .filter(|entry| is_included_file(entry))
    })
}

pub(crate) fn select_content_config<'a>(
    crawl: &'a G3WorkspaceCrawl,
    app_root_rel_path: &str,
) -> Option<&'a G3WorkspaceEntry> {
    CONTENT_CONFIGS.iter().find_map(|rel_path| {
        exact_file(crawl, &scoped_rel_path(app_root_rel_path, rel_path))
            .filter(|entry| is_included_file(entry))
    })
}

pub(crate) fn select_live_config<'a>(
    crawl: &'a G3WorkspaceCrawl,
    app_root_rel_path: &str,
) -> Option<&'a G3WorkspaceEntry> {
    LIVE_CONFIGS.iter().find_map(|rel_path| {
        exact_file(crawl, &scoped_rel_path(app_root_rel_path, rel_path))
            .filter(|entry| is_included_file(entry))
    })
}

pub(crate) fn select_velite_config<'a>(
    crawl: &'a G3WorkspaceCrawl,
    app_root_rel_path: &str,
) -> Option<&'a G3WorkspaceEntry> {
    crawl
        .entries
        .iter()
        .filter(|entry| {
            is_included_file(entry)
                && entry.kind == G3WorkspaceEntryKind::File
                && is_under_app_root(&entry.path.rel_path, app_root_rel_path)
                && !is_route_tree_path(&entry.path.rel_path, app_root_rel_path)
                && Path::new(&entry.path.rel_path)
                    .file_name()
                    .and_then(|name| name.to_str())
                    .is_some_and(|file_name| VELITE_CONFIGS.contains(&file_name))
        })
        .min_by(|left, right| left.path.rel_path.cmp(&right.path.rel_path))
}

pub(crate) fn velite_output_paths(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
) -> Vec<String> {
    crawl
        .entries
        .iter()
        .filter(|entry| {
            is_included_file(entry)
                && entry.kind == G3WorkspaceEntryKind::File
                && is_under_app_root(&entry.path.rel_path, app_root_rel_path)
                && path_has_segment(&entry.path.rel_path, ".velite")
        })
        .map(|entry| entry.path.rel_path.clone())
        .collect()
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

    crawl
        .entries
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

pub(crate) fn route_page_paths(crawl: &G3WorkspaceCrawl, app_root_rel_path: &str) -> Vec<String> {
    let pages_prefix = scoped_rel_path(app_root_rel_path, "src/pages/");

    crawl
        .entries
        .iter()
        .filter(|entry| {
            is_included_file(entry)
                && entry.kind == G3WorkspaceEntryKind::File
                && entry.path.rel_path.starts_with(&pages_prefix)
                && is_route_page_file(&entry.path.rel_path)
        })
        .map(|entry| app_relative_path(&entry.path.rel_path, app_root_rel_path))
        .collect()
}

pub(crate) fn endpoint_paths(crawl: &G3WorkspaceCrawl, app_root_rel_path: &str) -> Vec<String> {
    let pages_prefix = scoped_rel_path(app_root_rel_path, "src/pages/");

    crawl
        .entries
        .iter()
        .filter(|entry| {
            is_included_file(entry)
                && entry.kind == G3WorkspaceEntryKind::File
                && entry.path.rel_path.starts_with(&pages_prefix)
                && is_endpoint_file(&entry.path.rel_path)
        })
        .map(|entry| app_relative_path(&entry.path.rel_path, app_root_rel_path))
        .collect()
}

pub(crate) fn select_active_eslint_config<'a>(
    crawl: &'a G3WorkspaceCrawl,
    app_root_rel_path: &str,
) -> Option<&'a G3WorkspaceEntry> {
    ancestor_rel_paths(app_root_rel_path)
        .into_iter()
        .find_map(|candidate_root| {
            ROOT_ESLINT_CONFIGS.iter().find_map(|file_name| {
                exact_file(crawl, &scoped_rel_path(&candidate_root, file_name))
                    .filter(|entry| is_included_file(entry))
            })
        })
}

pub(crate) fn probe_targets(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
    config_rel_path: &str,
) -> Vec<EslintProbeTarget> {
    let mut probes = Vec::new();

    probes.push(probe(
        EslintProbeKind::AstroSource,
        first_astro_source_rel_path(crawl, app_root_rel_path)
            .unwrap_or_else(|| scoped_rel_path(app_root_rel_path, "src/__g3ts_probe__.astro")),
    ));

    probes.push(probe(
        EslintProbeKind::TsSource,
        first_ts_source_rel_path(crawl, app_root_rel_path)
            .unwrap_or_else(|| scoped_rel_path(app_root_rel_path, "src/index.ts")),
    ));

    probes.push(probe(
        EslintProbeKind::TsxSource,
        first_tsx_source_rel_path(crawl, app_root_rel_path)
            .unwrap_or_else(|| scoped_rel_path(app_root_rel_path, "src/__g3ts_probe__.tsx")),
    ));

    probes.push(probe(
        EslintProbeKind::MdxContent,
        first_mdx_content_rel_path(crawl, app_root_rel_path)
            .unwrap_or_else(|| scoped_rel_path(app_root_rel_path, "content/__g3ts_probe__.mdx")),
    ));

    probes.push(probe(
        EslintProbeKind::ConfigFile,
        config_rel_path.to_owned(),
    ));
    probes
}

fn exact_file<'a>(crawl: &'a G3WorkspaceCrawl, rel_path: &str) -> Option<&'a G3WorkspaceEntry> {
    crawl
        .entries
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
    let Some(parent) = Path::new(rel_path)
        .parent()
        .and_then(|parent| parent.to_str())
    else {
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

fn first_ts_source_rel_path(crawl: &G3WorkspaceCrawl, app_root_rel_path: &str) -> Option<String> {
    first_matching_app_rel_path(crawl, app_root_rel_path, is_primary_ts_source_rel_path)
}

fn is_under_app_root(rel_path: &str, app_root_rel_path: &str) -> bool {
    app_root_rel_path == "."
        || rel_path == app_root_rel_path
        || rel_path.starts_with(&format!("{app_root_rel_path}/"))
}

fn path_has_segment(rel_path: &str, segment: &str) -> bool {
    Path::new(rel_path)
        .components()
        .any(|component| component.as_os_str() == segment)
}

fn app_relative_path(rel_path: &str, app_root_rel_path: &str) -> String {
    if app_root_rel_path == "." {
        rel_path.to_owned()
    } else {
        rel_path
            .strip_prefix(&format!("{app_root_rel_path}/"))
            .unwrap_or(rel_path)
            .to_owned()
    }
}

fn is_route_tree_path(rel_path: &str, app_root_rel_path: &str) -> bool {
    let pages_prefix = scoped_rel_path(app_root_rel_path, "src/pages/");
    rel_path.starts_with(&pages_prefix)
}

fn is_route_page_file(rel_path: &str) -> bool {
    rel_path.ends_with(".astro")
        || rel_path.ends_with(".md")
        || rel_path.ends_with(".mdx")
        || rel_path.ends_with(".html")
}

fn is_endpoint_file(rel_path: &str) -> bool {
    (rel_path.ends_with(".js") || rel_path.ends_with(".ts")) && !rel_path.ends_with(".d.ts")
}

fn first_tsx_source_rel_path(crawl: &G3WorkspaceCrawl, app_root_rel_path: &str) -> Option<String> {
    first_matching_app_rel_path(crawl, app_root_rel_path, is_primary_tsx_source_rel_path)
}

fn first_astro_source_rel_path(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
) -> Option<String> {
    first_matching_app_rel_path(crawl, app_root_rel_path, is_primary_astro_source_rel_path)
}

fn first_mdx_content_rel_path(crawl: &G3WorkspaceCrawl, app_root_rel_path: &str) -> Option<String> {
    first_matching_app_rel_path(crawl, app_root_rel_path, |rel_path| {
        rel_path.starts_with("content/") && rel_path.ends_with(".mdx")
    })
    .or_else(|| {
        first_matching_app_rel_path(crawl, app_root_rel_path, |rel_path| {
            rel_path.starts_with("src/content/") && rel_path.ends_with(".mdx")
        })
    })
}

fn first_matching_app_rel_path(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
    predicate: impl Fn(&str) -> bool,
) -> Option<String> {
    crawl
        .entries
        .iter()
        .find(|entry| {
            is_included_file(entry)
                && is_under_app_root(&entry.path.rel_path, app_root_rel_path)
                && predicate(&app_relative_path(&entry.path.rel_path, app_root_rel_path))
        })
        .map(|entry| entry.path.rel_path.clone())
}

fn probe(probe: EslintProbeKind, rel_path: String) -> EslintProbeTarget {
    EslintProbeTarget { probe, rel_path }
}

fn is_included_file(entry: &G3WorkspaceEntry) -> bool {
    entry.kind == G3WorkspaceEntryKind::File
        && entry.ignore_state == G3WorkspaceIgnoreState::Included
}

fn is_primary_ts_source_rel_path(rel_path: &str) -> bool {
    rel_path.starts_with("src/") && rel_path.ends_with(".ts")
}

fn is_primary_tsx_source_rel_path(rel_path: &str) -> bool {
    rel_path.starts_with("src/") && rel_path.ends_with(".tsx")
}

fn is_primary_astro_source_rel_path(rel_path: &str) -> bool {
    rel_path.starts_with("src/") && rel_path.ends_with(".astro")
}
