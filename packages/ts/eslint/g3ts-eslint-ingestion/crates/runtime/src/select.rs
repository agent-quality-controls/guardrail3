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
    let config_scope = config_scope_dir(config_rel_path);
    let mut probes = vec![probe(
        EslintProbeKind::TsSource,
        first_ts_source_rel_path(crawl, config_scope)
            .unwrap_or_else(|| scoped_default_rel_path(config_scope, "src/index.ts")),
    )];

    if let Some(rel_path) = first_tsx_source_rel_path(crawl, config_scope) {
        probes.push(probe(EslintProbeKind::TsxSource, rel_path));
    }

    probes.push(probe(
        EslintProbeKind::TsTest,
        first_matching_rel_path(crawl, config_scope, |rel_path| {
            (rel_path.ends_with(".ts") || rel_path.ends_with(".tsx")) && is_test_rel_path(rel_path)
        })
        .unwrap_or_else(|| scoped_default_rel_path(config_scope, "src/index.test.ts")),
    ));

    probes.push(probe(
        EslintProbeKind::JsSource,
        first_js_source_rel_path(crawl, config_scope)
            .unwrap_or_else(|| scoped_default_rel_path(config_scope, "scripts/build.js")),
    ));

    probes.push(probe(
        EslintProbeKind::ConfigFile,
        config_rel_path.to_owned(),
    ));
    probes
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

fn first_js_source_rel_path(
    crawl: &G3WorkspaceCrawl,
    config_scope: Option<&str>,
) -> Option<String> {
    first_matching_rel_path(crawl, config_scope, is_primary_js_source_rel_path)
        .or_else(|| first_matching_rel_path(crawl, config_scope, is_fallback_js_source_rel_path))
}

fn probe(probe: EslintProbeKind, rel_path: String) -> EslintProbeTarget {
    EslintProbeTarget { probe, rel_path }
}

fn first_matching_rel_path(
    crawl: &G3WorkspaceCrawl,
    config_scope: Option<&str>,
    predicate: impl Fn(&str) -> bool,
) -> Option<String> {
    crawl
        .entries
        .iter()
        .find(|entry| {
            is_probe_candidate(entry)
                && in_config_scope(&entry.path.rel_path, config_scope)
                && predicate(&entry.path.rel_path)
        })
        .map(|entry| entry.path.rel_path.clone())
}

fn config_scope_dir(config_rel_path: &str) -> Option<&str> {
    let parent = std::path::Path::new(config_rel_path).parent()?;
    let parent = parent.to_str()?;
    if parent.is_empty() {
        None
    } else {
        Some(parent)
    }
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
        && !is_script_rel_path(rel_path)
        && !is_test_rel_path(rel_path)
        && !is_eslint_config(rel_path)
        && !is_config_like_rel_path(rel_path)
}

fn is_fallback_tsx_source_rel_path(rel_path: &str) -> bool {
    rel_path.ends_with(".tsx")
        && !is_script_rel_path(rel_path)
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
        && !is_script_rel_path(rel_path)
        && !is_eslint_config(rel_path)
        && !is_test_rel_path(rel_path)
        && !is_config_like_rel_path(rel_path)
}

fn is_script_rel_path(rel_path: &str) -> bool {
    rel_path.starts_with("scripts/") || rel_path.contains("/scripts/")
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
