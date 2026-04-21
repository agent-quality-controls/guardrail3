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

#[test]
fn run_command_uses_real_eslint_wiring_for_missing_config() {
    let tempdir = tempfile::tempdir().expect("create temporary ts workspace root");
    std::fs::write(tempdir.path().join("package.json"), "{}\n")
        .expect("write temporary workspace package.json");

    let output = super::super::run_command_with_defaults(super::super::Command::Validate {
        path: tempdir.path().to_path_buf(),
        family: Vec::new(),
        inventory: false,
    });

    assert!(
        output.stdout.contains("TS-ESLINT-CONFIG-01"),
        "expected missing eslint config finding in stdout: {}",
        output.stdout
    );
    assert!(
        output.stdout.contains("eslint config missing"),
        "expected missing eslint config title in stdout: {}",
        output.stdout
    );
    assert!(
        output.stdout.contains("== eslint =="),
        "expected eslint family header in stdout: {}",
        output.stdout
    );
    assert!(
        output.stdout.contains("== tsconfig =="),
        "expected tsconfig family header in stdout: {}",
        output.stdout
    );
    assert!(
        output.stdout.contains("TS-TSCONFIG-CONFIG-01"),
        "expected missing tsconfig finding in stdout: {}",
        output.stdout
    );
    assert!(
        !output.stdout.contains("== package =="),
        "did not expect package family output for a clean local manifest: {}",
        output.stdout
    );
    assert!(
        output.stdout.contains("== jscpd =="),
        "expected jscpd family header in stdout: {}",
        output.stdout
    );
    assert!(
        output.stdout.contains("TS-JSCPD-CONFIG-01"),
        "expected missing jscpd config finding in stdout: {}",
        output.stdout
    );
    assert!(
        !output.stdout.contains("No findings."),
        "stdout should not claim the run was clean: {}",
        output.stdout
    );
    assert!(
        output.stderr.is_empty(),
        "expected no stderr for clean app wiring failure path: {}",
        output.stderr
    );
    assert_eq!(output.exit_code, 1, "expected error exit code");
}
