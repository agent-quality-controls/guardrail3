use g3rs_test_types::{
    G3RsTestFileTreeInputFailure, G3RsTestSourceFile, G3RsTestSourceInputFailure,
};
use g3rs_workspace_crawl::{G3RsWorkspaceCrawl, G3RsWorkspaceEntryKind};

use crate::components::OwnedTestComponent;
use crate::components::classify::{classify_file_for_file_tree, classify_file_for_source};
use crate::components::facts::build_owned_component;
use crate::components::support::{dedupe_failures, is_fixture_path, parse_manifest_lenient};
use crate::ingest::IngestionError;
use crate::roots::OwnedTestRoot;

pub(crate) fn collect_components(
    crawl: &G3RsWorkspaceCrawl,
    root: &OwnedTestRoot,
) -> Result<Vec<OwnedTestComponent>, IngestionError> {
    let layout = crate::components::support::resolve_assertions_layout(crawl, root, None)?;
    Ok(vec![build_owned_component(crawl, root, layout)])
}

pub(crate) fn collect_file_tree_components(
    crawl: &G3RsWorkspaceCrawl,
    root: &OwnedTestRoot,
) -> (Vec<OwnedTestComponent>, Vec<G3RsTestFileTreeInputFailure>) {
    let mut input_failures = Vec::new();
    let layout = crate::components::support::resolve_assertions_layout(
        crawl,
        root,
        Some(&mut input_failures),
    )
    .expect("lenient assertions layout resolution should not fail");

    (
        vec![build_owned_component(crawl, root, layout)],
        input_failures,
    )
}

pub(crate) fn collect_ast_files(
    crawl: &G3RsWorkspaceCrawl,
    root: &OwnedTestRoot,
    components: &[OwnedTestComponent],
) -> (Vec<G3RsTestSourceFile>, Vec<G3RsTestSourceInputFailure>) {
    let mut files = Vec::new();
    let mut input_failures = Vec::new();

    for entry in crawl
        .entries
        .iter()
        .filter(|entry| entry.kind == G3RsWorkspaceEntryKind::File)
        .filter(|entry| entry.path.rel_path.ends_with(".rs"))
        .filter_map(|entry| {
            classify_file_for_source(&entry.path.rel_path, root, components)
                .map(|file| (entry, file))
        })
    {
        let (entry, mut file) = entry;
        if !entry.readable {
            input_failures.push(G3RsTestSourceInputFailure {
                rel_path: entry.path.rel_path.clone(),
                message:
                    "Failed to read Rust source file for test-family source analysis: file is not readable"
                        .to_owned(),
            });
            continue;
        }
        match crate::fs::read_to_string(&entry.path.abs_path) {
            Ok(content) => {
                file.content = content;
                files.push(file);
            }
            Err(err) => input_failures.push(G3RsTestSourceInputFailure {
                rel_path: entry.path.rel_path.clone(),
                message: format!(
                    "Failed to read Rust source file for test-family source analysis: {err}"
                ),
            }),
        }
    }

    files.sort_by(|left, right| left.rel_path.cmp(&right.rel_path));
    input_failures.sort_by(|left, right| left.rel_path.cmp(&right.rel_path));
    input_failures
        .dedup_by(|left, right| left.rel_path == right.rel_path && left.message == right.message);
    (files, input_failures)
}

pub(crate) fn collect_file_tree_files(
    crawl: &G3RsWorkspaceCrawl,
    root: &OwnedTestRoot,
    components: &[OwnedTestComponent],
) -> (Vec<G3RsTestSourceFile>, Vec<G3RsTestFileTreeInputFailure>) {
    let mut files = Vec::new();
    let mut input_failures = Vec::new();

    for entry in crawl
        .entries
        .iter()
        .filter(|entry| entry.kind == G3RsWorkspaceEntryKind::File)
        .filter(|entry| entry.path.rel_path.ends_with(".rs"))
    {
        let Some(mut file) = classify_file_for_file_tree(&entry.path.rel_path, root, components)
        else {
            continue;
        };
        if !entry.readable {
            input_failures.push(G3RsTestFileTreeInputFailure {
                rel_path: entry.path.rel_path.clone(),
                message:
                    "Failed to read Rust source file for test-family analysis: file is not readable"
                        .to_owned(),
            });
            continue;
        }
        match crate::fs::read_to_string(&entry.path.abs_path) {
            Ok(content) => {
                file.content = content;
                files.push(file);
            }
            Err(err) => input_failures.push(G3RsTestFileTreeInputFailure {
                rel_path: entry.path.rel_path.clone(),
                message: format!("Failed to read Rust source file for test-family analysis: {err}"),
            }),
        }
    }

    files.sort_by(|left, right| left.rel_path.cmp(&right.rel_path));
    input_failures.sort_by(|left, right| left.rel_path.cmp(&right.rel_path));
    dedupe_failures(&mut input_failures);
    (files, input_failures)
}

pub(crate) fn collect_local_package_names(
    crawl: &G3RsWorkspaceCrawl,
    root: &OwnedTestRoot,
) -> (
    std::collections::BTreeSet<String>,
    Vec<G3RsTestFileTreeInputFailure>,
) {
    let mut package_names = std::collections::BTreeSet::new();
    let mut input_failures = Vec::new();

    for entry in crawl
        .entries
        .iter()
        .filter(|entry| entry.kind == G3RsWorkspaceEntryKind::File)
        .filter(|entry| entry.path.rel_path.ends_with("Cargo.toml"))
        .filter(|entry| {
            crate::components::classify::root_rel_prefix(&entry.path.rel_path, &root.root_rel_dir)
                .is_some_and(|_| !is_fixture_path(&entry.path.rel_path))
        })
    {
        let Some(manifest) =
            parse_manifest_lenient(crawl, &entry.path.rel_path, &mut input_failures)
        else {
            continue;
        };
        if let Some(package_name) = manifest
            .package
            .as_ref()
            .and_then(|package| package.name.as_deref())
        {
            let _ = package_names.insert(crate::components::support::rust_crate_name(package_name));
        }
    }

    input_failures.sort_by(|left, right| left.rel_path.cmp(&right.rel_path));
    dedupe_failures(&mut input_failures);
    (package_names, input_failures)
}
