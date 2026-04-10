use std::collections::BTreeMap;

use g3rs_workspace_crawl::{
    G3RsWorkspaceCrawl, G3RsWorkspaceEntry, G3RsWorkspaceEntryKind, G3RsWorkspaceIgnoreState,
};

#[derive(Debug, Clone, Default)]
pub(crate) struct DirContents {
    files: Vec<String>,
    dirs: Vec<String>,
}

impl DirContents {
    pub(crate) fn files(&self) -> &[String] {
        &self.files
    }

    pub(crate) fn dirs(&self) -> &[String] {
        &self.dirs
    }
}

#[derive(Debug)]
pub(crate) struct CrawlView<'a> {
    crawl: &'a G3RsWorkspaceCrawl,
    dirs: BTreeMap<String, DirContents>,
}

impl<'a> CrawlView<'a> {
    pub(crate) fn new(crawl: &'a G3RsWorkspaceCrawl) -> Self {
        let mut dirs = BTreeMap::<String, DirContents>::new();
        let _ = dirs.insert(String::new(), DirContents::default());

        for entry in &crawl.entries {
            if entry.ignore_state == G3RsWorkspaceIgnoreState::Ignored {
                continue;
            }
            let rel_path = &entry.path.rel_path;
            if let Some((parent, name)) = rel_path.rsplit_once('/') {
                let parent_entry = dirs.entry(parent.to_owned()).or_default();
                match entry.kind {
                    G3RsWorkspaceEntryKind::File => {
                        if !parent_entry.files.iter().any(|existing| existing == name) {
                            parent_entry.files.push(name.to_owned());
                        }
                    }
                    G3RsWorkspaceEntryKind::Directory => {
                        if !parent_entry.dirs.iter().any(|existing| existing == name) {
                            parent_entry.dirs.push(name.to_owned());
                        }
                        let _ = dirs.entry(rel_path.clone()).or_default();
                    }
                }
            } else {
                let root = dirs.entry(String::new()).or_default();
                match entry.kind {
                    G3RsWorkspaceEntryKind::File => {
                        if !root.files.iter().any(|existing| existing == rel_path) {
                            root.files.push(rel_path.clone());
                        }
                    }
                    G3RsWorkspaceEntryKind::Directory => {
                        if !root.dirs.iter().any(|existing| existing == rel_path) {
                            root.dirs.push(rel_path.clone());
                        }
                        let _ = dirs.entry(rel_path.clone()).or_default();
                    }
                }
            }
        }

        for contents in dirs.values_mut() {
            contents.files.sort();
            contents.dirs.sort();
        }

        Self { crawl, dirs }
    }

    pub(crate) fn dir_contents(&self, dir: &str) -> Option<&DirContents> {
        self.dirs.get(dir)
    }

    pub(crate) fn file_exists(&self, rel_path: &str) -> bool {
        self.crawl.entries.iter().any(|entry| {
            entry.path.rel_path == rel_path
                && entry.kind == G3RsWorkspaceEntryKind::File
                && entry.ignore_state == G3RsWorkspaceIgnoreState::Included
        })
    }

    pub(crate) fn dir_exists(&self, rel_path: &str) -> bool {
        self.crawl.entries.iter().any(|entry| {
            entry.path.rel_path == rel_path
                && entry.kind == G3RsWorkspaceEntryKind::Directory
                && entry.ignore_state == G3RsWorkspaceIgnoreState::Included
        })
    }

    pub(crate) fn all_dir_rels(&self) -> impl Iterator<Item = &str> {
        self.dirs.keys().map(String::as_str)
    }

    pub(crate) fn entry(&self, rel_path: &str) -> Option<&G3RsWorkspaceEntry> {
        self.crawl.entry(rel_path)
    }

    pub(crate) fn read_file(&self, rel_path: &str) -> Result<String, std::io::Error> {
        let path = self
            .crawl
            .entry(rel_path)
            .map(|entry| entry.path.abs_path.clone())
            .unwrap_or_else(|| self.crawl.root_abs_path.join(rel_path));
        std::fs::read_to_string(path)
    }

    pub(crate) fn join_rel(dir: &str, child: &str) -> String {
        if dir.is_empty() {
            child.to_owned()
        } else {
            format!("{dir}/{child}")
        }
    }
}
