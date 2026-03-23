//! Project walker — builds a [`ProjectTree`] from the filesystem.
//!
//! Single walk using the `ignore` crate (respects `.gitignore`) plus a
//! `git ls-files` patch for tracked-but-gitignored files.
//!
//! The `ignore` crate skips ALL files matching `.gitignore` patterns — even
//! tracked ones. But tracked files are part of the project (they ship to other
//! developers on `git pull`). So after the initial walk, we run `git ls-files`
//! to find any tracked files the walker missed and add them back.
//!
//! Source files (.rs, .ts, .tsx) appear in the structure but their content
//! is NOT cached. Config files get their content cached.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use crate::domain::project_tree::{DirEntry, ProjectTree};
#[allow(clippy::disallowed_methods)] // reason: git ls-files requires Command::new
use crate::ports::outbound::FileSystem;

/// Config file names that get their content cached (exact match).
const CACHED_EXACT: &[&str] = &[
    "Cargo.toml",
    "Cargo.lock",
    ".gitignore",
    "clippy.toml",
    ".clippy.toml",
    "deny.toml",
    ".deny.toml",
    "rustfmt.toml",
    ".rustfmt.toml",
    "rust-toolchain.toml",
    "package.json",
    "pnpm-workspace.yaml",
    "tsconfig.json",
    "tsconfig.base.json",
    ".npmrc",
    ".jscpd.json",
    "cspell.json",
    ".cspell.json",
    "guardrail3.toml",
    "release-plz.toml",
    ".release-plz.toml",
    "cliff.toml",
    "CLAUDE.md",
    "LICENSE",
    "LICENSE-MIT",
    "LICENSE-APACHE",
    "LICENSE.md",
    ".gitkeep",
    "stryker.config.json",
];

/// Config file prefixes that get their content cached.
/// A file matches if its name starts with one of these.
const CACHED_PREFIX: &[&str] = &[
    "eslint.config.",
    ".stylelintrc",
    "stylelint.config.",
    "cspell.config.",
    ".cspell.config.",
    "prettier.config.",
    ".prettierrc",
    "velite.config.",
    "next.config.",
    "stryker.config.",
    "vitest.config.",
    "jest.config.",
];

/// Check if a file name should have its content cached.
fn should_cache(name: &str, rel_path: &str) -> bool {
    if CACHED_EXACT.contains(&name) {
        return true;
    }
    if CACHED_PREFIX.iter().any(|p| name.starts_with(p)) {
        return true;
    }
    // .cargo/mutants.toml — special path-based match
    if name == "mutants.toml" && rel_path.contains(".cargo/") {
        return true;
    }
    // GitHub workflow YAML files
    if rel_path.contains(".github/workflows/") {
        let ext = Path::new(name)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");
        if ext.eq_ignore_ascii_case("yml") || ext.eq_ignore_ascii_case("yaml") {
            return true;
        }
    }
    // Pre-commit hooks
    if name == "pre-commit" && (rel_path.contains(".githooks/") || rel_path.contains("/hooks/")) {
        return true;
    }
    false
}

/// Build a [`ProjectTree`] by walking the filesystem from `root`.
///
/// 1. Walk with `ignore` crate (respects `.gitignore`, skips `.git/`)
/// 2. If in a git repo, run `git ls-files` to find tracked-but-gitignored files
///    and add them back — tracked files are part of the project regardless of
///    `.gitignore` patterns.
pub fn walk_project(fs: &dyn FileSystem, root: &Path) -> ProjectTree {
    let mut dir_children: BTreeMap<String, (BTreeSet<String>, BTreeSet<String>)> = BTreeMap::new();
    let mut content: BTreeMap<String, String> = BTreeMap::new();

    // Phase 1: Walk with ignore crate
    let walker = ignore::WalkBuilder::new(root)
        .hidden(false)
        .git_ignore(true)
        .git_global(true)
        .git_exclude(true)
        .filter_entry(|entry| {
            // Skip .git/ — the ignore crate doesn't auto-skip it with hidden(false)
            if entry.file_type().is_some_and(|ft| ft.is_dir()) {
                let name = entry.file_name().to_string_lossy();
                if name == ".git" {
                    return false;
                }
            }
            true
        })
        .build();

    for entry in walker.flatten() {
        let path = entry.path();
        let rel = match path.strip_prefix(root) {
            Ok(r) => r.to_string_lossy().into_owned(),
            Err(_) => continue,
        };
        let Some(ft) = entry.file_type() else {
            continue;
        };

        if ft.is_dir() {
            let _ = dir_children
                .entry(rel.clone())
                .or_insert_with(|| (BTreeSet::new(), BTreeSet::new()));
            if let Some((parent_rel, dir_name)) = split_parent_child(&rel) {
                let parent = dir_children
                    .entry(parent_rel)
                    .or_insert_with(|| (BTreeSet::new(), BTreeSet::new()));
                let _ = parent.0.insert(dir_name);
            }
        } else if ft.is_file() {
            if let Some((parent_rel, file_name)) = split_parent_child(&rel) {
                let parent = dir_children
                    .entry(parent_rel)
                    .or_insert_with(|| (BTreeSet::new(), BTreeSet::new()));
                let _ = parent.1.insert(file_name);
            }
            let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if should_cache(file_name, &rel) {
                if let Some(file_content) = fs.read_file(path) {
                    let _ = content.insert(rel, file_content);
                }
            }
        }
    }

    // Phase 2: Add back tracked-but-gitignored files
    if root.join(".git").is_dir() {
        patch_tracked_files(fs, root, &mut dir_children, &mut content);
    }

    // Convert to DirEntry structs
    let structure = dir_children
        .into_iter()
        .map(|(dir_rel, (child_dirs, child_files))| {
            let entry = DirEntry {
                dirs: child_dirs.into_iter().collect(),
                files: child_files.into_iter().collect(),
            };
            (dir_rel, entry)
        })
        .collect();

    ProjectTree {
        root: root.to_owned(),
        structure,
        content,
    }
}

/// Find tracked files that the `ignore` crate skipped (because they match
/// `.gitignore` patterns) and add them to the tree.
///
/// Runs `git ls-files` to get all tracked files, checks which are missing
/// from the tree, and adds those back — including their parent directories
/// and cached content if applicable.
fn patch_tracked_files(
    fs: &dyn FileSystem,
    root: &Path,
    dir_children: &mut BTreeMap<String, (BTreeSet<String>, BTreeSet<String>)>,
    content: &mut BTreeMap<String, String>,
) {
    let output = std::process::Command::new("git")
        .args(["ls-files"])
        .current_dir(root)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .output();

    let Ok(output) = output else {
        return; // git not available — skip silently
    };
    if !output.status.success() {
        return;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Collect all files already in the tree for fast lookup
    let mut existing_files: BTreeSet<String> = BTreeSet::new();
    for (dir_rel, (_, files)) in dir_children.iter() {
        for f in files {
            let _ = existing_files.insert(ProjectTree::join_rel(dir_rel, f));
        }
    }

    for line in stdout.lines() {
        if line.is_empty() {
            continue;
        }
        // Skip files already in the tree
        if existing_files.contains(line) {
            continue;
        }
        // Skip files that don't exist on disk (deleted but still tracked)
        let abs = root.join(line);
        if !abs.exists() {
            continue;
        }

        // Add the file to the tree
        if let Some((parent_rel, file_name)) = split_parent_child(line) {
            // Ensure all parent dirs exist in the tree
            ensure_parents(dir_children, &parent_rel);

            // Add file to parent's children
            let parent = dir_children
                .entry(parent_rel)
                .or_insert_with(|| (BTreeSet::new(), BTreeSet::new()));
            let _ = parent.1.insert(file_name);

            // Cache content if it's a config file
            let name = abs.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if should_cache(name, line) {
                if let Some(file_content) = fs.read_file(&abs) {
                    let _ = content.insert(line.to_owned(), file_content);
                }
            }
        }
    }
}

/// Ensure a directory and all its parents exist in the tree.
fn ensure_parents(
    dir_children: &mut BTreeMap<String, (BTreeSet<String>, BTreeSet<String>)>,
    rel: &str,
) {
    // Walk from the target dir up to root, creating entries as needed
    let mut current = rel.to_owned();
    loop {
        let _ = dir_children
            .entry(current.clone())
            .or_insert_with(|| (BTreeSet::new(), BTreeSet::new()));

        if let Some((parent, child_name)) = split_parent_child(&current) {
            let p = dir_children
                .entry(parent.clone())
                .or_insert_with(|| (BTreeSet::new(), BTreeSet::new()));
            let _ = p.0.insert(child_name);
            current = parent;
        } else {
            break;
        }
    }
}

/// Split a relative path into (parent_rel, child_name).
/// `"apps/devctl/crates"` → `("apps/devctl", "crates")`
/// `"apps"` → `("", "apps")`
/// `""` → None (root has no parent)
fn split_parent_child(rel: &str) -> Option<(String, String)> {
    if rel.is_empty() {
        return None;
    }
    match rel.rfind('/') {
        Some(idx) => Some((
            rel[..idx].to_owned(),
            rel[idx.saturating_add(1)..].to_owned(),
        )),
        None => Some((String::new(), rel.to_owned())),
    }
}
