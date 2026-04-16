use std::path::Path;

use g3rs_workspace_crawl::G3RsWorkspaceCrawl;
use guardrail3_rs_app_types::{WorkspaceCrawlError, WorkspaceCrawler};

#[derive(Debug, Default)]
pub struct PackageRuntime;

impl WorkspaceCrawler for PackageRuntime {
    fn crawl(&self, root: &Path) -> Result<G3RsWorkspaceCrawl, WorkspaceCrawlError> {
        g3rs_workspace_crawl::crawl(root).map_err(|error| WorkspaceCrawlError {
            message: format!("{error:?}"),
        })
    }
}
