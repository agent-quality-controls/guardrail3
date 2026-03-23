//! Tests for the project walker.
//!
//! Tests construct adversarial temp directories and verify the walker handles
//! each case correctly. The golden fixture lossless test is kept as an
//! integration sanity check but the real coverage comes from the adversarial tests.
#![allow(clippy::expect_used)] // reason: test assertions

use std::collections::BTreeSet;
use std::path::Path;

use guardrail3::adapters::outbound::fs::RealFileSystem;
use guardrail3::app::core::project_walker::walk_project;
use guardrail3::domain::project_tree::ProjectTree;

fn golden_path() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/r_arch_01/golden")
}

/// Create a file in a temp dir, creating parent dirs as needed.
fn write(root: &Path, rel: &str, content: &str) {
    let path = root.join(rel);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("mkdir");
    }
    std::fs::write(&path, content).expect("write");
}

/// Create an empty directory.
fn mkdir(root: &Path, rel: &str) {
    std::fs::create_dir_all(root.join(rel)).expect("mkdir");
}

/// Verify lossless structure roundtrip: every dir and file on disk appears
/// in the tree, and nothing in the tree is phantom.
/// Uses walkdir (independent code path) as ground truth.
/// Only valid for directories WITHOUT .gitignore (walker uses `ignore` crate).
#[allow(dead_code)] // reason: used by golden fixture tests + available for ad-hoc verification
fn assert_lossless_structure(root: &Path, tree: &ProjectTree) {
    // Dirs
    let mut expected_dirs: BTreeSet<String> = BTreeSet::new();
    let mut expected_files: BTreeSet<String> = BTreeSet::new();
    for entry in walkdir::WalkDir::new(root).into_iter().flatten() {
        let rel = entry
            .path()
            .strip_prefix(root)
            .expect("strip")
            .to_string_lossy()
            .into_owned();
        if entry.file_type().is_dir() {
            let _ = expected_dirs.insert(rel);
        } else if entry.file_type().is_file() {
            let _ = expected_files.insert(rel);
        }
    }

    let actual_dirs: BTreeSet<String> = tree.structure.keys().cloned().collect();
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

    // Files
    let mut actual_files: BTreeSet<String> = BTreeSet::new();
    for (dir_rel, entry) in &tree.structure {
        for f in &entry.files {
            let _ = actual_files.insert(ProjectTree::join_rel(dir_rel, f));
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

    // Per-dir children match reality
    for (dir_rel, entry) in &tree.structure {
        let abs = tree.abs_path(dir_rel);
        let mut exp_d: Vec<String> = Vec::new();
        let mut exp_f: Vec<String> = Vec::new();
        for child in std::fs::read_dir(&abs)
            .unwrap_or_else(|e| panic!("read_dir '{dir_rel}': {e}"))
            .flatten()
        {
            let name = child.file_name().to_string_lossy().into_owned();
            if child.file_type().expect("ft").is_dir() {
                exp_d.push(name);
            } else {
                exp_f.push(name);
            }
        }
        exp_d.sort();
        exp_f.sort();
        assert_eq!(entry.dirs, exp_d, "dirs mismatch for '{dir_rel}'");
        assert_eq!(entry.files, exp_f, "files mismatch for '{dir_rel}'");
    }
}

// ============================================================================
// Lossless roundtrip on golden fixture (integration sanity check)
// ============================================================================

/// The golden fixture has no .gitignore inside it, so walkdir and the walker
/// should agree on every dir and file. This is a sanity check, not the
/// primary test — the adversarial tests below are what catch real bugs.
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
                .expect("strip")
                .to_string_lossy()
                .into_owned();
            let _ = expected.insert(rel);
        }
    }
    let actual: BTreeSet<String> = tree.structure.keys().cloned().collect();

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
                .expect("strip")
                .to_string_lossy()
                .into_owned();
            let _ = expected.insert(rel);
        }
    }

    let mut actual: BTreeSet<String> = BTreeSet::new();
    for (dir_rel, entry) in &tree.structure {
        for f in &entry.files {
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

    for (dir_rel, entry) in &tree.structure {
        let abs = tree.abs_path(dir_rel);
        let mut expected_dirs: Vec<String> = Vec::new();
        let mut expected_files: Vec<String> = Vec::new();
        for child in std::fs::read_dir(&abs)
            .unwrap_or_else(|e| panic!("read_dir '{dir_rel}': {e}"))
            .flatten()
        {
            let name = child.file_name().to_string_lossy().into_owned();
            if child.file_type().expect("ft").is_dir() {
                expected_dirs.push(name);
            } else {
                expected_files.push(name);
            }
        }
        expected_dirs.sort();
        expected_files.sort();

        assert_eq!(entry.dirs, expected_dirs, "dirs mismatch for '{dir_rel}'");
        assert_eq!(
            entry.files, expected_files,
            "files mismatch for '{dir_rel}'"
        );
    }
}

#[test]
fn lossless_golden_fixture_content_matches_disk() {
    let root = golden_path();
    let tree = walk_project(&RealFileSystem, &root);

    for (rel, cached) in &tree.content {
        let disk =
            std::fs::read_to_string(root.join(rel)).unwrap_or_else(|e| panic!("read '{rel}': {e}"));
        assert_eq!(cached, &disk, "content mismatch for '{rel}'");
    }
}

// ============================================================================
// Gitignore handling
// ============================================================================

#[test]
fn gitignore_skips_ignored_dirs() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let root = tmp.path();

    // Initialize a git repo so the ignore crate respects .gitignore
    let _ = std::process::Command::new("git")
        .args(["init", "--quiet"])
        .current_dir(root)
        .output()
        .expect("git init");

    write(root, ".gitignore", "ignored_dir/\n");
    write(root, "Cargo.toml", "[workspace]\nmembers = []\n");
    write(root, "visible/hello.rs", "fn main() {}\n");
    write(root, "ignored_dir/secret.toml", "bad = true\n");
    write(root, "ignored_dir/nested/deep.rs", "fn deep() {}\n");

    let tree = walk_project(&RealFileSystem, root);

    // visible/ should be in the tree
    assert!(tree.dir_exists("visible"), "visible/ should exist");
    // ignored_dir/ should NOT be in the tree
    assert!(
        !tree.dir_exists("ignored_dir"),
        "ignored_dir/ should be skipped"
    );
    assert!(
        !tree.content.contains_key("ignored_dir/secret.toml"),
        "ignored file content should not be cached"
    );
}

#[test]
fn gitignore_in_subdirectory() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let root = tmp.path();

    let _ = std::process::Command::new("git")
        .args(["init", "--quiet"])
        .current_dir(root)
        .output()
        .expect("git init");

    // Subdirectory has its own .gitignore
    write(root, "apps/myapp/.gitignore", "build/\n");
    write(
        root,
        "apps/myapp/Cargo.toml",
        "[package]\nname = \"myapp\"\n",
    );
    write(root, "apps/myapp/src/lib.rs", "");
    write(root, "apps/myapp/build/output.bin", "binary");

    let tree = walk_project(&RealFileSystem, root);

    assert!(tree.dir_exists("apps/myapp/src"), "src/ should exist");
    assert!(
        !tree.dir_exists("apps/myapp/build"),
        "build/ should be skipped by subdirectory .gitignore"
    );
    // But Cargo.toml should be cached
    assert!(
        tree.file_content("apps/myapp/Cargo.toml").is_some(),
        "Cargo.toml should be cached"
    );
}

// ============================================================================
// Tracked-but-gitignored files (git ls-files patch)
// ============================================================================

/// Helper: run a git command in a directory.
fn git(root: &Path, args: &[&str]) {
    let output = std::process::Command::new("git")
        .args(args)
        .current_dir(root)
        .env("GIT_AUTHOR_NAME", "test")
        .env("GIT_AUTHOR_EMAIL", "test@test.com")
        .env("GIT_COMMITTER_NAME", "test")
        .env("GIT_COMMITTER_EMAIL", "test@test.com")
        .output()
        .unwrap_or_else(|e| panic!("git {}: {e}", args.join(" ")));
    assert!(
        output.status.success(),
        "git {} failed: {}",
        args.join(" "),
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn tracked_config_in_gitignored_dir_is_visible() {
    // Commit a Cargo.toml inside a dir, THEN add that dir to .gitignore.
    // Walker must still see the Cargo.toml and cache its content.
    let tmp = tempfile::tempdir().expect("tempdir");
    let root = tmp.path();

    // Set up git repo and commit a config file
    git(root, &["init", "--quiet"]);
    write(
        root,
        "hidden_crate/Cargo.toml",
        "[package]\nname = \"hidden\"\nversion = \"0.1.0\"\n",
    );
    write(root, "hidden_crate/src/lib.rs", "pub fn hidden() {}\n");
    write(
        root,
        "visible/Cargo.toml",
        "[package]\nname = \"visible\"\n",
    );
    git(root, &["add", "-A"]);
    git(root, &["commit", "-m", "initial"]);

    // Now gitignore the dir — but files are already tracked
    write(root, ".gitignore", "hidden_crate/\n");
    git(root, &["add", ".gitignore"]);
    git(root, &["commit", "-m", "add gitignore"]);

    let tree = walk_project(&RealFileSystem, root);

    // Tracked file must be visible even though dir is gitignored
    assert!(
        tree.dir_exists("hidden_crate"),
        "hidden_crate/ should be in tree (tracked despite .gitignore)"
    );
    assert!(
        tree.file_content("hidden_crate/Cargo.toml").is_some(),
        "hidden_crate/Cargo.toml should be cached (it's tracked)"
    );
    let cargo = tree.file_content("hidden_crate/Cargo.toml").unwrap();
    assert!(
        cargo.contains("name = \"hidden\""),
        "cached content should match disk"
    );

    // Source file in tracked-but-ignored dir: in structure, NOT cached
    let src = tree
        .dir_contents("hidden_crate/src")
        .expect("src/ should exist");
    assert!(src.has_file("lib.rs"), "lib.rs should be in structure");
    assert!(
        tree.file_content("hidden_crate/src/lib.rs").is_none(),
        "source file should NOT be cached"
    );

    // Visible dir should also be there
    assert!(tree.file_content("visible/Cargo.toml").is_some());
}

#[test]
fn tracked_vs_untracked_in_same_ignored_dir() {
    // Dir has both tracked files (committed before .gitignore) and untracked
    // files (created after .gitignore). Walker sees tracked, skips untracked.
    let tmp = tempfile::tempdir().expect("tempdir");
    let root = tmp.path();

    git(root, &["init", "--quiet"]);
    write(root, "plans/old_plan.md", "# old plan (tracked)\n");
    write(root, "plans/old_config.toml", "key = \"old\"\n");
    git(root, &["add", "-A"]);
    git(root, &["commit", "-m", "initial"]);

    // Gitignore plans/
    write(root, ".gitignore", "plans/\n");
    git(root, &["add", ".gitignore"]);
    git(root, &["commit", "-m", "ignore plans"]);

    // Create new files AFTER gitignore — these are untracked + ignored
    write(
        root,
        "plans/new_plan.md",
        "# new plan (untracked, ignored)\n",
    );
    write(root, "plans/new_config.toml", "key = \"new\"\n");

    let tree = walk_project(&RealFileSystem, root);

    // Tracked files visible
    let plans = tree.dir_contents("plans").expect("plans/ should exist");
    assert!(
        plans.has_file("old_plan.md"),
        "tracked old_plan.md should be visible"
    );
    assert!(
        plans.has_file("old_config.toml"),
        "tracked old_config.toml should be visible"
    );

    // Untracked files NOT visible
    assert!(
        !plans.has_file("new_plan.md"),
        "untracked new_plan.md should NOT be visible"
    );
    assert!(
        !plans.has_file("new_config.toml"),
        "untracked new_config.toml should NOT be visible"
    );
}

#[test]
fn tracked_file_at_root_matching_gitignore() {
    // Commit .env, then add .env to .gitignore. Walker must still see it.
    let tmp = tempfile::tempdir().expect("tempdir");
    let root = tmp.path();

    git(root, &["init", "--quiet"]);
    write(root, ".env", "SECRET=oops\n");
    write(root, "Cargo.toml", "[workspace]\n");
    git(root, &["add", "-A"]);
    git(root, &["commit", "-m", "initial with .env"]);

    write(root, ".gitignore", ".env\n");
    git(root, &["add", ".gitignore"]);
    git(root, &["commit", "-m", "ignore .env"]);

    let tree = walk_project(&RealFileSystem, root);

    let root_entry = tree.dir_contents("").expect("root should exist");
    assert!(
        root_entry.has_file(".env"),
        ".env should be visible (it's tracked)"
    );
    assert!(
        root_entry.has_file("Cargo.toml"),
        "Cargo.toml should be visible"
    );
}

#[test]
fn non_git_project_works_without_patch() {
    // No .git/ — walker works purely via ignore crate, no git ls-files.
    let tmp = tempfile::tempdir().expect("tempdir");
    let root = tmp.path();

    write(root, "Cargo.toml", "[workspace]\n");
    write(root, "src/main.rs", "fn main() {}\n");
    write(root, ".gitignore", "target/\n");
    // .gitignore exists but no git repo — ignore crate still reads patterns
    write(root, "target/debug/binary", "binary");

    let tree = walk_project(&RealFileSystem, root);

    assert!(
        tree.file_content("Cargo.toml").is_some(),
        "Cargo.toml cached"
    );
    assert!(tree.dir_exists("src"), "src/ exists");
    // target/ may or may not be skipped (ignore crate reads .gitignore even
    // without git). What matters: no crash, tree is built.
    assert!(tree.dir_exists(""), "root exists");
}

#[test]
fn deleted_tracked_file_not_in_tree() {
    // A file tracked by git but deleted from disk should NOT appear in tree.
    let tmp = tempfile::tempdir().expect("tempdir");
    let root = tmp.path();

    git(root, &["init", "--quiet"]);
    write(root, "Cargo.toml", "[workspace]\n");
    write(root, "old_file.toml", "will be deleted\n");
    git(root, &["add", "-A"]);
    git(root, &["commit", "-m", "initial"]);

    // Delete from disk but don't git rm — file is tracked but absent
    std::fs::remove_file(root.join("old_file.toml")).expect("rm");

    let tree = walk_project(&RealFileSystem, root);

    let root_entry = tree.dir_contents("").expect("root");
    assert!(
        !root_entry.has_file("old_file.toml"),
        "deleted file should NOT be in tree even if tracked"
    );
}

// ============================================================================
// Config vs source classification
// ============================================================================

#[test]
fn caches_config_not_source() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let root = tmp.path();

    write(root, "Cargo.toml", "[workspace]\n");
    write(root, "src/main.rs", "fn main() {}\n");
    write(root, "src/lib.rs", "pub fn hello() {}\n");
    write(root, "clippy.toml", "max-struct-bools = 3\n");
    write(root, "deny.toml", "[graph]\n");
    write(root, "apps/api/Cargo.toml", "[package]\nname = \"api\"\n");
    write(root, "apps/api/src/lib.rs", "");

    let tree = walk_project(&RealFileSystem, root);

    // Config files: cached
    assert!(tree.file_content("Cargo.toml").is_some());
    assert!(tree.file_content("clippy.toml").is_some());
    assert!(tree.file_content("deny.toml").is_some());
    assert!(tree.file_content("apps/api/Cargo.toml").is_some());

    // Source files: in structure but NOT cached
    let src = tree.dir_contents("src").expect("src/ should exist");
    assert!(src.has_file("main.rs"));
    assert!(src.has_file("lib.rs"));
    assert!(tree.file_content("src/main.rs").is_none());
    assert!(tree.file_content("src/lib.rs").is_none());
    assert!(tree.file_content("apps/api/src/lib.rs").is_none());
}

#[test]
fn caches_all_cargo_tomls_at_every_depth() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let root = tmp.path();

    write(root, "Cargo.toml", "[workspace]\nmembers = []\n");
    write(root, "apps/api/Cargo.toml", "[workspace]\nmembers = []\n");
    write(
        root,
        "apps/api/crates/domain/types/Cargo.toml",
        "[package]\nname = \"types\"\n",
    );
    write(
        root,
        "apps/api/crates/adapters/inbound/rest/Cargo.toml",
        "[package]\nname = \"rest\"\n",
    );
    write(
        root,
        "packages/shared/Cargo.toml",
        "[package]\nname = \"shared\"\n",
    );

    let tree = walk_project(&RealFileSystem, root);

    assert!(tree.file_content("Cargo.toml").is_some());
    assert!(tree.file_content("apps/api/Cargo.toml").is_some());
    assert!(
        tree.file_content("apps/api/crates/domain/types/Cargo.toml")
            .is_some()
    );
    assert!(
        tree.file_content("apps/api/crates/adapters/inbound/rest/Cargo.toml")
            .is_some()
    );
    assert!(tree.file_content("packages/shared/Cargo.toml").is_some());
}

#[test]
fn caches_gitkeep_files() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let root = tmp.path();

    write(root, "apps/api/crates/ports/inbound/.gitkeep", "");

    let tree = walk_project(&RealFileSystem, root);

    let entry = tree
        .dir_contents("apps/api/crates/ports/inbound")
        .expect("ports/inbound should exist");
    assert!(entry.has_file(".gitkeep"));
    assert!(
        tree.file_content("apps/api/crates/ports/inbound/.gitkeep")
            .is_some()
    );
}

#[test]
fn caches_package_json() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let root = tmp.path();

    write(root, "package.json", r#"{"name": "root", "private": true}"#);
    write(root, "apps/web/package.json", r#"{"name": "web"}"#);

    let tree = walk_project(&RealFileSystem, root);

    assert!(tree.file_content("package.json").is_some());
    assert!(tree.file_content("apps/web/package.json").is_some());
}

#[test]
fn caches_gitignore_files() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let root = tmp.path();

    write(root, ".gitignore", "Cargo.lock\n");
    write(root, "apps/api/.gitignore", "Cargo.lock\n");

    let tree = walk_project(&RealFileSystem, root);

    assert!(tree.file_content(".gitignore").is_some());
    assert!(tree.file_content("apps/api/.gitignore").is_some());
}

#[test]
fn caches_workflow_yamls() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let root = tmp.path();

    write(root, ".github/workflows/ci.yml", "name: CI\n");
    write(root, ".github/workflows/release.yaml", "name: Release\n");
    // Non-workflow yaml should NOT be cached
    write(root, "config/settings.yml", "key: value\n");

    let tree = walk_project(&RealFileSystem, root);

    assert!(tree.file_content(".github/workflows/ci.yml").is_some());
    assert!(
        tree.file_content(".github/workflows/release.yaml")
            .is_some()
    );
    assert!(
        tree.file_content("config/settings.yml").is_none(),
        "non-workflow yaml should not be cached"
    );
}

#[test]
fn caches_eslint_variants() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let root = tmp.path();

    write(root, "eslint.config.mjs", "export default []\n");

    let tree = walk_project(&RealFileSystem, root);
    assert!(tree.file_content("eslint.config.mjs").is_some());
}

// ============================================================================
// Structure edge cases
// ============================================================================

#[test]
fn empty_directory_captured() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let root = tmp.path();

    mkdir(root, "apps/api/crates/domain");
    // domain/ is empty — no files, no subdirs

    let tree = walk_project(&RealFileSystem, root);

    assert!(
        tree.dir_exists("apps/api/crates/domain"),
        "empty dir should exist"
    );
    let entry = tree
        .dir_contents("apps/api/crates/domain")
        .expect("should exist");
    assert!(entry.dirs.is_empty(), "no subdirs");
    assert!(entry.files.is_empty(), "no files");
}

#[test]
fn deeply_nested_structure() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let root = tmp.path();

    // 10 levels deep
    write(root, "a/b/c/d/e/f/g/h/i/j/leaf.toml", "deep = true\n");

    let tree = walk_project(&RealFileSystem, root);

    assert!(tree.dir_exists("a/b/c/d/e/f/g/h/i/j"));
    let j = tree
        .dir_contents("a/b/c/d/e/f/g/h/i/j")
        .expect("j/ should exist");
    assert!(j.has_file("leaf.toml"));
}

#[test]
fn hidden_dirs_entered() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let root = tmp.path();

    // Walker has hidden(false) — should enter .hidden dirs
    write(root, ".guardrail3/overrides/clippy.toml", "extra = true\n");
    write(root, ".github/workflows/ci.yml", "name: CI\n");

    let tree = walk_project(&RealFileSystem, root);

    assert!(tree.dir_exists(".guardrail3/overrides"));
    assert!(tree.dir_exists(".github/workflows"));
    assert!(tree.file_content(".github/workflows/ci.yml").is_some());
}

#[test]
fn root_entry_is_empty_string() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let root = tmp.path();

    write(root, "Cargo.toml", "[workspace]\n");

    let tree = walk_project(&RealFileSystem, root);

    // Root dir is keyed as ""
    assert!(tree.dir_exists(""));
    let root_entry = tree.dir_contents("").expect("root should exist");
    assert!(root_entry.has_file("Cargo.toml"));
}

#[test]
fn children_are_sorted() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let root = tmp.path();

    // Create in non-alphabetical order
    write(root, "zebra/file.rs", "");
    write(root, "alpha/file.rs", "");
    write(root, "middle/file.rs", "");
    write(root, "z_file.txt", "");
    write(root, "a_file.txt", "");

    let tree = walk_project(&RealFileSystem, root);
    let root_entry = tree.dir_contents("").expect("root");

    assert_eq!(root_entry.dirs, vec!["alpha", "middle", "zebra"]);
    assert_eq!(root_entry.files, vec!["a_file.txt", "z_file.txt"]);
}

// ============================================================================
// ProjectTree API
// ============================================================================

#[test]
fn join_rel_handles_root() {
    assert_eq!(ProjectTree::join_rel("", "apps"), "apps");
    assert_eq!(ProjectTree::join_rel("apps", "devctl"), "apps/devctl");
    assert_eq!(ProjectTree::join_rel("a/b/c", "d"), "a/b/c/d");
}

#[test]
fn abs_path_works() {
    let root = golden_path();
    let tree = walk_project(&RealFileSystem, &root);
    assert_eq!(tree.abs_path(""), root);
    assert_eq!(tree.abs_path("apps/devctl"), root.join("apps/devctl"));
}

#[test]
fn json_roundtrip() {
    let tree = walk_project(&RealFileSystem, &golden_path());
    let json = serde_json::to_string_pretty(&tree).expect("serialize");
    let roundtrip: ProjectTree = serde_json::from_str(&json).expect("deserialize");

    assert_eq!(tree.structure.len(), roundtrip.structure.len());
    assert_eq!(tree.content.len(), roundtrip.content.len());
    for (key, entry) in &tree.structure {
        let rt = roundtrip
            .structure
            .get(key)
            .expect("key missing in roundtrip");
        assert_eq!(entry.dirs, rt.dirs, "dirs mismatch for '{key}'");
        assert_eq!(entry.files, rt.files, "files mismatch for '{key}'");
    }
    for (key, val) in &tree.content {
        assert_eq!(
            val,
            roundtrip.content.get(key).expect("missing"),
            "content mismatch for '{key}'"
        );
    }
}

// ============================================================================
// Lossless on ANY git repo (walkdir + git check-ignore as independent verifier)
// ============================================================================

/// Independent ground truth: walk with walkdir (raw, no gitignore), then ask
/// git which paths are ignored, subtract those. Result = everything the walker
/// should see. Completely independent code path from the `ignore` crate.
///
/// Returns (expected_dirs, expected_files) as sorted BTreeSets of relative paths.
#[allow(clippy::type_complexity)] // reason: return type is clear in context
fn independent_walk(root: &Path) -> (BTreeSet<String>, BTreeSet<String>) {
    // 1. Raw walk — every dir and file on disk (skip .git/)
    let mut all_dirs: Vec<String> = Vec::new();
    let mut all_files: Vec<String> = Vec::new();

    for entry in walkdir::WalkDir::new(root)
        .into_iter()
        .filter_entry(|e| {
            // Skip .git/ — both walker and verifier agree to exclude it
            e.file_name() != ".git"
        })
        .flatten()
    {
        let rel = entry
            .path()
            .strip_prefix(root)
            .expect("strip prefix")
            .to_string_lossy()
            .into_owned();
        if entry.file_type().is_dir() {
            all_dirs.push(rel);
        } else if entry.file_type().is_file() {
            all_files.push(rel);
        }
    }

    // 2. Ask git which paths are ignored (batch call)
    let mut all_paths: Vec<String> = Vec::new();
    for d in &all_dirs {
        if !d.is_empty() {
            all_paths.push(d.clone());
        }
    }
    all_paths.extend(all_files.iter().cloned());

    let ignored: BTreeSet<String> = if all_paths.is_empty() {
        BTreeSet::new()
    } else {
        // Write paths to a temp file to avoid pipe deadlock with large inputs
        let tmp_input = tempfile::NamedTempFile::new().expect("create temp file");
        std::fs::write(tmp_input.path(), all_paths.join("\n")).expect("write paths");
        let output = std::process::Command::new("git")
            .args(["check-ignore", "--stdin"])
            .current_dir(root)
            .stdin(std::fs::File::open(tmp_input.path()).expect("open temp"))
            .output()
            .expect("git check-ignore");
        let ignored_set: BTreeSet<String> = String::from_utf8_lossy(&output.stdout)
            .lines()
            .filter(|s| !s.is_empty())
            .map(ToOwned::to_owned)
            .collect();
        eprintln!(
            "git check-ignore: {} paths checked, {} ignored, exit={:?}",
            all_paths.len(),
            ignored_set.len(),
            output.status.code()
        );
        ignored_set
    };

    // 3. Filter: remove ignored paths and paths whose parent is ignored
    let is_under_ignored = |path: &str| -> bool {
        let mut cur = Path::new(path);
        while let Some(parent) = cur.parent() {
            let p = parent.to_string_lossy();
            if p.is_empty() {
                break;
            }
            if ignored.contains(p.as_ref()) {
                return true;
            }
            cur = parent;
        }
        false
    };

    let expected_dirs: BTreeSet<String> = all_dirs
        .into_iter()
        .filter(|d| !ignored.contains(d) && !is_under_ignored(d))
        .collect();

    let expected_files: BTreeSet<String> = all_files
        .into_iter()
        .filter(|f| !ignored.contains(f) && !is_under_ignored(f))
        .collect();

    (expected_dirs, expected_files)
}

/// Extract (dirs, files) from walker output for comparison.
fn walker_sets(tree: &ProjectTree) -> (BTreeSet<String>, BTreeSet<String>) {
    let dirs: BTreeSet<String> = tree.structure.keys().cloned().collect();
    let mut files: BTreeSet<String> = BTreeSet::new();
    for (dir_rel, entry) in &tree.structure {
        for f in &entry.files {
            let _ = files.insert(ProjectTree::join_rel(dir_rel, f));
        }
    }
    (dirs, files)
}

/// Assert two path sets match. On mismatch, prints detailed diff.
fn assert_sets_match(label: &str, expected: &BTreeSet<String>, actual: &BTreeSet<String>) {
    let missing: Vec<_> = expected.difference(actual).collect();
    let extra: Vec<_> = actual.difference(expected).collect();

    if !missing.is_empty() {
        eprintln!(
            "\nMissing {label} (expected but walker doesn't have) — {}/{} shown:",
            missing.len().min(50),
            missing.len()
        );
        for p in missing.iter().take(50) {
            eprintln!("  - {p}");
        }
    }
    if !extra.is_empty() {
        eprintln!(
            "\nExtra {label} (walker has but not expected) — {}/{} shown:",
            extra.len().min(50),
            extra.len()
        );
        for p in extra.iter().take(50) {
            eprintln!("  + {p}");
        }
    }

    assert!(
        missing.is_empty() && extra.is_empty(),
        "{label} mismatch: {} missing, {} extra",
        missing.len(),
        extra.len()
    );
}

/// Lossless structure test on steady-parent.
/// Independent verifier: walkdir + `git check-ignore --stdin`.
/// Tests file-for-file, dir-for-dir match.
///
/// Run with: cargo test --test unit lossless_real_project -- --ignored --nocapture
#[test]
#[ignore] // reason: requires steady-parent repo at known path
fn lossless_real_project() {
    let root = Path::new("/Users/tartakovsky/Projects/websmasher/websmasher");
    if !root.join(".git").exists() {
        eprintln!("Skipping: steady-parent repo not found");
        return;
    }

    let (expected_dirs, expected_files) = independent_walk(root);
    eprintln!(
        "Independent: {} files, {} dirs",
        expected_files.len(),
        expected_dirs.len()
    );

    let tree = walk_project(&RealFileSystem, root);
    let (walker_dirs, walker_files) = walker_sets(&tree);
    eprintln!(
        "Walker: {} files, {} dirs",
        walker_files.len(),
        walker_dirs.len()
    );

    assert_sets_match("files", &expected_files, &walker_files);
    assert_sets_match("dirs", &expected_dirs, &walker_dirs);
}

/// Lossless structure test on guardrail3 itself.
/// Same independent verifier, different project.
///
/// Run with: cargo test --test unit lossless_self -- --ignored --nocapture
#[test]
#[ignore] // reason: runs on the guardrail3 repo, slower than unit tests
fn lossless_self() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .expect("guardrail3 repo root");
    if !root.join(".git").exists() {
        eprintln!("Skipping: not in a git repo");
        return;
    }

    let (expected_dirs, expected_files) = independent_walk(root);
    eprintln!(
        "Independent: {} files, {} dirs",
        expected_files.len(),
        expected_dirs.len()
    );

    let tree = walk_project(&RealFileSystem, root);
    let (walker_dirs, walker_files) = walker_sets(&tree);
    eprintln!(
        "Walker: {} files, {} dirs",
        walker_files.len(),
        walker_dirs.len()
    );

    assert_sets_match("files", &expected_files, &walker_files);
    assert_sets_match("dirs", &expected_dirs, &walker_dirs);
}

/// Lossless content test: every cached file matches disk, independently verified.
/// And: every config file on disk is cached (none missing).
///
/// Run with: cargo test --test unit lossless_content_real -- --ignored --nocapture
#[test]
#[ignore] // reason: requires steady-parent repo
fn lossless_content_real() {
    let root = Path::new("/Users/tartakovsky/Projects/websmasher/websmasher");
    if !root.join(".git").exists() {
        eprintln!("Skipping: steady-parent repo not found");
        return;
    }

    let tree = walk_project(&RealFileSystem, root);

    // Every cached file must match disk (read independently with std::fs)
    let mut content_mismatches: Vec<String> = Vec::new();
    for (rel, cached) in &tree.content {
        let abs = root.join(rel);
        match std::fs::read_to_string(&abs) {
            Ok(disk) => {
                if cached != &disk {
                    content_mismatches.push(format!("{rel}: content differs"));
                }
            }
            Err(e) => {
                content_mismatches.push(format!("{rel}: can't read from disk: {e}"));
            }
        }
    }
    assert!(
        content_mismatches.is_empty(),
        "Content mismatches:\n{}",
        content_mismatches.join("\n")
    );
    eprintln!("{} cached files verified against disk", tree.content.len());

    // No source files leaked into content
    let source_exts = ["rs", "ts", "tsx", "js", "jsx", "mjs", "cjs"];
    let leaked: Vec<_> = tree
        .content
        .keys()
        .filter(|p| {
            let ext = Path::new(p)
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("");
            source_exts.contains(&ext)
                // Exception: config files that happen to have source extensions
                && !p.contains("eslint.config.")
                && !p.contains("vitest.config.")
                && !p.contains("jest.config.")
                && !p.contains("stryker.config.")
                && !p.contains("next.config.")
                && !p.contains("velite.config.")
                && !p.contains("stylelint.config.")
        })
        .collect();
    assert!(
        leaked.is_empty(),
        "Source files leaked into content cache:\n{}",
        leaked
            .iter()
            .map(|f| format!("  {f}"))
            .collect::<Vec<_>>()
            .join("\n")
    );
}
