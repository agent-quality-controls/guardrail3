use std::path::Path;

use g3_workspace_crawl::G3RsWorkspaceCrawl as G3WorkspaceCrawl;
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
fn run_command_normalizes_relative_validate_path_before_crawling() {
    let tempdir = tempfile::tempdir().expect("create temporary ts workspace root");
    std::fs::write(tempdir.path().join("package.json"), "{}\n")
        .expect("write temporary workspace package.json");
    let parent = tempdir
        .path()
        .parent()
        .expect("temporary workspace should have a parent")
        .to_path_buf();
    let name = tempdir
        .path()
        .file_name()
        .expect("temporary workspace should have a directory name")
        .to_owned();
    let original_cwd = std::env::current_dir().expect("current directory should be readable");
    std::env::set_current_dir(parent).expect("test should enter temporary parent");

    let output = super::super::run_command_with_defaults(super::super::Command::Validate {
        path: std::path::PathBuf::from(name),
        family: Vec::new(),
        inventory: false,
    });

    std::env::set_current_dir(original_cwd).expect("test should restore original cwd");

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

#[test]
fn run_command_uses_real_apparch_wiring_for_forbidden_types_dependency() {
    let tempdir = tempfile::tempdir().expect("create temporary ts workspace for apparch wiring");
    std::fs::write(tempdir.path().join("package.json"), "{}\n")
        .expect("write temporary workspace package.json for apparch wiring");
    std::fs::create_dir_all(tempdir.path().join("src/types"))
        .expect("create apparch types fixture directory");
    std::fs::create_dir_all(tempdir.path().join("src/logic"))
        .expect("create apparch logic fixture directory");
    std::fs::write(
        tempdir.path().join("src/logic/format_user.ts"),
        "export function formatUser(): string { return \"user\"; }\n",
    )
    .expect("write apparch logic fixture file");
    std::fs::write(
        tempdir.path().join("src/types/user.ts"),
        "import { formatUser } from \"@/logic/format_user\";\nexport interface User { formatted: ReturnType<typeof formatUser>; }\n",
    )
    .expect("write apparch types fixture file");

    let output = super::super::run_command_with_defaults(super::super::Command::Validate {
        path: tempdir.path().to_path_buf(),
        family: vec![super::super::super::cli::FamilyArg::Apparch],
        inventory: false,
    });

    guardrail3_ts_assertions::run::assert_cli_output(
        &output.stdout,
        &output.stderr,
        output.exit_code,
        "== apparch ==\n[Error] TS-APPARCH-CONFIG-01 src/types/user.ts types layer imports forbidden app layer\n  `src/types/user.ts` in `types` imports `src/logic/format_user.ts` in `logic`. Keep `types` passive and move behavior or framework coupling outward.\n",
        "",
        1,
    );
}

#[test]
fn run_command_uses_structure_runner_for_astro_family() {
    let tempdir = tempfile::tempdir().expect("create temporary ts workspace for astro wiring");
    std::fs::write(
        tempdir.path().join("package.json"),
        "{\n  \"dependencies\": {\n    \"astro\": \"5.0.0\"\n  }\n}\n",
    )
    .expect("write temporary workspace package.json for astro wiring");
    std::fs::create_dir_all(tempdir.path().join("src"))
        .expect("create temporary Astro src directory");
    std::fs::write(tempdir.path().join("src/content.config.ts"), "")
        .expect("write temporary Astro content config");
    std::fs::create_dir_all(tempdir.path().join(".next/server/app"))
        .expect("create temporary forbidden generated state directory");
    std::fs::write(tempdir.path().join(".next/server/app/page.js"), "")
        .expect("write temporary forbidden generated state file");

    let output = super::super::run_command_with_defaults(super::super::Command::Validate {
        path: tempdir.path().to_path_buf(),
        family: vec![super::super::super::cli::FamilyArg::Astro],
        inventory: false,
    });

    assert!(
        output.stdout.contains("== astro =="),
        "expected astro findings on stdout, got: {:?}",
        output
    );
    let mut last_index = 0;
    for prefix in [
        "TS-ASTRO-SETUP-CONFIG-",
        "TS-ASTRO-SETUP-FILETREE-",
        "TS-ASTRO-CONTENT-CONFIG-",
        "TS-ASTRO-MDX-CONFIG-",
        "TS-ASTRO-SEO-CONFIG-",
        "TS-ASTRO-STATE-FILETREE-",
    ] {
        let relative_index = output.stdout[last_index..]
            .find(prefix)
            .unwrap_or_else(|| panic!("expected `{prefix}` after byte {last_index}: {output:?}"));
        last_index += relative_index;
    }
}
