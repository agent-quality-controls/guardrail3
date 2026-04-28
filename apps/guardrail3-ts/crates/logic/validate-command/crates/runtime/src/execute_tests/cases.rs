use std::path::Path;

use g3_workspace_crawl::G3RsWorkspaceCrawl as G3WorkspaceCrawl;
use guardrail3_check_types::{G3CheckResult, G3Severity};
use guardrail3_ts_app_types::{
    FamilyRun, FamilyRunError, FamilyRunner, ReportRenderer, SupportedFamily, ValidateReport,
    ValidateRequest, WorkspaceCrawlError, WorkspaceCrawler,
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
                    "g3ts-eslint/exists".to_owned(),
                    G3Severity::Info,
                    "inventory".to_owned(),
                    "inventory".to_owned(),
                    Some("eslint.config.mjs".to_owned()),
                    None,
                )
                .into_inventory(),
                G3CheckResult::new(
                    "g3ts-eslint/parseable".to_owned(),
                    G3Severity::Warn,
                    "warn".to_owned(),
                    "warn".to_owned(),
                    Some("eslint.config.mjs".to_owned()),
                    None,
                ),
            ],
            SupportedFamily::AstroSetup => {
                astro_inventory("g3ts-astro-setup/astro-package-present")
            }
            SupportedFamily::AstroContent => {
                astro_inventory("g3ts-astro-content/pipeline-plugin-package-present")
            }
            SupportedFamily::AstroMdx => {
                astro_inventory("g3ts-astro-mdx/mdx-eslint-plugin-package-present")
            }
            SupportedFamily::AstroSeo => astro_inventory("g3ts-astro-seo/nuasite-checks"),
            SupportedFamily::AstroState => {
                astro_inventory("g3ts-astro-state/no-legacy-parallel-state")
            }
            SupportedFamily::Arch => vec![
                G3CheckResult::new(
                    "g3ts-arch/root-manifest-exists".to_owned(),
                    G3Severity::Info,
                    "inventory".to_owned(),
                    "inventory".to_owned(),
                    Some("package.json".to_owned()),
                    None,
                )
                .into_inventory(),
            ],
            SupportedFamily::Apparch => vec![
                G3CheckResult::new(
                    "g3ts-apparch/types-dependency-direction".to_owned(),
                    G3Severity::Info,
                    "inventory".to_owned(),
                    "inventory".to_owned(),
                    Some("src/types/model.ts".to_owned()),
                    None,
                )
                .into_inventory(),
            ],
            SupportedFamily::Tsconfig => vec![
                G3CheckResult::new(
                    "g3ts-tsconfig/exists".to_owned(),
                    G3Severity::Info,
                    "inventory".to_owned(),
                    "inventory".to_owned(),
                    Some("tsconfig.json".to_owned()),
                    None,
                )
                .into_inventory(),
            ],
            SupportedFamily::Package => vec![
                G3CheckResult::new(
                    "g3ts-package/root-exists".to_owned(),
                    G3Severity::Info,
                    "inventory".to_owned(),
                    "inventory".to_owned(),
                    Some("package.json".to_owned()),
                    None,
                )
                .into_inventory(),
            ],
            SupportedFamily::Npmrc => vec![
                G3CheckResult::new(
                    "g3ts-npmrc/root-exists".to_owned(),
                    G3Severity::Info,
                    "inventory".to_owned(),
                    "inventory".to_owned(),
                    Some(".npmrc".to_owned()),
                    None,
                )
                .into_inventory(),
            ],
            SupportedFamily::Jscpd => vec![
                G3CheckResult::new(
                    "g3ts-jscpd/root-exists".to_owned(),
                    G3Severity::Info,
                    "inventory".to_owned(),
                    "inventory".to_owned(),
                    Some(".jscpd.json".to_owned()),
                    None,
                )
                .into_inventory(),
            ],
            SupportedFamily::Hooks => vec![
                G3CheckResult::new(
                    "g3ts-hooks/pre-commit-exists".to_owned(),
                    G3Severity::Info,
                    "inventory".to_owned(),
                    "inventory".to_owned(),
                    Some(".githooks/pre-commit".to_owned()),
                    None,
                )
                .into_inventory(),
            ],
        };

        Ok(results)
    }
}

fn astro_inventory(id: &str) -> Vec<G3CheckResult> {
    vec![
        G3CheckResult::new(
            id.to_owned(),
            G3Severity::Info,
            "inventory".to_owned(),
            "inventory".to_owned(),
            Some("astro.config.mjs".to_owned()),
            None,
        )
        .into_inventory(),
    ]
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
fn execute_runs_selected_jscpd_family_only() {
    let tempdir = tempfile::tempdir().expect("create temporary workspace root");
    std::fs::write(tempdir.path().join("package.json"), "{}\n")
        .expect("write temporary workspace package.json");

    let request = ValidateRequest {
        workspace_root: tempdir.path().to_path_buf(),
        families: vec![SupportedFamily::Jscpd],
        include_inventory: false,
    };

    let outcome = execute(&request, &StubCrawler, &StubFamilyRunner, &StubRenderer)
        .expect("execute should succeed for selected jscpd-only run");

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
fn execute_runs_selected_arch_family_only() {
    let tempdir = tempfile::tempdir().expect("create temporary workspace root");
    std::fs::write(tempdir.path().join("package.json"), "{}\n")
        .expect("write temporary workspace package.json");

    let request = ValidateRequest {
        workspace_root: tempdir.path().to_path_buf(),
        families: vec![SupportedFamily::Arch],
        include_inventory: false,
    };

    let outcome = execute(&request, &StubCrawler, &StubFamilyRunner, &StubRenderer)
        .expect("execute should succeed for selected arch-only run");

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
fn execute_runs_selected_apparch_family_only() {
    let tempdir = tempfile::tempdir().expect("create temporary workspace root");
    std::fs::write(tempdir.path().join("package.json"), "{}\n")
        .expect("write temporary workspace package.json");

    let request = ValidateRequest {
        workspace_root: tempdir.path().to_path_buf(),
        families: vec![SupportedFamily::Apparch],
        include_inventory: false,
    };

    let outcome = execute(&request, &StubCrawler, &StubFamilyRunner, &StubRenderer)
        .expect("execute should succeed for selected apparch-only run");

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
fn execute_defaults_to_all_supported_families() {
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
        "runs=13 inventory=false",
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
            SupportedFamily::AstroSetup
            | SupportedFamily::AstroContent
            | SupportedFamily::AstroMdx
            | SupportedFamily::AstroSeo
            | SupportedFamily::AstroState => Err(FamilyRunError {
                message: "astro runner exploded".to_owned(),
            }),
            SupportedFamily::Arch => Err(FamilyRunError {
                message: "arch runner exploded".to_owned(),
            }),
            SupportedFamily::Apparch => Err(FamilyRunError {
                message: "apparch runner exploded".to_owned(),
            }),
            SupportedFamily::Tsconfig => Err(FamilyRunError {
                message: "tsconfig runner exploded".to_owned(),
            }),
            SupportedFamily::Package => Err(FamilyRunError {
                message: "package runner exploded".to_owned(),
            }),
            SupportedFamily::Npmrc => Err(FamilyRunError {
                message: "npmrc runner exploded".to_owned(),
            }),
            SupportedFamily::Jscpd => Err(FamilyRunError {
                message: "jscpd runner exploded".to_owned(),
            }),
            SupportedFamily::Hooks => Err(FamilyRunError {
                message: "hooks runner exploded".to_owned(),
            }),
        }
    }
}

#[test]
fn execute_does_not_print_clean_output_when_all_family_runs_fail() {
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
        "",
        "eslint: eslint runner exploded\n",
        1,
    );
}

#[test]
fn execute_does_not_print_clean_output_when_only_hidden_inventory_survives_an_error() {
    let report = ValidateReport {
        runs: vec![FamilyRun {
            family: SupportedFamily::Eslint,
            results: vec![
                G3CheckResult::new(
                    "g3ts-eslint/exists".to_owned(),
                    G3Severity::Info,
                    "inventory".to_owned(),
                    "inventory".to_owned(),
                    Some("eslint.config.mjs".to_owned()),
                    None,
                )
                .into_inventory(),
            ],
        }],
    };

    let stdout = super::super::render_stdout(&report, false, true, &StubRenderer);

    assert!(stdout.is_empty(), "stdout should stay empty: {stdout}");
}

#[test]
fn execute_keeps_visible_findings_on_stdout_when_an_error_also_happens() {
    let report = ValidateReport {
        runs: vec![FamilyRun {
            family: SupportedFamily::Eslint,
            results: vec![G3CheckResult::new(
                "g3ts-eslint/parseable".to_owned(),
                G3Severity::Warn,
                "warn".to_owned(),
                "warn".to_owned(),
                Some("eslint.config.mjs".to_owned()),
                None,
            )],
        }],
    };

    let stdout = super::super::render_stdout(&report, false, true, &StubRenderer);

    assert_eq!(stdout, "runs=1 inventory=false");
}
