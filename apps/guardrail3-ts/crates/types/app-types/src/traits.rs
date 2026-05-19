use std::path::Path;

use g3_workspace_crawl::G3WorkspaceCrawl;

use crate::{FamilyResults, FamilyRunError, SupportedFamily, ValidateReport, WorkspaceCrawlError};

/// Crawls a workspace into the shared snapshot used by all family runners.
pub trait WorkspaceCrawler {
    /// Builds the workspace snapshot for the requested root.
    ///
    /// # Errors
    ///
    /// Returns [`WorkspaceCrawlError`] when the workspace cannot be crawled.
    fn crawl(&self, root: &Path) -> Result<G3WorkspaceCrawl, WorkspaceCrawlError>;
}

/// Runs one selected family against a prepared workspace crawl.
pub trait FamilyRunner {
    /// Executes one family and returns its findings.
    ///
    /// # Errors
    ///
    /// Returns [`FamilyRunError`] when the selected family cannot complete.
    fn run_family(
        &self,
        family: SupportedFamily,
        crawl: &G3WorkspaceCrawl,
        enabled_families: &[SupportedFamily],
    ) -> Result<FamilyResults, FamilyRunError>;
}

/// Renders the accumulated report into CLI output.
pub trait ReportRenderer {
    /// Builds the final text output for the current report.
    fn render(&self, report: &ValidateReport, include_inventory: bool) -> String;
}
