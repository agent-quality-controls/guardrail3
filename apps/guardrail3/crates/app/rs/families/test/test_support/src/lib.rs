use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use guardrail3_adapters_outbound_fs::RealFileSystem;
use guardrail3_app_core::project_walker::walk_project;
use guardrail3_domain_project_tree::ProjectTree;
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

pub fn walk(root: &Path) -> ProjectTree {
    walk_project(&RealFileSystem, root)
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

#[derive(Default)]
pub struct StubToolChecker {
    installed_tools: BTreeSet<String>,
}

impl StubToolChecker {
    pub fn with_tools<I, S>(tools: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        Self {
            installed_tools: tools
                .into_iter()
                .map(|tool| tool.as_ref().to_owned())
                .collect(),
        }
    }
}

impl ToolChecker for StubToolChecker {
    fn is_installed(&self, tool: &str) -> bool {
        self.installed_tools.contains(tool)
    }

    fn run_cargo_publish_dry_run_outcome(&self, _path: &Path) -> Option<CommandRunResult> {
        None
    }
}
