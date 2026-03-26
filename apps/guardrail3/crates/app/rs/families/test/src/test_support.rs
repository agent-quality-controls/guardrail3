#![allow(dead_code)]

use std::path::{Path, PathBuf};

use guardrail3_adapters_outbound_fs::RealFileSystem;
use guardrail3_app_core::project_walker::walk_project;
use guardrail3_domain_report::CheckResult;
use guardrail3_outbound_traits::{CommandRunResult, ToolChecker};

pub fn temp_root(slug: &str) -> PathBuf {
    let unique = format!(
        "{}-{}-{}",
        slug,
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock")
            .as_nanos()
    );
    std::env::temp_dir().join(unique)
}

pub fn tempdir() -> tempfile::TempDir {
    tempfile::tempdir().expect("create tempdir")
}

pub fn write_file(root: &Path, rel_path: &str, content: &str) {
    let abs = root.join(rel_path);
    if let Some(parent) = abs.parent() {
        std::fs::create_dir_all(parent).expect("create parent");
    }
    std::fs::write(abs, content).expect("write file");
}

pub fn run_family(root: &Path) -> Vec<CheckResult> {
    run_family_with_tool(root, true)
}

pub fn run_family_with_tool(root: &Path, cargo_mutants_installed: bool) -> Vec<CheckResult> {
    let tree = walk_project(&RealFileSystem, root);
    crate::check(
        &tree,
        &StubToolChecker::new(cargo_mutants_installed),
        None,
    )
}

pub fn rule_files(results: &[CheckResult], rule_id: &str) -> Vec<String> {
    let mut files = results
        .iter()
        .filter(|result| result.id == rule_id)
        .filter_map(|result| result.file.clone())
        .collect::<Vec<_>>();
    files.sort();
    files
}

pub fn finding<'a>(results: &'a [CheckResult], rule_id: &str) -> &'a CheckResult {
    results
        .iter()
        .find(|result| result.id == rule_id)
        .unwrap_or_else(|| panic!("expected {rule_id} finding"))
}

pub struct StubToolChecker {
    cargo_mutants_installed: bool,
}

impl StubToolChecker {
    pub const fn new(cargo_mutants_installed: bool) -> Self {
        Self {
            cargo_mutants_installed,
        }
    }
}

impl ToolChecker for StubToolChecker {
    fn is_installed(&self, tool: &str) -> bool {
        tool == "cargo-mutants" && self.cargo_mutants_installed
    }

    fn run_cargo_publish_dry_run_outcome(&self, _path: &Path) -> Option<CommandRunResult> {
        None
    }
}
