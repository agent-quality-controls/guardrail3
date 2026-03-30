//! Project walker — builds a [`ProjectTree`] from the filesystem.
//!
//! Single walk using the `ignore` crate (respects `.gitignore`) plus a
//! `git ls-files` patch for tracked-but-gitignored files and a targeted
//! recovery pass for ignored-but-relevant files.
//!
//! The `ignore` crate skips ALL files matching `.gitignore` patterns — even
//! tracked ones. But tracked files are part of the project (they ship to other
//! developers on `git pull`). So after the initial walk, we run `git ls-files`
//! to find any tracked files the walker missed and add them back. We also
//! recover ignored files that still matter to validation, such as manifests,
//! config files, and Rust/TypeScript source files.
//!
//! Source files (.rs, .ts, .tsx) appear in the structure but their content
//! is NOT cached. Config files get their content cached.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use guardrail3_domain_project_tree::{DirEntry, ProjectTree};
#[allow(clippy::disallowed_methods)] // reason: git ls-files requires Command::new
use guardrail3_outbound_traits::FileSystem;
use walkdir::WalkDir;

type ChildSets = (
    BTreeSet<String>,
    BTreeSet<String>,
    BTreeSet<String>,
    BTreeSet<String>,
);

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

const RECOVERED_SOURCE_EXTENSIONS: &[&str] =
    &["rs", "ts", "tsx", "js", "jsx", "mts", "cts", "mjs", "cjs"];

/// Check if a file name should have its content cached.
fn should_cache(name: &str, rel_path: &str) -> bool {
    if CACHED_EXACT.contains(&name) {
        return true;
    }
    if CACHED_PREFIX.iter().any(|p| name.starts_with(p)) {
        return true;
    }
    // .cargo/config.toml and legacy .cargo/config — cargo-local env/tool overrides
    if rel_path.ends_with(".cargo/config.toml") || rel_path.ends_with(".cargo/config") {
        return true;
    }
    // .cargo/mutants.toml — special path-based match
    if name == "mutants.toml" && rel_path.contains(".cargo/") {
        return true;
    }
    // .config/nextest.toml — special path-based match
    if name == "nextest.toml" && rel_path.contains(".config/") {
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

fn should_recover_ignored(name: &str, rel_path: &str) -> bool {
    if should_cache(name, rel_path) {
        return true;
    }

    Path::new(name)
        .extension()
        .and_then(|ext| ext.to_str())
        .is_some_and(|ext| RECOVERED_SOURCE_EXTENSIONS.contains(&ext))
}

/// Build a [`ProjectTree`] by walking the filesystem from `root`.
///
/// 1. Walk with `ignore` crate (respects `.gitignore`, skips `.git/`)
/// 2. If in a git repo, run `git ls-files` to find tracked-but-gitignored files
///    and add them back — tracked files are part of the project regardless of
///    `.gitignore` patterns.
/// 3. Recover ignored-but-relevant files (manifests, config, source) so
///    validation still sees them even when they are untracked.
pub fn walk_project(fs: &dyn FileSystem, root: &Path) -> ProjectTree {
    let mut dir_children: BTreeMap<String, ChildSets> = BTreeMap::new();
    let mut content: BTreeMap<String, String> = BTreeMap::new();

    // Phase 1: Walk with ignore crate
    let walker = ignore::WalkBuilder::new(root)
        .hidden(false)
        .follow_links(true)
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
        let file_type = entry.file_type();
        let is_symlink = entry.path_is_symlink();

        if file_type.is_some_and(|ft| ft.is_dir()) {
            let _ = dir_children
                .entry(rel.clone())
                .or_insert_with(empty_child_sets);
            if let Some((parent_rel, dir_name)) = split_parent_child(&rel) {
                let parent = dir_children
                    .entry(parent_rel)
                    .or_insert_with(empty_child_sets);
                let _ = parent.0.insert(dir_name);
                if is_symlink {
                    let _ = parent
                        .2
                        .insert(path.file_name().unwrap().to_string_lossy().into_owned());
                }
            }
        } else if file_type.is_some_and(|ft| ft.is_file()) {
            if let Some((parent_rel, file_name)) = split_parent_child(&rel) {
                let parent = dir_children
                    .entry(parent_rel)
                    .or_insert_with(empty_child_sets);
                let _ = parent.1.insert(file_name);
                if is_symlink {
                    let _ = parent
                        .3
                        .insert(path.file_name().unwrap().to_string_lossy().into_owned());
                }
            }
            let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if should_cache(file_name, &rel) {
                if let Some(file_content) = fs.read_file(path) {
                    let _ = content.insert(rel, file_content);
                }
            }
        } else if is_symlink {
            if let Some((parent_rel, file_name)) = split_parent_child(&rel) {
                let parent = dir_children
                    .entry(parent_rel)
                    .or_insert_with(empty_child_sets);
                let _ = parent.1.insert(file_name.clone());
                let _ = parent.3.insert(file_name);
            }
        }
    }

    // Phase 2: Add back tracked-but-gitignored files.
    // A git worktree uses a `.git` file instead of a `.git/` directory, but
    // `git ls-files` still works and should still restore tracked ignored files.
    if root.join(".git").exists() {
        patch_tracked_files(fs, root, &mut dir_children, &mut content);
    }

    // Phase 3: Recover ignored-but-relevant files anywhere under the root.
    patch_relevant_ignored_files(fs, root, &mut dir_children, &mut content);

    // Phase 4: Preserve immediate filesystem children that the ignore walk may omit.
    // This keeps structural rules fail-closed for ignored loose files and broken
    // symlinks without broad recursive recovery of all ignored directories.
    patch_immediate_children(fs, root, &mut dir_children, &mut content);

    // Convert to DirEntry structs
    let structure = dir_children
        .into_iter()
        .map(
            |(dir_rel, (child_dirs, child_files, symlink_dirs, symlink_files))| {
                let entry = DirEntry::new(
                    child_dirs.into_iter().collect(),
                    child_files.into_iter().collect(),
                    symlink_dirs.into_iter().collect(),
                    symlink_files.into_iter().collect(),
                );
                (dir_rel, entry)
            },
        )
        .collect();

    ProjectTree::new(root.to_owned(), structure, content)
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
    dir_children: &mut BTreeMap<String, ChildSets>,
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
    for (dir_rel, (_, files, _, _)) in dir_children.iter() {
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
                .or_insert_with(empty_child_sets);
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

fn patch_relevant_ignored_files(
    fs: &dyn FileSystem,
    root: &Path,
    dir_children: &mut BTreeMap<String, ChildSets>,
    content: &mut BTreeMap<String, String>,
) {
    let mut existing_files = BTreeSet::new();
    for (dir_rel, (_, files, _, _)) in dir_children.iter() {
        for file in files {
            let _ = existing_files.insert(ProjectTree::join_rel(dir_rel, file));
        }
    }

    for entry in WalkDir::new(root)
        .follow_links(true)
        .into_iter()
        .filter_entry(|entry| entry.file_name().to_str().is_none_or(|name| name != ".git"))
        .flatten()
    {
        if !(entry.file_type().is_file() || entry.file_type().is_symlink()) {
            continue;
        }

        let Ok(rel_path) = entry.path().strip_prefix(root) else {
            continue;
        };
        let rel_path = rel_path.to_string_lossy().into_owned();
        if rel_path.is_empty() || existing_files.contains(&rel_path) {
            continue;
        }

        let name = entry.file_name().to_string_lossy().into_owned();
        if !should_recover_ignored(&name, &rel_path) {
            continue;
        }

        let Some((parent_rel, file_name)) = split_parent_child(&rel_path) else {
            continue;
        };
        ensure_parents(dir_children, &parent_rel);

        let parent = dir_children
            .entry(parent_rel)
            .or_insert_with(empty_child_sets);
        let _ = parent.1.insert(file_name.clone());
        if entry.path_is_symlink() {
            let _ = parent.3.insert(file_name);
        }

        if should_cache(&name, &rel_path) {
            if let Some(file_content) = fs.read_file(entry.path()) {
                let _ = content.insert(rel_path.clone(), file_content);
            }
        }

        let _ = existing_files.insert(rel_path);
    }
}

fn patch_immediate_children(
    fs: &dyn FileSystem,
    root: &Path,
    dir_children: &mut BTreeMap<String, ChildSets>,
    content: &mut BTreeMap<String, String>,
) {
    let discovered_dirs = dir_children.keys().cloned().collect::<Vec<_>>();

    for dir_rel in discovered_dirs {
        let abs_dir = if dir_rel.is_empty() {
            root.to_path_buf()
        } else {
            root.join(&dir_rel)
        };

        for entry in fs.list_dir(&abs_dir) {
            let Ok(file_type) = entry.file_type() else {
                continue;
            };
            let name = entry.file_name().to_string_lossy().into_owned();
            let parent = dir_children
                .entry(dir_rel.clone())
                .or_insert_with(empty_child_sets);
            if file_type.is_symlink() {
                match entry.metadata() {
                    Ok(metadata) if metadata.is_dir() => {
                        let _ = parent.0.insert(name.clone());
                        let _ = parent.2.insert(name);
                    }
                    _ => {
                        let _ = parent.1.insert(name.clone());
                        let _ = parent.3.insert(name);
                    }
                }
                continue;
            }

            if file_type.is_file() {
                let rel_path = ProjectTree::join_rel(&dir_rel, &name);
                let _ = parent.1.insert(name.clone());
                if !content.contains_key(&rel_path) && should_cache(&name, &rel_path) {
                    if let Some(file_content) = fs.read_file(&root.join(&rel_path)) {
                        let _ = content.insert(rel_path, file_content);
                    }
                }
                continue;
            }

            if file_type.is_dir() {
                let child_rel = ProjectTree::join_rel(&dir_rel, &name);
                let _ = parent.0.insert(name);
                let _ = dir_children
                    .entry(child_rel)
                    .or_insert_with(empty_child_sets);
            }
        }
    }
}

/// Ensure a directory and all its parents exist in the tree.
fn ensure_parents(dir_children: &mut BTreeMap<String, ChildSets>, rel: &str) {
    // Walk from the target dir up to root, creating entries as needed
    let mut current = rel.to_owned();
    loop {
        let _ = dir_children
            .entry(current.clone())
            .or_insert_with(empty_child_sets);

        if let Some((parent, child_name)) = split_parent_child(&current) {
            let p = dir_children
                .entry(parent.clone())
                .or_insert_with(empty_child_sets);
            let _ = p.0.insert(child_name);
            current = parent;
        } else {
            break;
        }
    }
}

fn empty_child_sets() -> ChildSets {
    (
        BTreeSet::new(),
        BTreeSet::new(),
        BTreeSet::new(),
        BTreeSet::new(),
    )
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

#[cfg(test)]
mod tests;

#[cfg(test)]
mod lossless_tests;
