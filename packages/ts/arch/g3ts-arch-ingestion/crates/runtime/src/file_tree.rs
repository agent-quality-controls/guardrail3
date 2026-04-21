use std::collections::BTreeSet;

use g3_workspace_crawl::{G3WorkspaceCrawl, G3WorkspaceEntryKind, G3WorkspaceIgnoreState, entry};
use g3ts_arch_types::{G3TsArchManifestState, G3TsArchSourceTree};

pub(crate) fn existing_entrypoints(
    crawl: &G3WorkspaceCrawl,
    manifest: &G3TsArchManifestState,
) -> Vec<String> {
    let G3TsArchManifestState::Parsed { snapshot } = manifest else {
        return Vec::new();
    };

    snapshot
        .declared_entrypoints
        .iter()
        .filter(|entrypoint| {
            entry(crawl, &entrypoint.rel_path)
                .is_some_and(|workspace_entry| workspace_entry.readable)
        })
        .map(|entrypoint| entrypoint.rel_path.clone())
        .collect()
}

pub(crate) fn source_tree(crawl: &G3WorkspaceCrawl) -> Option<G3TsArchSourceTree> {
    let source_entries = crawl
        .entries
        .iter()
        .filter(|entry| entry.kind == G3WorkspaceEntryKind::File)
        .filter(|entry| entry.ignore_state == G3WorkspaceIgnoreState::Included)
        .filter(|entry| entry.readable)
        .filter(|entry| is_source_code_file(&entry.path.rel_path))
        .collect::<Vec<_>>();

    if source_entries.is_empty() {
        return None;
    }

    let mut dirs = BTreeSet::new();
    let _ = dirs.insert("src".to_owned());
    for entry in &source_entries {
        let mut current = std::path::Path::new(&entry.path.rel_path).parent();
        while let Some(dir) = current {
            let Some(dir) = dir.to_str() else {
                break;
            };
            if dir == "src" || dir.starts_with("src/") {
                let _ = dirs.insert(dir.to_owned());
            }
            current = dir
                .rfind('/')
                .map(|index| std::path::Path::new(&dir[..index]));
        }
    }

    let mut max_depth = 0_usize;
    let mut max_sibling_dir_count = 0_usize;
    let mut max_sibling_code_file_count = 0_usize;

    for dir in dirs {
        let dir_prefix = format!("{dir}/");
        let depth = dir
            .trim_start_matches("src")
            .trim_matches('/')
            .split('/')
            .count()
            - 1;
        max_depth = max_depth.max(depth);

        let sibling_dir_count = crawl
            .entries
            .iter()
            .filter(|entry| entry.kind == G3WorkspaceEntryKind::Directory)
            .filter(|entry| entry.ignore_state == G3WorkspaceIgnoreState::Included)
            .filter(|entry| entry.path.rel_path.starts_with(&dir_prefix))
            .filter(|entry| immediate_child(&entry.path.rel_path, &dir_prefix))
            .count();
        max_sibling_dir_count = max_sibling_dir_count.max(sibling_dir_count);

        let sibling_code_file_count = crawl
            .entries
            .iter()
            .filter(|entry| entry.kind == G3WorkspaceEntryKind::File)
            .filter(|entry| entry.ignore_state == G3WorkspaceIgnoreState::Included)
            .filter(|entry| is_source_code_file(&entry.path.rel_path))
            .filter(|entry| entry.path.rel_path.starts_with(&dir_prefix))
            .filter(|entry| immediate_child(&entry.path.rel_path, &dir_prefix))
            .count();
        max_sibling_code_file_count = max_sibling_code_file_count.max(sibling_code_file_count);
    }

    Some(G3TsArchSourceTree {
        max_depth,
        max_sibling_dir_count,
        max_sibling_code_file_count,
    })
}

fn immediate_child(rel_path: &str, dir_prefix: &str) -> bool {
    !rel_path[dir_prefix.len()..].contains('/')
}

fn is_source_code_file(rel_path: &str) -> bool {
    rel_path.starts_with("src/")
        && (rel_path.ends_with(".ts") || rel_path.ends_with(".tsx"))
        && !rel_path.ends_with(".d.ts")
}
