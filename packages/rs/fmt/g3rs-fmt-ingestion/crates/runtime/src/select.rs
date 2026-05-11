use std::collections::BTreeMap;
use std::path::Path;

use g3_workspace_crawl::{
    G3WorkspaceCrawl, G3WorkspaceEntry, G3WorkspaceEntryKind, G3WorkspaceIgnoreState,
};
use g3rs_fmt_types::{G3RsFmtConfigFileKind, G3RsFmtNestedConfigFile};

/// Implements `select active rustfmt config`.
pub(crate) fn select_active_rustfmt_config(crawl: &G3WorkspaceCrawl) -> Option<&G3WorkspaceEntry> {
    select_root_rustfmt_toml(crawl).or_else(|| select_root_dot_rustfmt_toml(crawl))
}

/// Implements `select root rustfmt toml`.
pub(crate) fn select_root_rustfmt_toml(crawl: &G3WorkspaceCrawl) -> Option<&G3WorkspaceEntry> {
    g3_workspace_crawl::root_file(crawl, "rustfmt.toml")
}

/// Implements `select root dot rustfmt toml`.
pub(crate) fn select_root_dot_rustfmt_toml(crawl: &G3WorkspaceCrawl) -> Option<&G3WorkspaceEntry> {
    g3_workspace_crawl::root_file(crawl, ".rustfmt.toml")
}

/// Implements `select cargo toml`.
pub(crate) fn select_cargo_toml(crawl: &G3WorkspaceCrawl) -> Option<&G3WorkspaceEntry> {
    g3_workspace_crawl::root_file(crawl, "Cargo.toml")
}

/// Implements `select toolchain toml`.
pub(crate) fn select_toolchain_toml(crawl: &G3WorkspaceCrawl) -> Option<&G3WorkspaceEntry> {
    g3_workspace_crawl::root_file(crawl, "rust-toolchain.toml")
}

/// Implements `select rust policy toml`.
pub(crate) fn select_rust_policy_toml(crawl: &G3WorkspaceCrawl) -> Option<&G3WorkspaceEntry> {
    g3_workspace_crawl::root_file(crawl, "guardrail3-rs.toml")
}

/// Implements `collect nested config files`.
pub(crate) fn collect_nested_config_files(
    crawl: &G3WorkspaceCrawl,
) -> Vec<G3RsFmtNestedConfigFile> {
    let mut files = crawl
        .entries
        .iter()
        .filter(|entry| entry.kind == G3WorkspaceEntryKind::File)
        .filter(|entry| entry.ignore_state == G3WorkspaceIgnoreState::Included)
        .filter_map(|entry| {
            let kind = config_file_kind(&entry.path.rel_path)?;
            (!is_root_config_path(&entry.path.rel_path)
                && !is_ignored_tree_path(&entry.path.rel_path))
            .then_some(G3RsFmtNestedConfigFile {
                rel_path: entry.path.rel_path.clone(),
                kind,
            })
        })
        .collect::<Vec<_>>();
    files.sort_by(|left, right| left.rel_path.cmp(&right.rel_path));
    files
}

/// Implements `collect dual conflict dirs`.
pub(crate) fn collect_dual_conflict_dirs(crawl: &G3WorkspaceCrawl) -> Vec<String> {
    let mut dirs = BTreeMap::<String, (bool, bool)>::new();
    for entry in crawl
        .entries
        .iter()
        .filter(|entry| entry.kind == G3WorkspaceEntryKind::File)
        .filter(|entry| entry.ignore_state == G3WorkspaceIgnoreState::Included)
    {
        let Some(kind) = config_file_kind(&entry.path.rel_path) else {
            continue;
        };
        if is_ignored_tree_path(&entry.path.rel_path) {
            continue;
        }
        let dir_rel = Path::new(&entry.path.rel_path)
            .parent()
            .and_then(Path::to_str)
            .unwrap_or("")
            .to_owned();
        let flags = dirs.entry(dir_rel).or_default();
        match kind {
            G3RsFmtConfigFileKind::RustfmtToml => flags.0 = true,
            G3RsFmtConfigFileKind::DotRustfmtToml => flags.1 = true,
        }
    }
    dirs.into_iter()
        .filter_map(|(dir_rel, (has_rustfmt, has_dot_rustfmt))| {
            (has_rustfmt && has_dot_rustfmt).then_some(dir_rel)
        })
        .collect()
}

/// Implements `is root config path`.
fn is_root_config_path(rel_path: &str) -> bool {
    matches!(rel_path, "rustfmt.toml" | ".rustfmt.toml")
}

/// Implements `is ignored tree path`.
fn is_ignored_tree_path(rel_path: &str) -> bool {
    rel_path.starts_with("target/")
        || rel_path.starts_with("tests/fixtures/")
        || rel_path.starts_with("tests/snapshots/")
        || rel_path.starts_with(".claude/worktrees/")
}

/// Implements `config file kind`.
fn config_file_kind(rel_path: &str) -> Option<G3RsFmtConfigFileKind> {
    match Path::new(rel_path)
        .file_name()
        .and_then(|name| name.to_str())
    {
        Some("rustfmt.toml") => Some(G3RsFmtConfigFileKind::RustfmtToml),
        Some(".rustfmt.toml") => Some(G3RsFmtConfigFileKind::DotRustfmtToml),
        _ => None,
    }
}
