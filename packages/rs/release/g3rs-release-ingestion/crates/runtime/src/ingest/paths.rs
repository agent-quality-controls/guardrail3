use std::collections::BTreeSet;
use std::path::{Component, Path, PathBuf};

use g3rs_release_types::G3RsReleasePathTargetKind;
use g3rs_workspace_crawl::{G3RsWorkspaceCrawl, G3RsWorkspaceEntryKind};

/// `resolve_manifest_relative_path` function.
pub(super) fn resolve_manifest_relative_path(
    crawl: &G3RsWorkspaceCrawl,
    manifest_rel_dir: &str,
    relative: &str,
) -> (String, PathBuf) {
    let joined = if manifest_rel_dir.is_empty() {
        relative.to_owned()
    } else {
        format!("{manifest_rel_dir}/{relative}")
    };
    let rel = normalize_relative_path(Path::new(&joined));
    let abs = g3rs_workspace_crawl::entry(crawl, &rel).map_or_else(
        || crawl.root_abs_path.join(&rel),
        |entry| entry.path.abs_path.clone(),
    );
    (rel, abs)
}

/// `normalize_relative_path` function.
fn normalize_relative_path(path: &Path) -> String {
    let mut parts = Vec::new();
    for component in path.components() {
        match component {
            Component::CurDir | Component::RootDir | Component::Prefix(_) => {}
            Component::ParentDir => {
                let _ = parts.pop();
            }
            Component::Normal(part) => parts.push(part.to_string_lossy().into_owned()),
        }
    }
    parts.join("/")
}

/// `classify_dependency_path` function.
pub(super) fn classify_dependency_path(
    crawl: &G3RsWorkspaceCrawl,
    base_dir: &Path,
    relative: &str,
) -> G3RsReleasePathTargetKind {
    let normalized_target = normalize_absolute_path(&base_dir.join(relative));
    let normalized_root = normalize_absolute_path(&crawl.root_abs_path);
    if normalized_target.starts_with(&normalized_root) {
        G3RsReleasePathTargetKind::InWorkspace
    } else {
        G3RsReleasePathTargetKind::OutsideWorkspace
    }
}

/// `normalize_absolute_path` function.
fn normalize_absolute_path(path: &Path) -> PathBuf {
    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                let _ = normalized.pop();
            }
            Component::Normal(part) => normalized.push(part),
            Component::RootDir => normalized.push(Path::new("/")),
            Component::Prefix(prefix) => normalized.push(prefix.as_os_str()),
        }
    }
    normalized
}

/// `join_under_root` function.
pub(super) fn join_under_root(root_rel_dir: &str, child: &str) -> String {
    if root_rel_dir.is_empty() {
        child.to_owned()
    } else {
        format!("{root_rel_dir}/{child}")
    }
}

/// `file_exists` function.
pub(super) fn file_exists(crawl: &G3RsWorkspaceCrawl, rel_path: &str) -> bool {
    g3rs_workspace_crawl::entry(crawl, rel_path)
        .is_some_and(|entry| entry.kind == G3RsWorkspaceEntryKind::File)
}

/// `direct_child_files` function.
pub(super) fn direct_child_files(crawl: &G3RsWorkspaceCrawl, dir_rel: &str) -> Vec<String> {
    let prefix = if dir_rel.is_empty() {
        String::new()
    } else {
        format!("{dir_rel}/")
    };

    crawl
        .entries
        .iter()
        .filter(|entry| entry.kind == G3RsWorkspaceEntryKind::File)
        .filter_map(|entry| entry.path.rel_path.strip_prefix(&prefix))
        .filter(|rest| !rest.is_empty() && !rest.contains('/'))
        .map(str::to_owned)
        .collect()
}

/// `direct_child_dirs` function.
pub(super) fn direct_child_dirs(crawl: &G3RsWorkspaceCrawl, dir_rel: &str) -> Vec<String> {
    let prefix = if dir_rel.is_empty() {
        String::new()
    } else {
        format!("{dir_rel}/")
    };

    let dirs = crawl
        .entries
        .iter()
        .filter_map(|entry| entry.path.rel_path.strip_prefix(&prefix))
        .filter_map(|rest| rest.split_once('/').map(|(first, _)| first))
        .filter(|segment| !segment.is_empty())
        .map(str::to_owned)
        .collect::<BTreeSet<_>>();
    dirs.into_iter().collect()
}
