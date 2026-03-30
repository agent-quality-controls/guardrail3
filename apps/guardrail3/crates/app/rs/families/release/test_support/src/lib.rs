use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use guardrail3_adapters_outbound_fs::RealFileSystem;
use guardrail3_app_core::project_walker::walk_project;
use guardrail3_domain_project_tree::{DirEntry, ProjectTree};
use guardrail3_outbound_traits::{CommandRunResult, ToolChecker};
use guardrail3_shared_fs::{create_dir_all, write_file as write_file_at};

pub fn walk(root: &Path) -> ProjectTree {
    walk_project(&RealFileSystem, root)
}

pub fn write_file(root: &Path, rel: &str, content: &str) {
    let path = root.join(rel);
    write_file_at(&path, content).expect("failed to write release fixture file");
}

pub fn dir_entry(dirs: &[&str], files: &[&str]) -> DirEntry {
    DirEntry::new(
        dirs.iter().map(|value| (*value).to_owned()).collect(),
        files.iter().map(|value| (*value).to_owned()).collect(),
        Vec::new(),
        Vec::new(),
    )
}

pub fn project_tree(
    structure: Vec<(&str, DirEntry)>,
    content: Vec<(&str, &str)>,
    root: PathBuf,
) -> ProjectTree {
    for (rel, entry) in &structure {
        let abs_dir = if rel.is_empty() {
            root.clone()
        } else {
            root.join(rel)
        };
        create_dir_all(&abs_dir).expect("failed to create release project directory");
        for dir in entry.dirs() {
            create_dir_all(&abs_dir.join(dir)).expect("failed to create release child directory");
        }
    }
    for (rel, body) in &content {
        let abs_path = root.join(rel);
        write_file_at(&abs_path, body).expect("failed to write release project file");
    }

    ProjectTree::new(
        root,
        structure
            .into_iter()
            .map(|(rel, entry)| (rel.to_owned(), entry))
            .collect::<BTreeMap<_, _>>(),
        content
            .into_iter()
            .map(|(rel, body)| (rel.to_owned(), body.to_owned()))
            .collect::<BTreeMap<_, _>>(),
    )
}

pub fn temp_root(slug: &str) -> PathBuf {
    let unique = format!(
        "{}-{}-{}",
        slug,
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("failed to read system clock for release temp root")
            .as_nanos()
    );
    let root = std::env::temp_dir().join(unique);
    create_dir_all(&root).expect("failed to create release temp root");
    root
}

#[derive(Debug)]
pub struct StubToolChecker {
    semver_checks_installed: bool,
}

impl StubToolChecker {
    pub const fn new(semver_checks_installed: bool) -> Self {
        Self {
            semver_checks_installed,
        }
    }
}

impl ToolChecker for StubToolChecker {
    fn is_installed(&self, tool: &str) -> bool {
        tool == "cargo-semver-checks" && self.semver_checks_installed
    }

    fn run_cargo_publish_dry_run_outcome(&self, _path: &Path) -> Option<CommandRunResult> {
        None
    }
}
