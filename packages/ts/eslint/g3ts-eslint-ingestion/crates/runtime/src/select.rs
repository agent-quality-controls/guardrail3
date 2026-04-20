use eslint_config_parser::types::{EslintProbeKind, EslintProbeTarget};
use g3_workspace_crawl::{
    G3WorkspaceCrawl, G3WorkspaceEntry, G3WorkspaceEntryKind, G3WorkspaceIgnoreState, root_file,
};

const ROOT_ESLINT_CONFIGS: [&str; 6] = [
    "eslint.config.js",
    "eslint.config.mjs",
    "eslint.config.cjs",
    "eslint.config.ts",
    "eslint.config.mts",
    "eslint.config.cts",
];

pub(crate) fn select_active_root_eslint_config(
    crawl: &G3WorkspaceCrawl,
) -> Option<&G3WorkspaceEntry> {
    ROOT_ESLINT_CONFIGS.iter().find_map(|file_name| {
        root_file(crawl, file_name).filter(|entry| {
            entry.kind == G3WorkspaceEntryKind::File
                && entry.ignore_state == G3WorkspaceIgnoreState::Included
        })
    })
}

pub(crate) fn probe_targets(
    crawl: &G3WorkspaceCrawl,
    config_rel_path: &str,
) -> Vec<EslintProbeTarget> {
    let mut probes = vec![probe(
        EslintProbeKind::TsSource,
        first_ts_source_rel_path(crawl).unwrap_or_else(|| "src/index.ts".to_owned()),
    )];

    if let Some(rel_path) = first_tsx_source_rel_path(crawl) {
        probes.push(probe(EslintProbeKind::TsxSource, rel_path));
    }

    probes.push(probe(
        EslintProbeKind::TsTest,
        first_matching_rel_path(crawl, |rel_path| {
            (rel_path.ends_with(".ts") || rel_path.ends_with(".tsx")) && is_test_rel_path(rel_path)
        })
        .unwrap_or_else(|| "src/index.test.ts".to_owned()),
    ));

    probes.push(probe(
        EslintProbeKind::JsSource,
        first_js_source_rel_path(crawl).unwrap_or_else(|| "scripts/build.js".to_owned()),
    ));

    probes.push(probe(
        EslintProbeKind::ConfigFile,
        config_rel_path.to_owned(),
    ));
    probes
}

fn first_ts_source_rel_path(crawl: &G3WorkspaceCrawl) -> Option<String> {
    first_matching_rel_path(crawl, is_primary_ts_source_rel_path)
        .or_else(|| first_matching_rel_path(crawl, is_fallback_ts_source_rel_path))
        .or_else(|| first_tsx_source_rel_path(crawl))
}

fn first_tsx_source_rel_path(crawl: &G3WorkspaceCrawl) -> Option<String> {
    first_matching_rel_path(crawl, is_primary_tsx_source_rel_path)
        .or_else(|| first_matching_rel_path(crawl, is_fallback_tsx_source_rel_path))
}

fn first_js_source_rel_path(crawl: &G3WorkspaceCrawl) -> Option<String> {
    first_matching_rel_path(crawl, is_primary_js_source_rel_path)
        .or_else(|| first_matching_rel_path(crawl, is_fallback_js_source_rel_path))
}

fn probe(probe: EslintProbeKind, rel_path: String) -> EslintProbeTarget {
    EslintProbeTarget { probe, rel_path }
}

fn first_matching_rel_path(
    crawl: &G3WorkspaceCrawl,
    predicate: impl Fn(&str) -> bool,
) -> Option<String> {
    crawl
        .entries
        .iter()
        .find(|entry| is_probe_candidate(entry) && predicate(&entry.path.rel_path))
        .map(|entry| entry.path.rel_path.clone())
}

fn is_probe_candidate(entry: &G3WorkspaceEntry) -> bool {
    entry.kind == G3WorkspaceEntryKind::File
        && entry.ignore_state == G3WorkspaceIgnoreState::Included
        && entry.readable
}

fn is_test_rel_path(rel_path: &str) -> bool {
    rel_path.contains(".test.")
        || rel_path.contains(".spec.")
        || rel_path.contains("/tests/")
        || rel_path.contains("/__tests__/")
}

fn is_eslint_config(rel_path: &str) -> bool {
    ROOT_ESLINT_CONFIGS.contains(&rel_path)
}

fn is_primary_ts_source_rel_path(rel_path: &str) -> bool {
    rel_path.starts_with("src/")
        && rel_path.ends_with(".ts")
        && !rel_path.ends_with(".d.ts")
        && !is_test_rel_path(rel_path)
        && !is_eslint_config(rel_path)
        && !is_config_like_rel_path(rel_path)
}

fn is_primary_tsx_source_rel_path(rel_path: &str) -> bool {
    rel_path.starts_with("src/")
        && rel_path.ends_with(".tsx")
        && !is_test_rel_path(rel_path)
        && !is_eslint_config(rel_path)
        && !is_config_like_rel_path(rel_path)
}

fn is_fallback_ts_source_rel_path(rel_path: &str) -> bool {
    rel_path.ends_with(".ts")
        && !rel_path.ends_with(".d.ts")
        && !rel_path.starts_with("scripts/")
        && !is_test_rel_path(rel_path)
        && !is_eslint_config(rel_path)
        && !is_config_like_rel_path(rel_path)
}

fn is_fallback_tsx_source_rel_path(rel_path: &str) -> bool {
    rel_path.ends_with(".tsx")
        && !rel_path.starts_with("scripts/")
        && !is_test_rel_path(rel_path)
        && !is_eslint_config(rel_path)
        && !is_config_like_rel_path(rel_path)
}

fn is_primary_js_source_rel_path(rel_path: &str) -> bool {
    rel_path.starts_with("src/")
        && rel_path.ends_with(".js")
        && !is_eslint_config(rel_path)
        && !is_test_rel_path(rel_path)
        && !is_config_like_rel_path(rel_path)
}

fn is_fallback_js_source_rel_path(rel_path: &str) -> bool {
    rel_path.ends_with(".js")
        && !rel_path.starts_with("scripts/")
        && !is_eslint_config(rel_path)
        && !is_test_rel_path(rel_path)
        && !is_config_like_rel_path(rel_path)
}

fn is_config_like_rel_path(rel_path: &str) -> bool {
    std::path::Path::new(rel_path)
        .file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.contains(".config."))
}

#[cfg(test)]
#[path = "select_tests/mod.rs"]
// reason: owned sidecar tests for file module.
mod select_tests;
