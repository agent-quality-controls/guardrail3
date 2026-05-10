#![allow(
    clippy::excessive_nesting,
    reason = "view-builder loops fold workspace entries into a BTreeSet via direct match-on-kind; inherent two-level nesting matches the entry-kind/state product and flattening would split the fold across helpers"
)]

use std::collections::BTreeSet;

use g3rs_workspace_crawl::{
    G3RsWorkspaceCrawl, G3RsWorkspaceEntry, G3RsWorkspaceEntryKind, G3RsWorkspaceIgnoreState,
};

#[derive(Debug)]
pub(crate) struct CrawlView<'a> {
    crawl: &'a G3RsWorkspaceCrawl,
    dirs: BTreeSet<String>,
}

impl<'a> CrawlView<'a> {
    pub(crate) fn new(crawl: &'a G3RsWorkspaceCrawl) -> Self {
        let mut dirs = BTreeSet::<String>::new();
        let _ = dirs.insert(String::new());

        for entry in &crawl.entries {
            if entry.ignore_state == G3RsWorkspaceIgnoreState::Ignored {
                continue;
            }
            let rel_path = &entry.path.rel_path;
            if let Some((parent, _)) = rel_path.rsplit_once('/') {
                match entry.kind {
                    G3RsWorkspaceEntryKind::File => {
                        let _ = dirs.insert(parent.to_owned());
                    }
                    G3RsWorkspaceEntryKind::Directory => {
                        let _ = dirs.insert(parent.to_owned());
                        let _ = dirs.insert(rel_path.clone());
                    }
                }
            } else if entry.kind == G3RsWorkspaceEntryKind::Directory {
                let _ = dirs.insert(rel_path.clone());
            } else {
                let _ = dirs.insert(String::new());
            }
        }

        Self { crawl, dirs }
    }

    pub(crate) fn file_exists(&self, rel_path: &str) -> bool {
        self.crawl.entries.iter().any(|entry| {
            entry.path.rel_path == rel_path
                && entry.kind == G3RsWorkspaceEntryKind::File
                && entry.ignore_state == G3RsWorkspaceIgnoreState::Included
        })
    }

    pub(crate) fn all_dir_rels(&self) -> impl Iterator<Item = &str> {
        self.dirs.iter().map(String::as_str)
    }

    pub(crate) fn entry(&self, rel_path: &str) -> Option<&G3RsWorkspaceEntry> {
        g3rs_workspace_crawl::entry(self.crawl, rel_path)
    }

    pub(crate) fn read_file(&self, rel_path: &str) -> Result<String, std::io::Error> {
        let path = g3rs_workspace_crawl::entry(self.crawl, rel_path)
            .map(|entry| entry.path.abs_path.clone())
            .ok_or_else(|| {
                std::io::Error::new(std::io::ErrorKind::NotFound, "entry missing from crawl")
            })?;
        crate::fs::read_to_string(&path)
    }

    pub(crate) fn join_rel(dir: &str, child: &str) -> String {
        if dir.is_empty() {
            child.to_owned()
        } else {
            format!("{dir}/{child}")
        }
    }
}
