#![allow(
    clippy::disallowed_methods,
    reason = "CLI runtime tests create temporary Git repos and hook files"
)]

use std::path::Path;
use std::process::Command;

use g3_workspace_crawl::G3WorkspaceCrawl;
use guardrail3_rs_app_types::{
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

    fn crawl_any(&self, _root: &Path) -> Result<G3WorkspaceCrawl, WorkspaceCrawlError> {
        Err(WorkspaceCrawlError {
            message: "crawl failed".to_owned(),
        })
    }
}

#[allow(
    clippy::disallowed_methods,
    reason = "test fixture setup writes marker files needed to reach the stub crawler"
)]
fn workspace_root_with_guardrail_config() -> tempfile::TempDir {
    let root = tempfile::tempdir().expect("create temporary workspace root");
    std::fs::write(
        root.path().join("Cargo.toml"),
        "[workspace]\nmembers = []\nresolver = \"2\"\n",
    )
    .expect("write temporary Cargo.toml");
    std::fs::write(
        root.path().join("guardrail3-rs.toml"),
        "profile = \"library\"\n",
    )
    .expect("write temporary guardrail config");
    root
}

fn git_repo() -> tempfile::TempDir {
    let root = tempfile::tempdir().expect("create temporary git repo");
    let status = Command::new("git")
        .args(["init"])
        .current_dir(root.path())
        .status()
        .expect("run git init");
    assert!(status.success(), "git init should succeed");
    root
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
    let root = workspace_root_with_guardrail_config();
    let output = super::super::run_command(
        super::super::Command::Validate {
            command: super::super::ValidateCommand::Workspace {
                path: root.path().to_path_buf(),
                family: Vec::new(),
                inventory: false,
                staged: false,
                rules_only: true,
            },
        },
        &StubCrawler,
        &StubFamilyRunner,
        &StubRenderer,
    );

    guardrail3_rs_assertions::run::assert_cli_output(
        &output.stdout,
        &output.stderr,
        output.exit_code,
        "",
        "crawl failed\n",
        1,
    );
}

#[test]
fn run_command_init_repo_creates_managed_g3rs_hook_files() {
    let root = git_repo();
    let output = super::super::run_command(
        super::super::Command::Init {
            command: super::super::InitCommand::Repo {
                path: root.path().to_path_buf(),
                force: false,
            },
        },
        &StubCrawler,
        &StubFamilyRunner,
        &StubRenderer,
    );

    assert!(root.path().join(".githooks/pre-commit").is_file());
    assert!(root.path().join(".githooks/pre-commit.d/g3rs").is_file());
    let adapter = std::fs::read_to_string(root.path().join(".githooks/pre-commit"))
        .expect("read generated pre-commit adapter");
    assert!(adapter.contains(r#". ".githooks/pre-commit.d/g3rs""#));
    assert!(
        output
            .stdout
            .contains("created .githooks/pre-commit.d/g3rs"),
        "{}",
        output.stdout
    );
    assert_eq!(output.stderr, "crawl failed\n");
    assert_eq!(output.exit_code, 1);
}

#[test]
fn run_command_init_repo_refuses_project_owned_hook_without_force() {
    let root = git_repo();
    std::fs::create_dir_all(root.path().join(".githooks")).expect("create hooks dir");
    std::fs::write(
        root.path().join(".githooks/pre-commit"),
        "#!/usr/bin/env bash\necho project\n",
    )
    .expect("write project hook");

    let output = super::super::run_command(
        super::super::Command::Init {
            command: super::super::InitCommand::Repo {
                path: root.path().to_path_buf(),
                force: false,
            },
        },
        &StubCrawler,
        &StubFamilyRunner,
        &StubRenderer,
    );

    assert_eq!(output.stdout, "");
    assert!(
        output
            .stderr
            .contains("project-owned hook needs explicit managed-block insertion"),
        "{}",
        output.stderr
    );
    assert_eq!(output.exit_code, 1);
}
