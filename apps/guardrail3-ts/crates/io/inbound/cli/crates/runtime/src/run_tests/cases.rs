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

    guardrail3_ts_assertions::run::assert_cli_output(
        &output.stdout,
        &output.stderr,
        output.exit_code,
        "== eslint ==\n[Error] TS-ESLINT-CONFIG-01 - eslint config missing\n  No root `eslint.config.*` file was found. Add a root flat ESLint config.\n== tsconfig ==\n[Error] TS-TSCONFIG-CONFIG-01 - tsconfig missing\n  No root `tsconfig.json` or `tsconfig.base.json` file was found. Add a root TypeScript config.\n== jscpd ==\n[Error] TS-JSCPD-CONFIG-01 - root .jscpd.json missing\n  No root `.jscpd.json` file was found. Add a root duplication-policy config.\n",
        "",
        1,
    );
}

#[test]
fn run_command_uses_real_arch_wiring_for_missing_entrypoint() {
    let tempdir = tempfile::tempdir().expect("create temporary ts workspace for arch wiring");
    std::fs::write(
        tempdir.path().join("package.json"),
        "{\n  \"exports\": {\n    \".\": \"./src/public.ts\"\n  }\n}\n",
    )
    .expect("write temporary workspace package manifest for arch wiring");

    let output = super::super::run_command_with_defaults(super::super::Command::Validate {
        path: tempdir.path().to_path_buf(),
        family: vec![super::super::super::cli::FamilyArg::Arch],
        inventory: false,
    });

    guardrail3_ts_assertions::run::assert_cli_output(
        &output.stdout,
        &output.stderr,
        output.exit_code,
        "== arch ==\n[Error] TS-ARCH-CONFIG-03 package.json declared facade entrypoint is not canonical\n  Declared facade entrypoint `src/public.ts` is not canonical. Use `src/index.ts`, `src/index.tsx`, `index.ts`, or `index.tsx`.\n[Error] TS-ARCH-FILETREE-01 package.json declared facade entrypoint missing\n  Declared facade entrypoint `src/public.ts` does not exist. Create the facade file or fix the manifest.\n",
        "",
        1,
    );
}
