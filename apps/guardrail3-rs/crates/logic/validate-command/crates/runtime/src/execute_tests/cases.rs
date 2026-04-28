use std::path::Path;

use g3rs_workspace_crawl::G3RsWorkspaceCrawl;
use guardrail3_check_types::{G3CheckResult, G3Severity};
use guardrail3_rs_app_types::{
    FamilyRunError, FamilyRunner, ReportRenderer, SUPPORTED_FAMILIES, SupportedFamily,
    ValidateReport, ValidateRequest, WorkspaceCrawlError, WorkspaceCrawler,
};
use guardrail3_rs_validate_command_assertions::execute as assertions;

use crate::execute;

use super::fs;

#[derive(Debug)]
struct StubCrawler;

impl WorkspaceCrawler for StubCrawler {
    fn crawl(&self, root: &Path) -> Result<G3RsWorkspaceCrawl, WorkspaceCrawlError> {
        g3rs_workspace_crawl::crawl(root).map_err(|error| WorkspaceCrawlError {
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
        _crawl: &G3RsWorkspaceCrawl,
    ) -> Result<Vec<G3CheckResult>, FamilyRunError> {
        Ok(match family {
            SupportedFamily::Fmt => vec![
                G3CheckResult::new(
                    "g3rs-fmt/rustfmt-required-settings".to_owned(),
                    G3Severity::Info,
                    "inventory".to_owned(),
                    "inventory".to_owned(),
                    Some("rustfmt.toml".to_owned()),
                    None,
                )
                .into_inventory(),
                G3CheckResult::new(
                    "g3rs-fmt/rustfmt-extra-settings-inventory".to_owned(),
                    G3Severity::Warn,
                    "warn".to_owned(),
                    "warn".to_owned(),
                    Some("rustfmt.toml".to_owned()),
                    None,
                ),
            ],
            SupportedFamily::Deny => vec![G3CheckResult::new(
                "g3rs-deny/deprecated-advisories".to_owned(),
                G3Severity::Error,
                "error".to_owned(),
                "error".to_owned(),
                Some("deny.toml".to_owned()),
                None,
            )],
            SupportedFamily::Topology
            | SupportedFamily::Toolchain
            | SupportedFamily::Cargo
            | SupportedFamily::Clippy
            | SupportedFamily::Code
            | SupportedFamily::Arch
            | SupportedFamily::Deps
            | SupportedFamily::Garde
            | SupportedFamily::Test
            | SupportedFamily::Release
            | SupportedFamily::Hooks
            | SupportedFamily::Apparch => Vec::new(),
        })
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
    fs::write(
        &tempdir.path().join("Cargo.toml"),
        "[workspace]\nmembers = []\n",
    );

    let request = ValidateRequest {
        workspace_root: tempdir.path().to_path_buf(),
        families: vec![SupportedFamily::Fmt],
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
    fs::write(
        &tempdir.path().join("Cargo.toml"),
        "[workspace]\nmembers = []\n",
    );

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
        1,
    );
}

#[derive(Debug)]
struct ErroringFamilyRunner;

impl FamilyRunner for ErroringFamilyRunner {
    fn run_family(
        &self,
        family: SupportedFamily,
        _crawl: &G3RsWorkspaceCrawl,
    ) -> Result<Vec<G3CheckResult>, FamilyRunError> {
        match family {
            SupportedFamily::Fmt => Ok(vec![G3CheckResult::new(
                "g3rs-fmt/rustfmt-extra-settings-inventory".to_owned(),
                G3Severity::Warn,
                "warn".to_owned(),
                "warn".to_owned(),
                Some("rustfmt.toml".to_owned()),
                None,
            )]),
            SupportedFamily::Deny => Err(FamilyRunError {
                message: "deny runner exploded".to_owned(),
            }),
            SupportedFamily::Topology
            | SupportedFamily::Toolchain
            | SupportedFamily::Cargo
            | SupportedFamily::Clippy
            | SupportedFamily::Code
            | SupportedFamily::Arch
            | SupportedFamily::Deps
            | SupportedFamily::Garde
            | SupportedFamily::Test
            | SupportedFamily::Release
            | SupportedFamily::Hooks
            | SupportedFamily::Apparch => Ok(Vec::new()),
        }
    }
}

#[test]
fn execute_keeps_successful_family_results_when_one_family_errors() {
    let tempdir = tempfile::tempdir().expect("create temporary workspace root");
    fs::write(
        &tempdir.path().join("Cargo.toml"),
        "[workspace]\nmembers = []\n",
    );

    let request = ValidateRequest {
        workspace_root: tempdir.path().to_path_buf(),
        families: vec![SupportedFamily::Deny, SupportedFamily::Fmt],
        include_inventory: false,
    };

    let outcome = execute(&request, &StubCrawler, &ErroringFamilyRunner, &StubRenderer)
        .expect("execute should succeed when one family run fails");

    assertions::assert_execution_outcome(
        outcome.stdout(),
        outcome.stderr(),
        outcome.exit_code(),
        "runs=1 inventory=false",
        "deny: deny runner exploded\n",
        1,
    );
}
