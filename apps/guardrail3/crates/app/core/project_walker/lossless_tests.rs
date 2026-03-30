//! Tests for the project walker.
//!
//! Tests construct adversarial temp directories and verify the walker handles
//! each case correctly. The golden fixture lossless test is kept as an
//! integration sanity check but the real coverage comes from the adversarial tests.
use std::collections::BTreeSet;
use std::path::Path;

use guardrail3_adapters_outbound_fs::RealFileSystem;
use guardrail3_domain_project_tree::ProjectTree;

use super::walk_project;

fn golden_path() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../tests/fixtures/r_arch_01/golden")
}

/// Verify lossless structure roundtrip: every dir and file on disk appears
/// in the tree, and nothing in the tree is phantom.
/// Uses walkdir (independent code path) as ground truth.
/// Only valid for directories WITHOUT .gitignore (walker uses `ignore` crate).
fn assert_lossless_structure(root: &Path, tree: &ProjectTree) {
    let mut expected_dirs: BTreeSet<String> = BTreeSet::new();
    let mut expected_files: BTreeSet<String> = BTreeSet::new();
    for entry in walkdir::WalkDir::new(root).into_iter().flatten() {
        let rel = entry
            .path()
            .strip_prefix(root)
            .unwrap_or_else(|_| panic!("walkdir entry should remain under the golden root"))
            .to_string_lossy()
            .into_owned();
        if entry.file_type().is_dir() {
            let _ = expected_dirs.insert(rel);
        } else if entry.file_type().is_file() {
            let _ = expected_files.insert(rel);
        }
    }

    let actual_dirs: BTreeSet<String> = tree.structure().keys().cloned().collect();
    let missing_dirs: Vec<_> = expected_dirs.difference(&actual_dirs).collect();
    let extra_dirs: Vec<_> = actual_dirs.difference(&expected_dirs).collect();
    assert!(
        missing_dirs.is_empty(),
        "Walker missed dirs:\n{missing_dirs:#?}"
    );
    assert!(
        extra_dirs.is_empty(),
        "Walker has phantom dirs:\n{extra_dirs:#?}"
    );

    let mut actual_files: BTreeSet<String> = BTreeSet::new();
    for (dir_rel, entry) in tree.structure() {
        for file in entry.files() {
            let _ = actual_files.insert(ProjectTree::join_rel(dir_rel, file));
        }
    }
    let missing_files: Vec<_> = expected_files.difference(&actual_files).collect();
    let extra_files: Vec<_> = actual_files.difference(&expected_files).collect();
    assert!(
        missing_files.is_empty(),
        "Walker missed files:\n{missing_files:#?}"
    );
    assert!(
        extra_files.is_empty(),
        "Walker has phantom files:\n{extra_files:#?}"
    );

    for (dir_rel, entry) in tree.structure() {
        let abs = tree.abs_path(dir_rel);
        let mut exp_d: Vec<String> = Vec::new();
        let mut exp_f: Vec<String> = Vec::new();
        for child in std::fs::read_dir(&abs)
            .unwrap_or_else(|e| panic!("read_dir '{dir_rel}': {e}"))
            .flatten()
        {
            let name = child.file_name().to_string_lossy().into_owned();
            if child
                .file_type()
                .unwrap_or_else(|e| {
                    panic!("failed to read child file type while verifying golden fixture: {e}")
                })
                .is_dir()
            {
                exp_d.push(name);
            } else {
                exp_f.push(name);
            }
        }
        exp_d.sort();
        exp_f.sort();
        assert_eq!(entry.dirs(), exp_d, "dirs mismatch for '{dir_rel}'");
        assert_eq!(entry.files(), exp_f, "files mismatch for '{dir_rel}'");
    }
}

#[test]
fn lossless_golden_fixture_dirs() {
    let root = golden_path();
    let tree = walk_project(&RealFileSystem, &root);

    let mut expected: BTreeSet<String> = BTreeSet::new();
    for entry in walkdir::WalkDir::new(&root).into_iter().flatten() {
        if entry.file_type().is_dir() {
            let rel = entry
                .path()
                .strip_prefix(&root)
                .unwrap_or_else(|_| panic!("walkdir entry should remain under the golden root"))
                .to_string_lossy()
                .into_owned();
            let _ = expected.insert(rel);
        }
    }
    let actual: BTreeSet<String> = tree.structure().keys().cloned().collect();

    let missing: Vec<_> = expected.difference(&actual).collect();
    let extra: Vec<_> = actual.difference(&expected).collect();
    assert!(missing.is_empty(), "Walker missed dirs:\n{missing:#?}");
    assert!(extra.is_empty(), "Walker has phantom dirs:\n{extra:#?}");
}

#[test]
fn lossless_golden_fixture_files() {
    let root = golden_path();
    let tree = walk_project(&RealFileSystem, &root);

    let mut expected: BTreeSet<String> = BTreeSet::new();
    for entry in walkdir::WalkDir::new(&root).into_iter().flatten() {
        if entry.file_type().is_file() {
            let rel = entry
                .path()
                .strip_prefix(&root)
                .unwrap_or_else(|_| panic!("walkdir entry should remain under the golden root"))
                .to_string_lossy()
                .into_owned();
            let _ = expected.insert(rel);
        }
    }

    let mut actual: BTreeSet<String> = BTreeSet::new();
    for (dir_rel, entry) in tree.structure() {
        for f in entry.files() {
            let _ = actual.insert(ProjectTree::join_rel(dir_rel, f));
        }
    }

    let missing: Vec<_> = expected.difference(&actual).collect();
    let extra: Vec<_> = actual.difference(&expected).collect();
    assert!(missing.is_empty(), "Walker missed files:\n{missing:#?}");
    assert!(extra.is_empty(), "Walker has phantom files:\n{extra:#?}");
}

#[test]
fn lossless_golden_fixture_per_dir_children() {
    let root = golden_path();
    let tree = walk_project(&RealFileSystem, &root);

    for (dir_rel, entry) in tree.structure() {
        let abs = tree.abs_path(dir_rel);
        let mut expected_dirs: Vec<String> = Vec::new();
        let mut expected_files: Vec<String> = Vec::new();
        for child in std::fs::read_dir(&abs)
            .unwrap_or_else(|e| panic!("read_dir '{dir_rel}': {e}"))
            .flatten()
        {
            let name = child.file_name().to_string_lossy().into_owned();
            if child
                .file_type()
                .unwrap_or_else(|e| {
                    panic!("failed to read child file type while verifying golden fixture: {e}")
                })
                .is_dir()
            {
                expected_dirs.push(name);
            } else {
                expected_files.push(name);
            }
        }
        expected_dirs.sort();
        expected_files.sort();

        assert_eq!(entry.dirs(), expected_dirs, "dirs mismatch for '{dir_rel}'");
        assert_eq!(
            entry.files(),
            expected_files,
            "files mismatch for '{dir_rel}'"
        );
    }
}

#[test]
fn lossless_golden_fixture_content_matches_disk() {
    let root = golden_path();
    let tree = walk_project(&RealFileSystem, &root);

    for (rel, cached) in tree.content() {
        let disk =
            std::fs::read_to_string(root.join(rel)).unwrap_or_else(|e| panic!("read '{rel}': {e}"));
        assert_eq!(cached, &disk, "content mismatch for '{rel}'");
    }
}

#[test]
fn lossless_golden_fixture_helper_matches_disk_and_structure() {
    let root = golden_path();
    let tree = walk_project(&RealFileSystem, &root);

    assert!(
        !tree.structure().is_empty(),
        "golden fixture should produce a non-empty project tree"
    );
    assert_lossless_structure(&root, &tree);
}
