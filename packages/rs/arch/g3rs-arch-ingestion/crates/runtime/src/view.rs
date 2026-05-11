use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use g3_workspace_crawl::{
    G3WorkspaceCrawl, G3WorkspaceEntry, G3WorkspaceEntryKind, G3WorkspaceIgnoreState,
};

use crate::fs;

/// Dir Contents struct.
#[derive(Debug, Clone, Default)]
pub(crate) struct DirContents {
    /// files.
    files: Vec<String>,
    /// dirs.
    dirs: Vec<String>,
}

impl DirContents {
    /// files.
    pub(crate) fn files(&self) -> &[String] {
        &self.files
    }

    /// dirs.
    pub(crate) fn dirs(&self) -> &[String] {
        &self.dirs
    }
}

/// Crawl View struct.
#[derive(Debug)]
pub(crate) struct CrawlView<'a> {
    /// crawl.
    crawl: &'a G3WorkspaceCrawl,
    /// dirs.
    dirs: BTreeMap<String, DirContents>,
}

impl<'a> CrawlView<'a> {
    /// new.
    pub(crate) fn new(crawl: &'a G3WorkspaceCrawl) -> Self {
        let mut dirs = BTreeMap::<String, DirContents>::new();
        let _ = dirs.insert(String::new(), DirContents::default());

        for entry in &crawl.entries {
            if entry.ignore_state == G3WorkspaceIgnoreState::Ignored {
                continue;
            }

            let rel_path = &entry.path.rel_path;
            let (parent, name) = rel_path.rsplit_once('/').unwrap_or(("", rel_path.as_str()));
            let parent_entry = dirs.entry(parent.to_owned()).or_default();
            insert_entry_into(parent_entry, entry.kind, name);
            if entry.kind == G3WorkspaceEntryKind::Directory {
                let _ = dirs.entry(rel_path.clone()).or_default();
            }
        }

        for contents in dirs.values_mut() {
            contents.files.sort();
            contents.dirs.sort();
        }

        Self { crawl, dirs }
    }

    /// dir contents.
    pub(crate) fn dir_contents(&self, dir: &str) -> Option<&DirContents> {
        self.dirs.get(dir)
    }

    /// file exists.
    pub(crate) fn file_exists(&self, rel_path: &str) -> bool {
        self.crawl.entries.iter().any(|entry| {
            entry.path.rel_path == rel_path
                && entry.kind == G3WorkspaceEntryKind::File
                && entry.ignore_state == G3WorkspaceIgnoreState::Included
        })
    }

    /// entry.
    pub(crate) fn entry(&self, rel_path: &str) -> Option<&G3WorkspaceEntry> {
        g3_workspace_crawl::entry(self.crawl, rel_path)
    }

    /// abs path.
    pub(crate) fn abs_path(&self, rel_path: &str) -> Option<PathBuf> {
        g3_workspace_crawl::entry(self.crawl, rel_path).map(|entry| entry.path.abs_path.clone())
    }

    /// all dir rels.
    pub(crate) fn all_dir_rels(&self) -> impl Iterator<Item = &str> {
        self.dirs.keys().map(String::as_str)
    }

    /// read file.
    pub(crate) fn read_file(&self, rel_path: &str) -> Result<String, std::io::Error> {
        let path = g3_workspace_crawl::entry(self.crawl, rel_path)
            .map(|entry| entry.path.abs_path.clone())
            .ok_or_else(|| {
                std::io::Error::new(std::io::ErrorKind::NotFound, "entry missing from crawl")
            })?;
        fs::read_to_string(&path)
    }

    /// root abs path.
    pub(crate) fn root_abs_path(&self) -> &Path {
        &self.crawl.root_abs_path
    }

    /// join rel.
    pub(crate) fn join_rel(dir: &str, child: &str) -> String {
        if dir.is_empty() {
            child.to_owned()
        } else {
            format!("{dir}/{child}")
        }
    }
}

/// Insert a single entry's name into the parent's files or dirs vec, deduping
/// against any existing entry with the same name.
fn insert_entry_into(parent_entry: &mut DirContents, kind: G3WorkspaceEntryKind, name: &str) {
    match kind {
        G3WorkspaceEntryKind::File => {
            if !parent_entry.files.iter().any(|existing| existing == name) {
                parent_entry.files.push(name.to_owned());
            }
        }
        G3WorkspaceEntryKind::Directory => {
            if !parent_entry.dirs.iter().any(|existing| existing == name) {
                parent_entry.dirs.push(name.to_owned());
            }
        }
    }
}
