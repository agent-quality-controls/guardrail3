//! The parsed project tree — full directory structure + cached config file content.
//!
//! Built once by the walker, consumed by all checkers. No checker should touch
//! the filesystem directly for config files — everything is in the tree.
//!
//! Two maps:
//! - `structure`: every directory visited -> its children (dirs + file names)
//! - `content`: cached raw content of config files we check (keyed by relative path)
//!
//! Source files (.rs, .ts, .tsx) appear in the structure (we know they exist)
//! but their content is NOT cached. Source scan checks read those on demand.

use std::collections::BTreeMap;
use std::path::PathBuf;

use garde::Validate;
use glob::Pattern;
use serde::{Deserialize, Serialize};

/// The full project tree.
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ProjectTree {
    /// Absolute path to the project root.
    #[garde(skip)] // reason: walker-owned absolute path, not user-provided boundary data
    root: PathBuf,

    /// Directory structure: every dir visited -> its immediate children.
    /// Keys are relative paths from root. `""` = the root directory itself.
    /// Sorted by path (BTreeMap).
    #[garde(skip)]
    // reason: walker-owned structural map, validated by project walker construction
    structure: BTreeMap<String, DirEntry>,

    /// Cached config file contents, keyed by relative path from root.
    /// Contains every config file we check — NOT source code (.rs/.ts/.tsx).
    /// Sorted by path (BTreeMap).
    #[garde(skip)] // reason: walker-owned config cache, not direct external boundary input
    content: BTreeMap<String, String>,
}

/// A single directory's immediate children.
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct DirEntry {
    /// Child directory names (just the name, not the full path).
    #[garde(skip)] // reason: walker-owned directory listing
    dirs: Vec<String>,
    /// Child file names (just the name, not the full path).
    #[garde(skip)] // reason: walker-owned file listing
    files: Vec<String>,
    /// Child directory names that are symlinks.
    #[serde(default)]
    #[garde(skip)] // reason: walker-owned symlinked directory listing
    symlink_dirs: Vec<String>,
    /// Child file names that are symlinks or unusable symlink-like entries.
    #[serde(default)]
    #[garde(skip)] // reason: walker-owned symlinked file listing
    symlink_files: Vec<String>,
}

impl ProjectTree {
    /// Create a project tree from walker-owned data.
    #[must_use]
    pub fn new(
        root: PathBuf,
        structure: BTreeMap<String, DirEntry>,
        content: BTreeMap<String, String>,
    ) -> Self {
        Self {
            root,
            structure,
            content,
        }
    }

    /// Absolute path to the project root.
    #[must_use]
    pub fn root(&self) -> &PathBuf {
        &self.root
    }

    /// Directory structure map.
    #[must_use]
    pub fn structure(&self) -> &BTreeMap<String, DirEntry> {
        &self.structure
    }

    /// Cached config content map.
    #[must_use]
    pub fn content(&self) -> &BTreeMap<String, String> {
        &self.content
    }

    /// Check if a directory exists in the tree.
    #[must_use]
    pub fn dir_exists(&self, rel: &str) -> bool {
        self.structure.contains_key(rel)
    }

    /// Get the contents of a directory.
    #[must_use]
    pub fn dir_contents(&self, rel: &str) -> Option<&DirEntry> {
        self.structure.get(rel)
    }

    /// Get cached config file content by relative path.
    #[must_use]
    pub fn file_content(&self, rel: &str) -> Option<&str> {
        self.content.get(rel).map(String::as_str)
    }

    /// Check if a file exists in the tree by relative path.
    #[must_use]
    pub fn file_exists(&self, rel: &str) -> bool {
        let (parent, name) = split_rel(rel);
        self.dir_contents(parent)
            .is_some_and(|entry| entry.has_file(name))
    }

    /// Return all known directory relative paths except the root.
    #[must_use]
    pub fn all_dir_rels(&self) -> Vec<String> {
        self.structure
            .keys()
            .filter(|dir_rel| !dir_rel.is_empty())
            .cloned()
            .collect()
    }

    /// Return every directory that contains a child file with the given name.
    #[must_use]
    pub fn dirs_with_file(&self, name: &str) -> Vec<String> {
        self.structure
            .iter()
            .filter_map(|(dir_rel, entry)| {
                if dir_rel.is_empty() || !entry.has_file(name) {
                    None
                } else {
                    Some(dir_rel.clone())
                }
            })
            .collect()
    }

    /// Return all actual directories whose relative path matches the glob pattern.
    #[must_use]
    pub fn matching_dir_rels(&self, pattern: &str) -> Vec<String> {
        let normalized = pattern.trim_matches('/');
        let Ok(pattern) = Pattern::new(normalized) else {
            return Vec::new();
        };

        self.all_dir_rels()
            .into_iter()
            .filter(|dir_rel| pattern.matches(dir_rel))
            .collect()
    }

    /// Build an absolute path from a relative path.
    #[must_use]
    pub fn abs_path(&self, rel: &str) -> PathBuf {
        if rel.is_empty() {
            self.root.clone()
        } else {
            self.root.join(rel)
        }
    }

    /// Join a directory relative path with a child name.
    /// Handles the root case (`""` + `"foo"` -> `"foo"`).
    #[must_use]
    pub fn join_rel(parent: &str, child: &str) -> String {
        if parent.is_empty() {
            child.to_owned()
        } else {
            format!("{parent}/{child}")
        }
    }
}

impl DirEntry {
    /// Create a directory entry from walker-owned child sets.
    #[must_use]
    pub fn new(
        dirs: Vec<String>,
        files: Vec<String>,
        symlink_dirs: Vec<String>,
        symlink_files: Vec<String>,
    ) -> Self {
        Self {
            dirs,
            files,
            symlink_dirs,
            symlink_files,
        }
    }

    /// Child directory names.
    #[must_use]
    pub fn dirs(&self) -> &[String] {
        &self.dirs
    }

    /// Child file names.
    #[must_use]
    pub fn files(&self) -> &[String] {
        &self.files
    }

    /// Child directory names that are symlinks.
    #[must_use]
    pub fn symlink_dirs(&self) -> &[String] {
        &self.symlink_dirs
    }

    /// Child file names that are symlinks or unusable symlink-like entries.
    #[must_use]
    pub fn symlink_files(&self) -> &[String] {
        &self.symlink_files
    }

    /// Check if this directory has a child file with the given name.
    #[must_use]
    pub fn has_file(&self, name: &str) -> bool {
        self.files.iter().any(|f| f == name)
    }

    /// Check if this directory has a child directory with the given name.
    #[must_use]
    pub fn has_dir(&self, name: &str) -> bool {
        self.dirs.iter().any(|d| d == name)
    }
}

fn split_rel(rel: &str) -> (&str, &str) {
    match rel.rsplit_once('/') {
        Some((parent, name)) => (parent, name),
        None => ("", rel),
    }
}

#[cfg(test)]
#[path = "lib_tests/mod.rs"]
mod lib_tests;
