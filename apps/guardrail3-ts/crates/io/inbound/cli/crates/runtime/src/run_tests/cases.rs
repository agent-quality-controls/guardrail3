use std::path::Path;

use g3_workspace_crawl::G3WorkspaceCrawl;
use guardrail3_ts_app_types::{
    FamilyResults, FamilyRunError, FamilyRunner, ReportRenderer, SupportedFamily, ValidateReport,
    WorkspaceCrawlError, WorkspaceCrawler,
};
#[derive(Debug)]
struct StubCrawler;

impl WorkspaceCrawler for StubCrawler {
    fn crawl(&self, _root: &Path) -> Result<G3WorkspaceCrawl, WorkspaceCrawlError> {
        Err(WorkspaceCrawlError {
            message: "crawl failed".to_owned(),
        })
    }
}

#[derive(Debug)]
struct StubFamilyRunner;

impl FamilyRunner for StubFamilyRunner {
    fn run_family(
        &self,
        _family: SupportedFamily,
        _crawl: &G3WorkspaceCrawl,
    ) -> Result<FamilyResults, FamilyRunError> {
        Ok(FamilyResults::new())
    }
}

#[derive(Debug)]
struct StubRenderer;

impl ReportRenderer for StubRenderer {
    fn render(&self, _report: &ValidateReport, _include_inventory: bool) -> String {
        "rendered\n".to_owned()
    }
}

#[test]
fn run_command_sends_failures_to_stderr() {
    let output = super::super::run_command(
        super::super::Command::Validate {
            path: ".".into(),
            family: Vec::new(),
            inventory: false,
        },
        &StubCrawler,
        &StubFamilyRunner,
        &StubRenderer,
    );

    guardrail3_ts_assertions::run::assert_cli_output(
        &output.stdout,
        &output.stderr,
        output.exit_code,
        "",
        "crawl failed\n",
        1,
    );
}
