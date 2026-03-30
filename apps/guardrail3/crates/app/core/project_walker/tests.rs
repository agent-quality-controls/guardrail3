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
