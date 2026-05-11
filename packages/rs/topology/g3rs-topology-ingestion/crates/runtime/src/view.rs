use g3_workspace_crawl::{
    G3WorkspaceCrawl, G3WorkspaceEntry, G3WorkspaceEntryKind, G3WorkspaceIgnoreState,
};

/// `CrawlView` struct.
#[derive(Debug)]
pub(crate) struct CrawlView<'a> {
    /// `crawl` item.
    crawl: &'a G3WorkspaceCrawl,
}

impl<'a> CrawlView<'a> {
    /// `new` function.
    pub(crate) const fn new(crawl: &'a G3WorkspaceCrawl) -> Self {
        Self { crawl }
    }

    /// `root_abs_path` method.
    pub(crate) fn root_abs_path(&self) -> &std::path::Path {
        self.crawl.root_abs_path.as_path()
    }

    /// `entry` method.
    pub(crate) fn entry(&self, rel_path: &str) -> Option<&G3WorkspaceEntry> {
        g3_workspace_crawl::entry(self.crawl, rel_path)
    }

    /// `included_file_entries` method.
    pub(crate) fn included_file_entries(&self) -> impl Iterator<Item = &'a G3WorkspaceEntry> {
        self.crawl.entries.iter().filter(|entry| {
            entry.kind == G3WorkspaceEntryKind::File
                && entry.ignore_state == G3WorkspaceIgnoreState::Included
        })
    }

    /// `read_file` method.
    pub(crate) fn read_file(&self, rel_path: &str) -> Result<String, std::io::Error> {
        let path = g3_workspace_crawl::entry(self.crawl, rel_path)
            .map(|entry| entry.path.abs_path.clone())
            .ok_or_else(|| {
                std::io::Error::new(std::io::ErrorKind::NotFound, "entry missing from crawl")
            })?;
        crate::fs::read_to_string(path.as_path())
    }
}
