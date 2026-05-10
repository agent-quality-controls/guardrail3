#![expect(
    clippy::case_sensitive_file_extension_comparisons,
    reason = "rust source filenames are conventionally lowercase; case-insensitive matching not needed"
)]
#![expect(
    clippy::type_complexity,
    reason = "structural code pattern (parser/assertion helper) where lint conflicts with module architecture"
)]
use std::collections::BTreeSet;

use cargo_toml_parser::{parse, types::CargoToml};
use g3rs_test_types::{G3RsTestFileTreeInputFailure, G3RsTestOwnedSidecarFacts};
use g3rs_workspace_crawl::{G3RsWorkspaceCrawl, G3RsWorkspaceEntryKind};

use crate::ingest::IngestionError;
use crate::roots::{OwnedTestRoot, join_under_root, parent_dir};

/// `AssertionsLayout` struct.
pub(crate) struct AssertionsLayout {
    /// `assertions_rel_dir` item.
    pub(crate) assertions_rel_dir: String,
    /// `assertions_cargo_rel_path` item.
    pub(crate) assertions_cargo_rel_path: String,
    /// `assertions_manifest` item.
    pub(crate) assertions_manifest: Option<CargoToml>,
    /// `nested_assertions_cargo_rel_path` item.
    pub(crate) nested_assertions_cargo_rel_path: Option<String>,
}

/// `parse_manifest_lenient` function.
pub(crate) fn parse_manifest_lenient(
    crawl: &G3RsWorkspaceCrawl,
    rel_path: &str,
    input_failures: &mut Vec<G3RsTestFileTreeInputFailure>,
) -> Option<CargoToml> {
    let Some(entry) = g3rs_workspace_crawl::entry(crawl, rel_path) else {
        input_failures.push(G3RsTestFileTreeInputFailure {
            rel_path: rel_path.to_owned(),
            message: "Failed to parse Cargo.toml for test-family boundaries: required Cargo.toml entry is missing from crawl".to_owned(),
        });
        return None;
    };
    if !entry.readable {
        input_failures.push(G3RsTestFileTreeInputFailure {
            rel_path: rel_path.to_owned(),
            message: "Failed to read Cargo.toml for test-family boundaries: file is not readable"
                .to_owned(),
        });
        return None;
    }
    let content = match crate::fs::read_to_string(&entry.path.abs_path) {
        Ok(content) => content,
        Err(err) => {
            input_failures.push(G3RsTestFileTreeInputFailure {
                rel_path: rel_path.to_owned(),
                message: format!("Failed to read Cargo.toml for test-family boundaries: {err}"),
            });
            return None;
        }
    };
    match parse(&content) {
        Ok(manifest) => Some(manifest),
        Err(err) => {
            input_failures.push(G3RsTestFileTreeInputFailure {
                rel_path: rel_path.to_owned(),
                message: format!("Failed to parse Cargo.toml for test-family boundaries: {err}"),
            });
            None
        }
    }
}

/// `manifest_normal_dependencies` function.
pub(crate) fn manifest_normal_dependencies(manifest: &CargoToml) -> BTreeSet<String> {
    manifest
        .dependencies
        .keys()
        .chain(manifest.build_dependencies.keys())
        .map(|name| rust_crate_name(name))
        .collect()
}

/// `manifest_dev_dependencies` function.
pub(crate) fn manifest_dev_dependencies(manifest: &CargoToml) -> BTreeSet<String> {
    manifest
        .dev_dependencies
        .keys()
        .map(|name| rust_crate_name(name))
        .collect()
}

/// `resolve_assertions_layout` function.
pub(crate) fn resolve_assertions_layout(
    crawl: &G3RsWorkspaceCrawl,
    root: &OwnedTestRoot,
    input_failures: Option<&mut Vec<G3RsTestFileTreeInputFailure>>,
) -> Result<AssertionsLayout, IngestionError> {
    let nested_assertions_cargo_rel_path = g3rs_workspace_crawl::entry(
        crawl,
        &join_under_root(&root.root_rel_dir, "assertions/Cargo.toml"),
    )
    .filter(|entry| entry.kind == G3RsWorkspaceEntryKind::File)
    .map(|_| join_under_root(&root.root_rel_dir, "assertions/Cargo.toml"));
    let assertions_rel_dir = if root.runtime_rel_dir == root.root_rel_dir
        && nested_assertions_cargo_rel_path.is_some()
    {
        join_under_root(&root.root_rel_dir, "crates/assertions")
    } else if root.runtime_rel_dir == root.root_rel_dir {
        join_under_root(&root.root_rel_dir, "assertions")
    } else {
        format!("{}/assertions", parent_dir(&root.runtime_rel_dir))
    };
    let assertions_cargo_rel_path = format!("{assertions_rel_dir}/Cargo.toml");
    let assertions_manifest = if let Some(failures) = input_failures {
        parse_optional_manifest_lenient(crawl, &assertions_cargo_rel_path, failures)
    } else {
        parse_optional_manifest(crawl, &assertions_cargo_rel_path)?
    };

    Ok(AssertionsLayout {
        assertions_rel_dir,
        assertions_cargo_rel_path: assertions_cargo_rel_path.clone(),
        assertions_manifest,
        nested_assertions_cargo_rel_path: nested_assertions_cargo_rel_path
            .filter(|nested| nested != &assertions_cargo_rel_path),
    })
}

/// `rust_crate_name` function.
pub(crate) fn rust_crate_name(package_name: &str) -> String {
    package_name.replace('-', "_")
}

/// `collect_sidecars` function.
pub(crate) fn collect_sidecars(
    crawl: &G3RsWorkspaceCrawl,
    runtime_rel_dir: &str,
    assertions_rel_dir: &str,
) -> Vec<G3RsTestOwnedSidecarFacts> {
    let src_rel_dir = join_under_root(runtime_rel_dir, "src");
    let mut sidecars = Vec::new();
    for entry in crawl
        .entries
        .iter()
        .filter(|entry| entry.kind == G3RsWorkspaceEntryKind::File)
        .filter(|entry| entry.path.rel_path.starts_with(src_rel_dir.as_str()))
        .filter(|entry| entry.path.rel_path.ends_with("_tests/mod.rs"))
    {
        let dir_rel = parent_dir(&entry.path.rel_path).to_owned();
        let Some(dir_name) = dir_rel.rsplit('/').next() else {
            continue;
        };
        let Some(owner_module_name) = dir_name.strip_suffix("_tests") else {
            continue;
        };
        let sidecar_root_rel = dir_rel
            .strip_prefix(&src_rel_dir)
            .and_then(|rest| rest.strip_prefix('/'))
            .unwrap_or(dir_name);
        let relative_parent = parent_dir(sidecar_root_rel);
        let assertions_src_rel = join_under_root(assertions_rel_dir, "src");
        let assertions_module_rel_path = if relative_parent.is_empty() {
            join_under_root(&assertions_src_rel, &format!("{owner_module_name}.rs"))
        } else {
            join_under_root(
                &assertions_src_rel,
                &format!("{relative_parent}/{owner_module_name}.rs"),
            )
        };
        sidecars.push(G3RsTestOwnedSidecarFacts {
            mod_rel_path: entry.path.rel_path.clone(),
            assertions_module_rel_path,
        });
    }

    sidecars.sort_by(|left, right| left.mod_rel_path.cmp(&right.mod_rel_path));
    sidecars
}

/// `collect_external_harnesses` function.
pub(crate) fn collect_external_harnesses(
    crawl: &G3RsWorkspaceCrawl,
    runtime_rel_dir: &str,
) -> Vec<String> {
    let tests_rel_dir = join_under_root(runtime_rel_dir, "tests");
    let mut files = crawl
        .entries
        .iter()
        .filter(|entry| entry.kind == G3RsWorkspaceEntryKind::File)
        .filter(|entry| entry.path.rel_path.ends_with(".rs"))
        .filter_map(|entry| {
            let rel_path = entry.path.rel_path.as_str();
            let parent = parent_dir(rel_path);
            (parent == tests_rel_dir).then(|| rel_path.to_owned())
        })
        .collect::<Vec<_>>();
    files.sort();
    files
}

/// `dedupe_failures` function.
pub(crate) fn dedupe_failures(input_failures: &mut Vec<G3RsTestFileTreeInputFailure>) {
    input_failures
        .dedup_by(|left, right| left.rel_path == right.rel_path && left.message == right.message);
}

/// `path_is_under` function.
pub(crate) fn path_is_under(rel_path: &str, prefix: &str) -> bool {
    rel_path == prefix
        || rel_path
            .strip_prefix(prefix)
            .is_some_and(|rest| rest.starts_with('/'))
}

/// `file_stem` function.
pub(crate) fn file_stem(rel_path: &str) -> Option<&str> {
    rel_path
        .rsplit('/')
        .next()
        .and_then(|name| name.strip_suffix(".rs"))
}

/// `owner_module_name_from_sidecar_path` function.
pub(crate) fn owner_module_name_from_sidecar_path(rel_after_src: &str) -> Option<String> {
    rel_after_src.split('/').find_map(|segment| {
        segment
            .strip_suffix("_tests")
            .map(str::to_owned)
            .filter(|value| !value.is_empty())
    })
}

/// `is_fixture_path` function.
pub(crate) fn is_fixture_path(rel_path: &str) -> bool {
    rel_path.contains("/tests/fixtures/")
        || rel_path.starts_with("tests/fixtures/")
        || rel_path.contains("_tests/fixtures/")
        || rel_path.contains("assertions/src/fixtures/")
        || rel_path.contains("test_support/src/fixtures/")
}

/// `parse_optional_manifest` function.
fn parse_optional_manifest(
    crawl: &G3RsWorkspaceCrawl,
    rel_path: &str,
) -> Result<Option<CargoToml>, IngestionError> {
    let Some(entry) = g3rs_workspace_crawl::entry(crawl, rel_path) else {
        return Ok(None);
    };
    if !entry.readable {
        return Err(IngestionError::Unreadable {
            path: entry.path.abs_path.clone(),
            reason: "file is not readable".to_owned(),
        });
    }
    let content = crate::fs::read_to_string(&entry.path.abs_path).map_err(|err| {
        IngestionError::Unreadable {
            path: entry.path.abs_path.clone(),
            reason: err.to_string(),
        }
    })?;
    parse(&content)
        .map(Some)
        .map_err(|err| IngestionError::ParseFailed {
            path: entry.path.abs_path.clone(),
            reason: err.to_string(),
        })
}

/// `parse_optional_manifest_lenient` function.
fn parse_optional_manifest_lenient(
    crawl: &G3RsWorkspaceCrawl,
    rel_path: &str,
    input_failures: &mut Vec<G3RsTestFileTreeInputFailure>,
) -> Option<CargoToml> {
    let _entry = g3rs_workspace_crawl::entry(crawl, rel_path)?;
    parse_manifest_lenient(crawl, rel_path, input_failures)
}
