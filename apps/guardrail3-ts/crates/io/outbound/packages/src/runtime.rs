use std::path::Path;

use g3_workspace_crawl::G3RsWorkspaceCrawl as G3WorkspaceCrawl;
use guardrail3_ts_app_types::{WorkspaceCrawlError, WorkspaceCrawler};

#[derive(Debug, Default)]
pub struct PackageRuntime;

impl WorkspaceCrawler for PackageRuntime {
    fn crawl(&self, root: &Path) -> Result<G3WorkspaceCrawl, WorkspaceCrawlError> {
        g3_workspace_crawl::crawl(root).map_err(|error| WorkspaceCrawlError {
            message: format!("{error:?}"),
        })
    }
}
