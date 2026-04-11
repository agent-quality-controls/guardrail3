use g3rs_workspace_crawl::{
    G3RsWorkspaceCrawl, G3RsWorkspaceEntry, G3RsWorkspaceEntryKind, G3RsWorkspaceIgnoreState,
};

#[derive(Debug)]
pub(crate) struct CrawlView<'a> {
    crawl: &'a G3RsWorkspaceCrawl,
}

impl<'a> CrawlView<'a> {
    pub(crate) fn new(crawl: &'a G3RsWorkspaceCrawl) -> Self {
        Self { crawl }
    }

    pub(crate) fn root_abs_path(&self) -> &std::path::Path {
        self.crawl.root_abs_path.as_path()
    }

    pub(crate) fn entry(&self, rel_path: &str) -> Option<&G3RsWorkspaceEntry> {
        self.crawl.entry(rel_path)
    }

    pub(crate) fn included_file_entries(&self) -> impl Iterator<Item = &'a G3RsWorkspaceEntry> {
        self.crawl.entries.iter().filter(|entry| {
            entry.kind == G3RsWorkspaceEntryKind::File
                && entry.ignore_state == G3RsWorkspaceIgnoreState::Included
        })
    }

    pub(crate) fn read_file(&self, rel_path: &str) -> Result<String, std::io::Error> {
        let path = self
            .crawl
            .entry(rel_path)
            .map(|entry| entry.path.abs_path.clone())
            .ok_or_else(|| {
                std::io::Error::new(std::io::ErrorKind::NotFound, "entry missing from crawl")
            })?;
        std::fs::read_to_string(path)
    }
}
