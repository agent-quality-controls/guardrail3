#![allow(
    clippy::disallowed_methods,
    reason = "fixture helpers must call std::fs and Command directly to seed test workspaces"
)]

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub(super) fn git_init(path: &Path) {
    let _status = Command::new("git")
        .args(["init", "--quiet"])
        .current_dir(path)
        .status()
        .expect("git init should succeed in test fixture setup");
}

pub(super) fn write(path: impl AsRef<Path>, content: &str) {
    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent)
            .expect("should create parent directories for test fixture files");
    }
    fs::write(path, content).expect("should write test fixture file to disk");
}

pub(super) fn crawl(root: &Path) -> g3_workspace_crawl::G3WorkspaceCrawl {
    g3_workspace_crawl::crawl_any_root(root).expect("crawl should succeed on valid test workspace")
}

pub(super) fn packages_dir() -> PathBuf {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
    manifest
        .parent()
        .and_then(Path::parent)
        .and_then(Path::parent)
        .and_then(Path::parent)
        .and_then(Path::parent)
        .expect("should resolve packages/ directory from CARGO_MANIFEST_DIR")
        .to_path_buf()
}

pub(super) fn package_dir(rel_path: &str) -> PathBuf {
    packages_dir().join(rel_path)
}

pub(super) fn collect_package_dirs(root: &Path) -> Vec<PathBuf> {
    let mut packages = Vec::new();
    let entries = std::fs::read_dir(root).expect("should be able to list package directories");

    for entry in entries {
        let entry = entry.expect("should read package directory entry");
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }

        if path.join("Cargo.toml").exists() {
            packages.push(path);
            continue;
        }

        packages.extend(collect_package_dirs(&path));
    }

    packages
}

pub(super) fn is_supported_channel(channel: &str) -> bool {
    if channel == "stable" {
        return true;
    }

    let normalized = channel.trim().trim_start_matches('v');
    let version_part = normalized
        .split_once('-')
        .map_or(normalized, |(version, _)| version);
    let mut parts = version_part.split('.');
    let Some(major) = parts.next() else {
        return false;
    };
    let Some(minor) = parts.next() else {
        return false;
    };

    if major.parse::<u64>().is_err() || minor.parse::<u64>().is_err() {
        return false;
    }

    parts
        .next()
        .is_none_or(|patch| patch.parse::<u64>().is_ok() && parts.next().is_none())
}
