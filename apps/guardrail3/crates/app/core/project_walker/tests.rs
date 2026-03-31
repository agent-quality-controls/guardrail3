use std::path::{Path, PathBuf};

use tempfile::tempdir;

use guardrail3_adapters_outbound_fs::RealFileSystem;

use super::walk_project;

#[test]
fn preserves_immediate_ignored_file_children_in_discovered_dirs() {
    let tmp = tempdir().expect("failed to create temporary project root");
    guardrail3_shared_fs::create_dir_all(&tmp.path().join("apps/devctl/crates/app/core"))
        .expect("failed to create project fixture directories");
    guardrail3_shared_fs::write_file(&tmp.path().join(".gitignore"), "*.env\n")
        .expect("failed to write project fixture .gitignore");
    guardrail3_shared_fs::write_file(&tmp.path().join("apps/devctl/Cargo.toml"), "[workspace]\n")
        .expect("failed to write project fixture Cargo.toml");
    guardrail3_shared_fs::write_file(&tmp.path().join("apps/devctl/crates/app/.env"), "SECRET=1")
        .expect("failed to write ignored environment fixture");
    guardrail3_shared_fs::write_file(
        &tmp.path().join("apps/devctl/crates/app/core/lib.rs"),
        "pub fn ready() {}\n",
    )
    .expect("failed to write project fixture Rust source");

    let tree = walk_project(&RealFileSystem, tmp.path());
    let entry = tree
        .dir_contents("apps/devctl/crates/app")
        .expect("app container discovered");

    assert!(
        entry.files().iter().any(|file| file == ".env"),
        "ignored immediate file should still be visible in ProjectTree: {entry:#?}"
    );
}

#[test]
fn preserves_immediate_ignored_directory_children_in_discovered_dirs() {
    let tmp = tempdir().expect("failed to create temporary project root");
    guardrail3_shared_fs::create_dir_all(&tmp.path().join("apps/devctl/crates/app/core"))
        .expect("failed to create project fixture directories");
    guardrail3_shared_fs::write_file(&tmp.path().join(".gitignore"), "orphan/\n")
        .expect("failed to write project fixture .gitignore");
    guardrail3_shared_fs::write_file(&tmp.path().join("apps/devctl/Cargo.toml"), "[workspace]\n")
        .expect("failed to write project fixture Cargo.toml");
    guardrail3_shared_fs::create_dir_all(&tmp.path().join("apps/devctl/crates/app/orphan/src"))
        .expect("failed to create ignored fixture directory");
    guardrail3_shared_fs::write_file(
        &tmp.path().join("apps/devctl/crates/app/orphan/src/lib.rs"),
        "pub fn ignored_leaf() {}\n",
    )
    .expect("failed to write project fixture Rust source");
    guardrail3_shared_fs::write_file(
        &tmp.path().join("apps/devctl/crates/app/core/lib.rs"),
        "pub fn ready() {}\n",
    )
    .expect("failed to write project fixture Rust source");

    let tree = walk_project(&RealFileSystem, tmp.path());
    let entry = tree
        .dir_contents("apps/devctl/crates/app")
        .expect("app container discovered");

    assert!(
        entry.dirs().iter().any(|dir| dir == "orphan")
            && tree.dir_exists("apps/devctl/crates/app/orphan"),
        "ignored immediate child directory should still be visible in ProjectTree: {entry:#?}"
    );
}

#[test]
fn caches_repo_local_cargo_config_files() {
    let tmp = tempdir().expect("failed to create temporary project root");
    guardrail3_shared_fs::create_dir_all(&tmp.path().join(".cargo")).expect("create cargo dir");
    guardrail3_shared_fs::write_file(
        &tmp.path().join(".cargo/config.toml"),
        "[env]\nCLIPPY_CONF_DIR = \".\"\n",
    )
    .expect("write cargo config toml");
    guardrail3_shared_fs::write_file(
        &tmp.path().join(".cargo/config"),
        "[env]\nRUSTFLAGS = \"-Dwarnings\"\n",
    )
    .expect("write cargo config");

    let tree = walk_project(&RealFileSystem, tmp.path());

    assert_eq!(
        tree.file_content(".cargo/config.toml"),
        Some("[env]\nCLIPPY_CONF_DIR = \".\"\n"),
    );
    assert_eq!(
        tree.file_content(".cargo/config"),
        Some("[env]\nRUSTFLAGS = \"-Dwarnings\"\n"),
    );
}

#[test]
fn recursively_scans_newly_recovered_ignored_directories() {
    let tmp = tempdir().expect("failed to create temporary project root");
    guardrail3_shared_fs::create_dir_all(&tmp.path().join("apps/devctl/crates/app/core"))
        .expect("failed to create project fixture directories");
    guardrail3_shared_fs::write_file(&tmp.path().join(".gitignore"), "valid_crate/\n")
        .expect("failed to write project fixture .gitignore");
    guardrail3_shared_fs::write_file(&tmp.path().join("apps/devctl/Cargo.toml"), "[workspace]\n")
        .expect("failed to write project fixture Cargo.toml");
    guardrail3_shared_fs::create_dir_all(&tmp.path().join("apps/devctl/crates/app/valid_crate"))
        .expect("failed to create ignored fixture directory");
    guardrail3_shared_fs::write_file(
        &tmp.path()
            .join("apps/devctl/crates/app/valid_crate/Cargo.toml"),
        "[package]\nname = \"valid-crate\"\nversion = \"0.1.0\"\n",
    )
    .expect("failed to write project fixture Cargo.toml");
    guardrail3_shared_fs::write_file(
        &tmp.path().join("apps/devctl/crates/app/core/lib.rs"),
        "pub fn ready() {}\n",
    )
    .expect("failed to write project fixture Rust source");

    let tree = walk_project(&RealFileSystem, tmp.path());
    let entry = tree
        .dir_contents("apps/devctl/crates/app/valid_crate")
        .expect("recovered leaf dir discovered");

    assert!(
        entry.files().iter().any(|file| file == "Cargo.toml"),
        "recovered ignored directories should also have their immediate children scanned: {entry:#?}"
    );
}

#[test]
fn recovers_ignored_untracked_manifest_files_anywhere_under_root() {
    let tmp = tempdir().expect("failed to create temporary project root");
    guardrail3_shared_fs::create_dir_all(&tmp.path().join("ignored/nested/crate"))
        .expect("failed to create ignored manifest fixture directories");
    guardrail3_shared_fs::write_file(&tmp.path().join(".gitignore"), "ignored/\n")
        .expect("failed to write project fixture .gitignore");
    guardrail3_shared_fs::write_file(
        &tmp.path().join("ignored/nested/crate/Cargo.toml"),
        "[package]\nname = \"ignored-crate\"\nversion = \"0.1.0\"\n",
    )
    .expect("failed to write ignored Cargo.toml");
    guardrail3_shared_fs::write_file(
        &tmp.path().join("ignored/nested/crate/guardrail3.toml"),
        "[profile]\nname = \"service\"\n",
    )
    .expect("failed to write ignored guardrail3.toml");

    let tree = walk_project(&RealFileSystem, tmp.path());

    assert!(
        tree.file_exists("ignored/nested/crate/Cargo.toml"),
        "ignored nested Cargo.toml should be recovered into ProjectTree"
    );
    assert_eq!(
        tree.file_content("ignored/nested/crate/Cargo.toml"),
        Some("[package]\nname = \"ignored-crate\"\nversion = \"0.1.0\"\n"),
    );
    assert_eq!(
        tree.file_content("ignored/nested/crate/guardrail3.toml"),
        Some("[profile]\nname = \"service\"\n"),
    );
}

#[test]
fn recovers_ignored_untracked_typescript_source_files_anywhere_under_root() {
    let tmp = tempdir().expect("failed to create temporary project root");
    guardrail3_shared_fs::create_dir_all(&tmp.path().join("ignored/web/src"))
        .expect("failed to create ignored typescript fixture directories");
    guardrail3_shared_fs::write_file(&tmp.path().join(".gitignore"), "ignored/\n")
        .expect("failed to write project fixture .gitignore");
    guardrail3_shared_fs::write_file(
        &tmp.path().join("ignored/web/src/index.ts"),
        "export const ready = true;\n",
    )
    .expect("failed to write ignored TypeScript source");

    let tree = walk_project(&RealFileSystem, tmp.path());
    let entry = tree
        .dir_contents("ignored/web/src")
        .expect("ignored typescript source parent directory should be recovered");

    assert!(
        entry.files().iter().any(|file| file == "index.ts"),
        "ignored TypeScript source should be recovered into ProjectTree: {entry:#?}"
    );
}

#[test]
fn finds_mutated_rust_toolchain_files_in_golden_fixture_tree() {
    let tmp = copy_rust_golden_fixture();

    write_fixture_file(
        tmp.path(),
        "rust-toolchain.toml",
        "[toolchain]\nchannel = \"stable\"\ncomponents = [\"clippy\", \"rustfmt\"]\n",
    );
    write_fixture_file(
        tmp.path(),
        "apps/backend/rust-toolchain.toml",
        "[toolchain]\nchannel = \"1.85.1\"\ncomponents = [\"clippy\", \"rustfmt\"]\n",
    );
    write_fixture_file(
        tmp.path(),
        "apps/backend/crates/domain/engine/rust-toolchain.toml",
        "[toolchain]\nchannel = \"1.85.1\"\ncomponents = [\"clippy\", \"rustfmt\"]\n",
    );
    write_fixture_file(
        tmp.path(),
        "apps/admin/rust-toolchain.toml",
        "[toolchain]\nchannel = \"stable\"\ncomponents = [\"clippy\", \"rustfmt\"]\n",
    );
    write_fixture_file(tmp.path(), "packages/ui-kit/rust-toolchain", "stable\n");

    let tree = walk_project(&RealFileSystem, tmp.path());

    for rel_path in [
        "rust-toolchain.toml",
        "apps/backend/rust-toolchain.toml",
        "apps/backend/crates/domain/engine/rust-toolchain.toml",
        "apps/admin/rust-toolchain.toml",
        "packages/ui-kit/rust-toolchain",
    ] {
        assert!(
            tree.file_exists(rel_path),
            "ProjectTree should contain `{rel_path}` after golden-fixture mutation"
        );
    }

    assert_eq!(
        tree.file_content("rust-toolchain.toml"),
        Some("[toolchain]\nchannel = \"stable\"\ncomponents = [\"clippy\", \"rustfmt\"]\n"),
    );
    assert_eq!(
        tree.file_content("apps/backend/rust-toolchain.toml"),
        Some("[toolchain]\nchannel = \"1.85.1\"\ncomponents = [\"clippy\", \"rustfmt\"]\n"),
    );
    assert_eq!(
        tree.file_content("apps/backend/crates/domain/engine/rust-toolchain.toml"),
        Some("[toolchain]\nchannel = \"1.85.1\"\ncomponents = [\"clippy\", \"rustfmt\"]\n"),
    );
    assert_eq!(
        tree.file_content("apps/admin/rust-toolchain.toml"),
        Some("[toolchain]\nchannel = \"stable\"\ncomponents = [\"clippy\", \"rustfmt\"]\n"),
    );

    let modern_dirs = tree.dirs_with_file("rust-toolchain.toml");
    assert!(
        modern_dirs.contains(&"apps/backend".to_owned())
            && modern_dirs.contains(&"apps/backend/crates/domain/engine".to_owned())
            && modern_dirs.contains(&"apps/admin".to_owned()),
        "ProjectTree should index every non-root rust-toolchain.toml: {modern_dirs:#?}"
    );

    let legacy_dirs = tree.dirs_with_file("rust-toolchain");
    assert!(
        legacy_dirs.contains(&"packages/ui-kit".to_owned()),
        "ProjectTree should index legacy rust-toolchain files too: {legacy_dirs:#?}"
    );
}

#[test]
#[cfg(unix)]
fn preserves_immediate_ignored_symlink_file_children_in_discovered_dirs() {
    let tmp = tempdir().expect("failed to create temporary project root");
    guardrail3_shared_fs::create_dir_all(&tmp.path().join("apps/devctl/crates/app/core"))
        .expect("failed to create project fixture directories");
    guardrail3_shared_fs::write_file(&tmp.path().join(".gitignore"), "*.env\n")
        .expect("failed to write project fixture .gitignore");
    guardrail3_shared_fs::write_file(&tmp.path().join("apps/devctl/Cargo.toml"), "[workspace]\n")
        .expect("failed to write project fixture Cargo.toml");
    std::os::unix::fs::symlink(
        tmp.path().join("apps/devctl/Cargo.toml"),
        tmp.path().join("apps/devctl/crates/app/.env"),
    )
    .expect("failed to create project fixture symlink");
    guardrail3_shared_fs::write_file(
        &tmp.path().join("apps/devctl/crates/app/core/lib.rs"),
        "pub fn ready() {}\n",
    )
    .expect("failed to write project fixture Rust source");

    let tree = walk_project(&RealFileSystem, tmp.path());
    let entry = tree
        .dir_contents("apps/devctl/crates/app")
        .expect("app container discovered");

    assert!(
        entry.files().iter().any(|file| file == ".env")
            && entry.symlink_files().iter().any(|file| file == ".env"),
        "ignored immediate symlink file should stay visible as a symlink child: {entry:#?}"
    );
}

#[test]
#[cfg(unix)]
fn preserves_immediate_broken_symlink_children_in_discovered_dirs() {
    let tmp = tempdir().expect("failed to create temporary project root");
    guardrail3_shared_fs::create_dir_all(&tmp.path().join("apps/devctl/crates/app/core"))
        .expect("failed to create project fixture directories");
    guardrail3_shared_fs::write_file(&tmp.path().join("apps/devctl/Cargo.toml"), "[workspace]\n")
        .expect("failed to write project fixture Cargo.toml");
    std::os::unix::fs::symlink(
        tmp.path().join("missing-target"),
        tmp.path().join("apps/devctl/crates/app/.env"),
    )
    .expect("failed to create project fixture symlink");
    guardrail3_shared_fs::write_file(
        &tmp.path().join("apps/devctl/crates/app/core/lib.rs"),
        "pub fn ready() {}\n",
    )
    .expect("failed to write project fixture Rust source");

    let tree = walk_project(&RealFileSystem, tmp.path());
    let entry = tree
        .dir_contents("apps/devctl/crates/app")
        .expect("app container discovered");

    assert!(
        entry.files().iter().any(|file| file == ".env")
            && entry.symlink_files().iter().any(|file| file == ".env"),
        "broken immediate symlink child should stay visible as a symlink file: {entry:#?}"
    );
}

fn rust_golden_fixture_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../tests/fixtures/full_golden")
}

fn copy_rust_golden_fixture() -> tempfile::TempDir {
    let tmp = tempdir().expect("failed to create temporary directory for golden fixture copy");
    copy_dir_recursive(&rust_golden_fixture_root(), tmp.path());
    tmp
}

fn copy_dir_recursive(src: &Path, dst: &Path) {
    for entry in std::fs::read_dir(src).expect("failed to read source fixture directory") {
        let entry = entry.expect("failed to read source fixture entry");
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if src_path.is_dir() {
            std::fs::create_dir_all(&dst_path)
                .expect("failed to create destination directory in fixture copy");
            copy_dir_recursive(&src_path, &dst_path);
        } else {
            let _ = std::fs::copy(&src_path, &dst_path)
                .expect("failed to copy file into temporary fixture");
        }
    }
}

fn write_fixture_file(root: &Path, rel_path: &str, content: &str) {
    let abs_path = root.join(rel_path);
    if let Some(parent) = abs_path.parent() {
        guardrail3_shared_fs::create_dir_all(parent)
            .expect("failed to create parent directories for fixture mutation");
    }
    guardrail3_shared_fs::write_file(&abs_path, content)
        .expect("failed to write mutated fixture file");
}

#[test]
fn recovers_tracked_ignored_files_when_git_marker_is_a_file() {
    let tmp = tempdir().expect("failed to create temporary project root");
    let root = tmp.path();

    let status = std::process::Command::new("git")
        .arg("init")
        .arg("-q")
        .current_dir(root)
        .status()
        .expect("run git init");
    assert!(status.success(), "git init should succeed");

    let status = std::process::Command::new("git")
        .args(["config", "user.email", "guardrail3@example.test"])
        .current_dir(root)
        .status()
        .expect("configure git user.email");
    assert!(status.success(), "git config user.email should succeed");

    let status = std::process::Command::new("git")
        .args(["config", "user.name", "guardrail3"])
        .current_dir(root)
        .status()
        .expect("configure git user.name");
    assert!(status.success(), "git config user.name should succeed");

    guardrail3_shared_fs::write_file(&root.join(".gitignore"), "tracked.env\n")
        .expect("write project fixture .gitignore");
    guardrail3_shared_fs::write_file(&root.join("tracked.env"), "SECRET=1\n")
        .expect("write tracked ignored fixture");

    let status = std::process::Command::new("git")
        .args(["add", ".gitignore"])
        .current_dir(root)
        .status()
        .expect("git add .gitignore");
    assert!(status.success(), "git add .gitignore should succeed");

    let status = std::process::Command::new("git")
        .args(["add", "-f", "tracked.env"])
        .current_dir(root)
        .status()
        .expect("git add tracked ignored fixture");
    assert!(status.success(), "git add should succeed");

    let status = std::process::Command::new("git")
        .args(["commit", "-qm", "fixture"])
        .current_dir(root)
        .status()
        .expect("git commit tracked fixture");
    assert!(status.success(), "git commit should succeed");

    std::fs::rename(root.join(".git"), root.join(".git-real"))
        .expect("rename .git dir to simulated worktree gitdir");
    guardrail3_shared_fs::write_file(&root.join(".git"), "gitdir: .git-real\n")
        .expect("write simulated worktree .git file");

    let tree = walk_project(&RealFileSystem, root);

    assert!(
        tree.file_exists("tracked.env"),
        "tracked ignored file should still be visible when .git is a file"
    );
}
