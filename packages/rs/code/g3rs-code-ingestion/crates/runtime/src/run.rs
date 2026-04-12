use std::collections::BTreeSet;

use cargo_toml_parser::parse;
use g3rs_code_ingestion_types::{
    G3RsCodeConfigChecksInput, G3RsCodeFileTreeChecksInput, G3RsCodeSourceChecksInput,
    G3RsCodeStructuralCapRoot,
};
use g3rs_workspace_crawl::{G3RsWorkspaceCrawl, G3RsWorkspaceEntryKind};

/// Re-export of `G3RsCodeIngestionError` so the facade can reach it.
pub use g3rs_code_ingestion_types::G3RsCodeIngestionError as IngestionError;

pub fn ingest_for_config_checks(
    crawl: &G3RsWorkspaceCrawl,
) -> Result<G3RsCodeConfigChecksInput, IngestionError> {
    let exception_comments = crate::config_comments::collect_exception_comments(crawl)?;
    let unsafe_code_lints = crate::unsafe_code_lints::collect_unsafe_code_lints(crawl)?;
    Ok(crate::config::assemble(exception_comments, unsafe_code_lints))
}

/// Ingest `code` source checks input from a workspace crawl.
pub fn ingest_for_source_checks(
    crawl: &G3RsWorkspaceCrawl,
) -> Result<Vec<G3RsCodeSourceChecksInput>, IngestionError> {
    crate::select::select_source_files(crawl)?
        .into_iter()
        .map(|selected| {
            if !selected.entry.readable {
                return Err(IngestionError::Unreadable {
                    path: selected.entry.path.abs_path.clone(),
                    reason: "file is not readable".to_owned(),
                });
            }

            let content =
                crate::fs::read_to_string(&selected.entry.path.abs_path).map_err(|err| {
                    IngestionError::Unreadable {
                        path: selected.entry.path.abs_path.clone(),
                        reason: err.to_string(),
                    }
                })?;

            Ok(crate::ingest::assemble(
                selected.entry.path.rel_path.clone(),
                content,
                selected.is_test,
                selected.profile_name,
                selected.is_library_root,
            ))
        })
        .collect()
}

/// Stub file-tree ingestion entry point for the code family.
pub fn ingest_for_file_tree_checks(
    crawl: &G3RsWorkspaceCrawl,
) -> Result<G3RsCodeFileTreeChecksInput, IngestionError> {
    let roots = selected_package_roots(crawl)?;
    let root_dirs = roots
        .iter()
        .map(|root| root.root_rel_dir.clone())
        .collect::<BTreeSet<_>>();

    Ok(G3RsCodeFileTreeChecksInput {
        roots: roots
            .into_iter()
            .map(|root| measure_root_structure(crawl, root, &root_dirs))
            .collect(),
    })
}

fn selected_package_roots(
    crawl: &G3RsWorkspaceCrawl,
) -> Result<Vec<G3RsCodeStructuralCapRoot>, IngestionError> {
    let Some(root_cargo_entry) = crawl.root_file("Cargo.toml") else {
        return Err(IngestionError::ParseFailed {
            path: crawl.root_abs_path.join("Cargo.toml"),
            reason: "root Cargo.toml missing from crawl".to_owned(),
        });
    };

    if !root_cargo_entry.readable {
        return Err(IngestionError::Unreadable {
            path: root_cargo_entry.path.abs_path.clone(),
            reason: "file is not readable".to_owned(),
        });
    }

    let root_content =
        crate::fs::read_to_string(&root_cargo_entry.path.abs_path).map_err(|err| {
            IngestionError::Unreadable {
                path: root_cargo_entry.path.abs_path.clone(),
                reason: err.to_string(),
            }
        })?;
    let root_manifest = parse(&root_content).map_err(|err| IngestionError::ParseFailed {
        path: root_cargo_entry.path.abs_path.clone(),
        reason: err.to_string(),
    })?;

    let mut roots = crate::config_scope::owned_root_dirs(crawl)?
        .into_iter()
        .filter(|rel_dir| !rel_dir.is_empty())
        .map(|rel_dir| G3RsCodeStructuralCapRoot {
            cargo_rel_path: join_rel(&rel_dir, "Cargo.toml"),
            root_rel_dir: rel_dir,
            max_module_depth: 0,
            max_sibling_dirs: 0,
            max_sibling_rs_files: 0,
        })
        .collect::<Vec<_>>();

    if root_manifest.package.is_some() {
        roots.push(G3RsCodeStructuralCapRoot {
            root_rel_dir: String::new(),
            cargo_rel_path: "Cargo.toml".to_owned(),
            max_module_depth: 0,
            max_sibling_dirs: 0,
            max_sibling_rs_files: 0,
        });
    }

    roots.sort_by(|left, right| left.root_rel_dir.cmp(&right.root_rel_dir));
    Ok(roots)
}

fn measure_root_structure(
    crawl: &G3RsWorkspaceCrawl,
    mut root: G3RsCodeStructuralCapRoot,
    root_dirs: &BTreeSet<String>,
) -> G3RsCodeStructuralCapRoot {
    let rust_files = crawl
        .entries
        .iter()
        .filter(|entry| entry.kind == G3RsWorkspaceEntryKind::File)
        .filter(|entry| entry.path.rel_path.ends_with(".rs"))
        .filter(|entry| !crate::classify::is_fixture_path(entry.path.rel_path.as_str()))
        .filter(|entry| !is_generated_path(entry.path.rel_path.as_str()))
        .filter(|entry| owning_root_dir(entry.path.rel_path.as_str(), root_dirs) == Some(root.root_rel_dir.as_str()))
        .map(|entry| entry.path.rel_path.clone())
        .collect::<Vec<_>>();

    let rust_related_dirs = collect_rust_related_dirs(&rust_files, &root.root_rel_dir);

    root.max_module_depth = rust_files
        .iter()
        .map(|rel_path| module_depth(&root.root_rel_dir, rel_path))
        .max()
        .unwrap_or(0);
    root.max_sibling_dirs = rust_related_dirs
        .iter()
        .map(|dir_rel| count_direct_child_dirs(dir_rel, &rust_related_dirs))
        .max()
        .unwrap_or(0);
    root.max_sibling_rs_files = rust_related_dirs
        .iter()
        .map(|dir_rel| count_rs_files_in_dir(dir_rel, &rust_files))
        .max()
        .unwrap_or(0);

    root
}

fn collect_rust_related_dirs(rust_files: &[String], root_rel_dir: &str) -> BTreeSet<String> {
    let mut dirs = BTreeSet::new();

    for rel_path in rust_files {
        let mut current = file_parent_rel(rel_path).to_owned();
        loop {
            let _ = dirs.insert(current.clone());
            if current == root_rel_dir || current.is_empty() {
                break;
            }
            let Some((parent, _)) = current.rsplit_once('/') else {
                current.clear();
                continue;
            };
            current = parent.to_owned();
        }
    }

    dirs
}

fn count_direct_child_dirs(dir_rel: &str, rust_related_dirs: &BTreeSet<String>) -> usize {
    rust_related_dirs
        .iter()
        .filter(|candidate| is_direct_child_dir(candidate, dir_rel))
        .count()
}

fn count_rs_files_in_dir(dir_rel: &str, rust_files: &[String]) -> usize {
    rust_files
        .iter()
        .filter(|rel_path| file_parent_rel(rel_path) == dir_rel)
        .count()
}

fn is_direct_child_dir(candidate: &str, parent: &str) -> bool {
    if candidate.is_empty() {
        return false;
    }

    if parent.is_empty() {
        return !candidate.contains('/');
    }

    let Some(rest) = candidate.strip_prefix(parent) else {
        return false;
    };

    rest.strip_prefix('/').is_some_and(|suffix| !suffix.contains('/'))
}

fn file_parent_rel(rel_path: &str) -> &str {
    rel_path.rsplit_once('/').map_or("", |(parent, _)| parent)
}

fn owning_root_dir<'a>(rel_path: &str, root_dirs: &'a BTreeSet<String>) -> Option<&'a str> {
    let parent = file_parent_rel(rel_path);

    root_dirs
        .iter()
        .filter(|root| {
            root.is_empty()
                || parent == root.as_str()
                || parent
                    .strip_prefix(root.as_str())
                    .is_some_and(|rest| rest.starts_with('/'))
        })
        .max_by_key(|root| root.len())
        .map(String::as_str)
}

fn module_depth(root_rel_dir: &str, rel_path: &str) -> usize {
    let root_segments = path_segments(root_rel_dir);
    let path_segments = path_segments(rel_path);
    if path_segments.len() <= root_segments.len() {
        return 0;
    }
    let relative = &path_segments[root_segments.len()..];
    let file_name = *relative.last().unwrap_or(&"");
    let dir_depth = relative.len().saturating_sub(1);
    if matches!(file_name, "lib.rs" | "main.rs" | "mod.rs") {
        dir_depth
    } else {
        dir_depth.saturating_add(1)
    }
}

fn path_segments(rel_path: &str) -> Vec<&str> {
    rel_path
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect()
}

fn is_generated_path(rel_path: &str) -> bool {
    rel_path == "target" || rel_path.starts_with("target/") || rel_path.contains("/target/")
}

fn join_rel(dir: &str, child: &str) -> String {
    if dir.is_empty() {
        child.to_owned()
    } else {
        format!("{dir}/{child}")
    }
}
