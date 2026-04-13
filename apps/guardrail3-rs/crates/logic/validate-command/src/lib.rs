use guardrail3_check_types::G3Severity;
use guardrail3_rs_app_types::{
    FamilyRun, FamilyRunner, ReportRenderer, ValidateReport, ValidateRequest, WorkspaceCrawler,
};

pub fn execute(
    request: &ValidateRequest,
    crawler: &dyn WorkspaceCrawler,
    family_runner: &dyn FamilyRunner,
    renderer: &dyn ReportRenderer,
) -> Result<ExecutionOutcome, String> {
    let crawl = crawler.crawl(&request.workspace_root)?;
    let mut report = ValidateReport::default();
    let mut family_errors = Vec::new();

    for family in request.selected_families() {
        match family_runner.run_family(family, &crawl) {
            Ok(results) => report.runs.push(FamilyRun { family, results }),
            Err(error) => family_errors.push(format!("{}: {error}", family.cli_name())),
        }
    }

    let stdout = renderer.render(&report, request.include_inventory);
    let stderr = if family_errors.is_empty() {
        String::new()
    } else {
        format!("{}\n", family_errors.join("\n"))
    };
    let exit_code = match (
        report.highest_severity(request.include_inventory),
        family_errors.is_empty(),
    ) {
        (Some(G3Severity::Error), _) | (_, false) => 1,
        (Some(_), true) | (None, true) => 0,
    };

    Ok(ExecutionOutcome {
        stdout,
        stderr,
        exit_code,
    })
}

#[derive(Debug, Clone)]
pub struct ExecutionOutcome {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use g3rs_workspace_crawl::G3RsWorkspaceCrawl;
    use guardrail3_check_types::{G3CheckResult, G3Severity};
    use guardrail3_rs_app_types::{
        FamilyRunner, ReportRenderer, SupportedFamily, ValidateRequest, WorkspaceCrawler,
    };

    use super::execute;

    #[derive(Debug)]
    struct StubCrawler;

    impl WorkspaceCrawler for StubCrawler {
        fn crawl(&self, root: &Path) -> Result<G3RsWorkspaceCrawl, String> {
            g3rs_workspace_crawl::crawl(root).map_err(|error| format!("{error:?}"))
        }
    }

    #[derive(Debug)]
    struct StubFamilyRunner;

    impl FamilyRunner for StubFamilyRunner {
        fn run_family(
            &self,
            family: SupportedFamily,
            _crawl: &G3RsWorkspaceCrawl,
        ) -> Result<Vec<G3CheckResult>, String> {
            Ok(match family {
                SupportedFamily::Fmt => vec![
                    G3CheckResult::new(
                        "RS-FMT-CONFIG-01".to_owned(),
                        G3Severity::Info,
                        "inventory".to_owned(),
                        "inventory".to_owned(),
                        Some("rustfmt.toml".to_owned()),
                        None,
                    )
                    .into_inventory(),
                    G3CheckResult::new(
                        "RS-FMT-CONFIG-02".to_owned(),
                        G3Severity::Warn,
                        "warn".to_owned(),
                        "warn".to_owned(),
                        Some("rustfmt.toml".to_owned()),
                        None,
                    ),
                ],
                SupportedFamily::Deny => vec![G3CheckResult::new(
                    "RS-DENY-CONFIG-01".to_owned(),
                    G3Severity::Error,
                    "error".to_owned(),
                    "error".to_owned(),
                    Some("deny.toml".to_owned()),
                    None,
                )],
                _ => Vec::new(),
            })
        }
    }

    #[derive(Debug)]
    struct StubRenderer;

    impl ReportRenderer for StubRenderer {
        fn render(
            &self,
            report: &guardrail3_rs_app_types::ValidateReport,
            include_inventory: bool,
        ) -> String {
            format!("runs={} inventory={include_inventory}", report.runs.len())
        }
    }

    #[test]
    fn execute_uses_selected_families_and_hides_inventory_for_exit_code() {
        let tempdir = tempfile::tempdir().expect("tempdir");
        std::fs::write(
            tempdir.path().join("Cargo.toml"),
            "[workspace]\nmembers = []\n",
        )
        .expect("write Cargo.toml");

        let request = ValidateRequest {
            workspace_root: tempdir.path().to_path_buf(),
            families: vec![SupportedFamily::Fmt],
            include_inventory: false,
        };

        let outcome = execute(&request, &StubCrawler, &StubFamilyRunner, &StubRenderer)
            .expect("execute should succeed");

        assert_eq!(outcome.stdout, "runs=1 inventory=false");
        assert_eq!(outcome.stderr, "");
        assert_eq!(outcome.exit_code, 0);
    }

    #[test]
    fn execute_defaults_to_all_families_and_errors_on_non_inventory_error() {
        let tempdir = tempfile::tempdir().expect("tempdir");
        std::fs::write(
            tempdir.path().join("Cargo.toml"),
            "[workspace]\nmembers = []\n",
        )
        .expect("write Cargo.toml");

        let request = ValidateRequest {
            workspace_root: tempdir.path().to_path_buf(),
            families: Vec::new(),
            include_inventory: false,
        };

        let outcome = execute(&request, &StubCrawler, &StubFamilyRunner, &StubRenderer)
            .expect("execute should succeed");

        assert_eq!(
            outcome.stdout,
            format!(
                "runs={} inventory=false",
                guardrail3_rs_app_types::SupportedFamily::ALL.len()
            )
        );
        assert_eq!(outcome.stderr, "");
        assert_eq!(outcome.exit_code, 1);
    }

    #[derive(Debug)]
    struct ErroringFamilyRunner;

    impl FamilyRunner for ErroringFamilyRunner {
        fn run_family(
            &self,
            family: SupportedFamily,
            _crawl: &G3RsWorkspaceCrawl,
        ) -> Result<Vec<G3CheckResult>, String> {
            match family {
                SupportedFamily::Fmt => Ok(vec![G3CheckResult::new(
                    "RS-FMT-CONFIG-02".to_owned(),
                    G3Severity::Warn,
                    "warn".to_owned(),
                    "warn".to_owned(),
                    Some("rustfmt.toml".to_owned()),
                    None,
                )]),
                SupportedFamily::Deny => Err("deny runner exploded".to_owned()),
                _ => Ok(Vec::new()),
            }
        }
    }

    #[test]
    fn execute_keeps_successful_family_results_when_one_family_errors() {
        let tempdir = tempfile::tempdir().expect("tempdir");
        std::fs::write(
            tempdir.path().join("Cargo.toml"),
            "[workspace]\nmembers = []\n",
        )
        .expect("write Cargo.toml");

        let request = ValidateRequest {
            workspace_root: tempdir.path().to_path_buf(),
            families: vec![SupportedFamily::Deny, SupportedFamily::Fmt],
            include_inventory: false,
        };

        let outcome = execute(&request, &StubCrawler, &ErroringFamilyRunner, &StubRenderer)
            .expect("execute should succeed");

        assert_eq!(outcome.stdout, "runs=1 inventory=false");
        assert_eq!(outcome.stderr, "deny: deny runner exploded\n");
        assert_eq!(outcome.exit_code, 1);
    }
}
