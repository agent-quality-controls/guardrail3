use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

pub use guardrail3_domain_project_tree::DirEntry;

/// Scoped view of the project for a family. Contains only structure and content
/// that legality approved for this family's roots. Families cannot access
/// anything outside their scope.
///
/// This type replaces `RsProjectSurface`. It provides the same read API but:
/// - Built from LegalityFacts, not from the raw ProjectTree
/// - Scoped to route-approved roots — fixtures, illegal roots are excluded
/// - `abs_path()` validates paths are within scope before returning
#[derive(Debug, Clone)]
pub struct FamilyView {
    root: PathBuf,
    structure: BTreeMap<String, DirEntry>,
    content: BTreeMap<String, String>,
    scope_roots: Vec<String>,
}

impl FamilyView {
    /// Build a FamilyView from carried-forward structure data, scoped to
    /// the given root directories and extra files/dirs.
    ///
    /// This is called by the runtime, not by families.
    #[must_use]
    pub fn build(
        root: PathBuf,
        full_structure: &BTreeMap<String, DirEntry>,
        full_content: &BTreeMap<String, String>,
        root_rels: &[String],
        extra_file_rels: &[String],
        extra_dir_rels: &[String],
        scoped_files: Option<&BTreeSet<String>>,
    ) -> Self {
        let mut allowed_files = BTreeSet::new();
        let mut allowed_dirs = BTreeSet::new();

        // Include all files/dirs under root_rels.
        for (dir_rel, entry) in full_structure {
            if root_rels
                .iter()
                .any(|root_rel| path_is_under(dir_rel, root_rel))
            {
                let _ = allowed_dirs.insert(dir_rel.clone());
                for file_name in entry.files() {
                    let rel_path = join_rel(dir_rel, file_name);
                    let _ = allowed_files.insert(rel_path);
                }
            }
        }

        // Add extra files and dirs.
        for rel_path in extra_file_rels {
            let _ = allowed_files.insert(rel_path.clone());
        }
        for dir_rel in extra_dir_rels {
            let _ = allowed_dirs.insert(dir_rel.clone());
        }

        // Apply scoped_files filter if present.
        if let Some(scoped) = scoped_files {
            allowed_files.retain(|rel_path| scoped.contains(rel_path));
            // Re-add extra files (not subject to scoping).
            for rel_path in extra_file_rels {
                let _ = allowed_files.insert(rel_path.clone());
            }
        }

        // Ensure parent directories of allowed files are included.
        for rel_path in &allowed_files.clone() {
            let mut cursor = split_parent(rel_path);
            loop {
                let _ = allowed_dirs.insert(cursor.to_owned());
                if cursor.is_empty() {
                    break;
                }
                cursor = split_parent(cursor);
            }
        }

        // Build filtered structure map.
        let mut structure = BTreeMap::new();
        for dir_rel in &allowed_dirs {
            if let Some(entry) = full_structure.get(dir_rel.as_str()) {
                let filtered_dirs = entry
                    .dirs()
                    .iter()
                    .filter(|child| {
                        let rel = join_rel(dir_rel, child);
                        allowed_dirs.contains(&rel)
                    })
                    .cloned()
                    .collect::<Vec<_>>();
                let filtered_files = entry
                    .files()
                    .iter()
                    .filter(|child| {
                        let rel = join_rel(dir_rel, child);
                        allowed_files.contains(&rel)
                    })
                    .cloned()
                    .collect::<Vec<_>>();
                let filtered_symlink_dirs = entry
                    .symlink_dirs()
                    .iter()
                    .filter(|child| {
                        let rel = join_rel(dir_rel, child);
                        allowed_dirs.contains(&rel)
                    })
                    .cloned()
                    .collect::<Vec<_>>();
                let filtered_symlink_files = entry
                    .symlink_files()
                    .iter()
                    .filter(|child| {
                        let rel = join_rel(dir_rel, child);
                        allowed_files.contains(&rel)
                    })
                    .cloned()
                    .collect::<Vec<_>>();
                let _ = structure.insert(
                    dir_rel.clone(),
                    DirEntry::new(
                        filtered_dirs,
                        filtered_files,
                        filtered_symlink_dirs,
                        filtered_symlink_files,
                    ),
                );
            }
        }

        // Build filtered content map.
        let content = full_content
            .iter()
            .filter(|(rel, _)| allowed_files.contains(rel.as_str()))
            .map(|(rel, value)| (rel.clone(), value.clone()))
            .collect();

        Self {
            root,
            structure,
            content,
            scope_roots: root_rels.to_vec(),
        }
    }

    /// Get cached config file content by relative path.
    #[must_use]
    pub fn file_content(&self, rel: &str) -> Option<&str> {
        self.content.get(rel).map(String::as_str)
    }

    /// Check if a file exists in the scoped structure.
    #[must_use]
    pub fn file_exists(&self, rel: &str) -> bool {
        let (parent, name) = split_rel(rel);
        self.dir_contents(parent)
            .is_some_and(|entry| entry.has_file(name))
    }

    /// Check if a directory exists in the scoped structure.
    #[must_use]
    pub fn dir_exists(&self, rel: &str) -> bool {
        self.structure.contains_key(rel)
    }

    /// Get a directory's children in the scoped structure.
    #[must_use]
    pub fn dir_contents(&self, rel: &str) -> Option<&DirEntry> {
        self.structure.get(rel)
    }

    /// Get absolute path for a relative path — ONLY within scope roots.
    /// Returns None for out-of-scope paths or paths containing `..`.
    /// Prevents filesystem bypass and path traversal.
    #[must_use]
    pub fn abs_path(&self, rel: &str) -> Option<PathBuf> {
        // Reject path traversal.
        if rel.split('/').any(|seg| seg == "..") {
            return None;
        }
        if self.is_in_scope(rel) {
            Some(self.root.join(rel))
        } else {
            None
        }
    }

    /// The full scoped structure map.
    #[must_use]
    pub fn structure(&self) -> &BTreeMap<String, DirEntry> {
        &self.structure
    }

    /// The full scoped content map.
    #[must_use]
    pub fn content(&self) -> &BTreeMap<String, String> {
        &self.content
    }

    /// Glob-match directories in the scoped structure.
    #[must_use]
    pub fn matching_dir_rels(&self, pattern: &str) -> Vec<String> {
        let normalized = pattern.trim_matches('/');
        let Ok(pat) = glob::Pattern::new(normalized) else {
            return Vec::new();
        };
        self.structure
            .keys()
            .filter(|dir_rel| !dir_rel.is_empty() && pat.matches(dir_rel))
            .cloned()
            .collect()
    }

    /// Join parent directory with child name.
    #[must_use]
    pub fn join_rel(parent: &str, child: &str) -> String {
        join_rel(parent, child)
    }

    fn is_in_scope(&self, rel: &str) -> bool {
        // Empty rel is root — in scope if any scope root is empty (project-wide).
        if rel.is_empty() {
            return self.scope_roots.iter().any(|r| r.is_empty());
        }
        // ONLY check scope roots. Do not fall back to structure/content maps.
        // The scope roots are the source of truth for what's in scope.
        self.scope_roots
            .iter()
            .any(|root| path_is_under(rel, root))
    }
}

fn path_is_under(rel_path: &str, parent_rel: &str) -> bool {
    parent_rel.is_empty()
        || rel_path == parent_rel
        || rel_path
            .strip_prefix(parent_rel)
            .is_some_and(|rest| rest.starts_with('/'))
}

fn split_parent(rel: &str) -> &str {
    rel.rsplit_once('/').map_or("", |(parent, _)| parent)
}

fn split_rel(rel: &str) -> (&str, &str) {
    rel.rsplit_once('/').unwrap_or(("", rel))
}

fn join_rel(parent: &str, child: &str) -> String {
    if parent.is_empty() {
        child.to_owned()
    } else {
        format!("{parent}/{child}")
    }
}
