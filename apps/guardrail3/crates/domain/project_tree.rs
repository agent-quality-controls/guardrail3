//! The parsed project tree — full directory structure + cached config file content.
//!
//! Built once by the walker, consumed by all checkers. No checker should touch
//! the filesystem directly for config files — everything is in the tree.
//!
//! Two maps:
//! - `structure`: every directory visited → its children (dirs + file names)
//! - `content`: cached raw content of config files we check (keyed by relative path)
//!
//! Source files (.rs, .ts, .tsx) appear in the structure (we know they exist)
//! but their content is NOT cached. Source scan checks read those on demand.

use std::collections::BTreeMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// The full project tree.
///
/// Built once by [`crate::app::core::project_walker::walk_project`],
/// then passed to checkers as `&ProjectTree`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectTree {
    /// Absolute path to the project root.
    pub root: PathBuf,

    /// Directory structure: every dir visited → its immediate children.
    /// Keys are relative paths from root. `""` = the root directory itself.
    /// Sorted by path (BTreeMap).
    pub structure: BTreeMap<String, DirEntry>,

    /// Cached config file contents, keyed by relative path from root.
    /// Contains every config file we check — NOT source code (.rs/.ts/.tsx).
    /// Sorted by path (BTreeMap).
    pub content: BTreeMap<String, String>,
}

/// A single directory's immediate children.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirEntry {
    /// Child directory names (just the name, not the full path).
    pub dirs: Vec<String>,
    /// Child file names (just the name, not the full path).
    pub files: Vec<String>,
}

impl ProjectTree {
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
    /// Handles the root case (`""` + `"foo"` → `"foo"`).
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
