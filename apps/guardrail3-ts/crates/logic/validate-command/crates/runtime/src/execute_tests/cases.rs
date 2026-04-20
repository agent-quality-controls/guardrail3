use std::path::Path;

use g3_workspace_crawl::G3WorkspaceCrawl;
use guardrail3_check_types::{G3CheckResult, G3Severity};
use guardrail3_ts_app_types::{
    FamilyRunError, FamilyRunner, ReportRenderer, SUPPORTED_FAMILIES, SupportedFamily,
    ValidateReport, ValidateRequest, WorkspaceCrawlError, WorkspaceCrawler,
};
use guardrail3_ts_validate_command_assertions::execute as assertions;

use crate::execute;

#[derive(Debug)]
struct StubCrawler;

impl WorkspaceCrawler for StubCrawler {
    fn crawl(&self, root: &Path) -> Result<G3WorkspaceCrawl, WorkspaceCrawlError> {
        g3_workspace_crawl::crawl(root).map_err(|error| WorkspaceCrawlError {
            message: format!("{error:?}"),
        })
    }
}

#[derive(Debug)]
struct StubFamilyRunner;

impl FamilyRunner for StubFamilyRunner {
    fn run_family(
        &self,
        family: SupportedFamily,
        _crawl: &G3WorkspaceCrawl,
    ) -> Result<Vec<G3CheckResult>, FamilyRunError> {
        let results = match family {
            SupportedFamily::Eslint => vec![
                G3CheckResult::new(
                    "TS-ESLINT-CONFIG-01".to_owned(),
                    G3Severity::Info,
                    "inventory".to_owned(),
                    "inventory".to_owned(),
                    Some("eslint.config.mjs".to_owned()),
                    None,
                )
                .into_inventory(),
                G3CheckResult::new(
                    "TS-ESLINT-CONFIG-02".to_owned(),
                    G3Severity::Warn,
                    "warn".to_owned(),
                    "warn".to_owned(),
                    Some("eslint.config.mjs".to_owned()),
                    None,
                ),
            ],
        };

        Ok(results)
    }
}

#[derive(Debug)]
struct StubRenderer;

impl ReportRenderer for StubRenderer {
    fn render(&self, report: &ValidateReport, include_inventory: bool) -> String {
        format!("runs={} inventory={include_inventory}", report.runs.len())
    }
}

#[test]
fn execute_uses_selected_families_and_hides_inventory_for_exit_code() {
    let tempdir = tempfile::tempdir().expect("create temporary workspace root");
    std::fs::write(tempdir.path().join("package.json"), "{}\n")
        .expect("write temporary workspace package.json");

    let request = ValidateRequest {
        workspace_root: tempdir.path().to_path_buf(),
        families: vec![SupportedFamily::Eslint],
        include_inventory: false,
    };

    let outcome = execute(&request, &StubCrawler, &StubFamilyRunner, &StubRenderer)
        .expect("execute should succeed for selected-family run");

    assertions::assert_execution_outcome(
        outcome.stdout(),
        outcome.stderr(),
        outcome.exit_code(),
        "runs=1 inventory=false",
        "",
        0,
    );
}

#[test]
fn execute_defaults_to_all_families_and_errors_on_non_inventory_error() {
    let tempdir = tempfile::tempdir().expect("create temporary workspace root");
    std::fs::write(tempdir.path().join("package.json"), "{}\n")
        .expect("write temporary workspace package.json");

    let request = ValidateRequest {
        workspace_root: tempdir.path().to_path_buf(),
        families: Vec::new(),
        include_inventory: false,
    };

    let outcome = execute(&request, &StubCrawler, &StubFamilyRunner, &StubRenderer)
        .expect("execute should succeed for all-family run");

    assertions::assert_execution_outcome(
        outcome.stdout(),
        outcome.stderr(),
        outcome.exit_code(),
        &format!("runs={} inventory=false", SUPPORTED_FAMILIES.len()),
        "",
        0,
    );
}

#[derive(Debug)]
struct ErroringFamilyRunner;

impl FamilyRunner for ErroringFamilyRunner {
    fn run_family(
        &self,
        family: SupportedFamily,
        _crawl: &G3WorkspaceCrawl,
    ) -> Result<Vec<G3CheckResult>, FamilyRunError> {
        match family {
            SupportedFamily::Eslint => Err(FamilyRunError {
                message: "eslint runner exploded".to_owned(),
            }),
        }
    }
}

#[test]
fn execute_keeps_successful_family_results_when_one_family_errors() {
    let tempdir = tempfile::tempdir().expect("create temporary workspace root");
    std::fs::write(tempdir.path().join("package.json"), "{}\n")
        .expect("write temporary workspace package.json");

    let request = ValidateRequest {
        workspace_root: tempdir.path().to_path_buf(),
        families: vec![SupportedFamily::Eslint],
        include_inventory: false,
    };

    let outcome = execute(&request, &StubCrawler, &ErroringFamilyRunner, &StubRenderer)
        .expect("execute should succeed when one family run fails");

    assertions::assert_execution_outcome(
        outcome.stdout(),
        outcome.stderr(),
        outcome.exit_code(),
        "runs=0 inventory=false",
        "eslint: eslint runner exploded\n",
        1,
    );
}
