use std::path::Path;

use g3rs_workspace_crawl::{G3RsWorkspaceCrawl, crawl};

mod basic;
mod pipeline;

pub(super) fn crawl_workspace(root: &Path) -> G3RsWorkspaceCrawl {
    crawl(root).expect("crawl should succeed on valid test workspace")
}

pub(super) fn write(path: impl AsRef<Path>, content: &str) {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("create parent dirs");
    }
    std::fs::write(path, content).expect("write fixture");
}
