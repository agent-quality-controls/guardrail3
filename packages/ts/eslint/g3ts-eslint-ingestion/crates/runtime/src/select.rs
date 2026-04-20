use eslint_config_parser::types::{EslintProbeKind, EslintProbeTarget};
use g3_workspace_crawl::{G3WorkspaceCrawl, G3WorkspaceEntry, G3WorkspaceEntryKind, root_file};

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
    ROOT_ESLINT_CONFIGS
        .iter()
        .find_map(|file_name| root_file(crawl, file_name))
}

pub(crate) fn probe_targets(
    crawl: &G3WorkspaceCrawl,
    config_rel_path: &str,
) -> Vec<EslintProbeTarget> {
    vec![
        probe(
            EslintProbeKind::TsSource,
            first_matching_rel_path(crawl, |rel_path| {
                rel_path.ends_with(".ts")
                    && !rel_path.ends_with(".d.ts")
                    && !is_test_rel_path(rel_path)
                    && !is_eslint_config(rel_path)
            })
            .unwrap_or_else(|| "src/index.ts".to_owned()),
        ),
        probe(
            EslintProbeKind::TsxSource,
            first_matching_rel_path(crawl, |rel_path| {
                rel_path.ends_with(".tsx") && !is_test_rel_path(rel_path)
            })
            .unwrap_or_else(|| "src/index.tsx".to_owned()),
        ),
        probe(
            EslintProbeKind::TsTest,
            first_matching_rel_path(crawl, |rel_path| {
                (rel_path.ends_with(".ts") || rel_path.ends_with(".tsx"))
                    && is_test_rel_path(rel_path)
            })
            .unwrap_or_else(|| "src/index.test.ts".to_owned()),
        ),
        probe(
            EslintProbeKind::JsSource,
            first_matching_rel_path(crawl, |rel_path| {
                rel_path.ends_with(".js") && !is_eslint_config(rel_path)
            })
            .unwrap_or_else(|| "scripts/build.js".to_owned()),
        ),
        probe(EslintProbeKind::ConfigFile, config_rel_path.to_owned()),
    ]
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
        .find(|entry| entry.kind == G3WorkspaceEntryKind::File && predicate(&entry.path.rel_path))
        .map(|entry| entry.path.rel_path.clone())
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

#[cfg(test)]
#[path = "select_tests/mod.rs"]
// reason: owned sidecar tests for file module.
mod select_tests;
