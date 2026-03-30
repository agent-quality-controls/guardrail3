use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use guardrail3_domain_project_tree::{DirEntry, ProjectTree};
use guardrail3_outbound_traits::{CommandRunResult, ToolChecker};

pub fn dir_entry(dirs: &[&str], files: &[&str]) -> DirEntry {
    DirEntry::new(
        dirs.iter().map(|value| (*value).to_owned()).collect(),
        files.iter().map(|value| (*value).to_owned()).collect(),
        Vec::new(),
        Vec::new(),
    )
}

pub fn project_tree(structure: Vec<(&str, DirEntry)>, content: Vec<(&str, &str)>) -> ProjectTree {
    ProjectTree::new(
        PathBuf::from("/tmp/project"),
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

#[derive(Debug)]
pub struct StubToolChecker {
    installed: BTreeSet<String>,
}

impl StubToolChecker {
    pub fn new(installed: &[&str]) -> Self {
        Self {
            installed: installed.iter().map(|value| (*value).to_owned()).collect(),
        }
    }
}

impl ToolChecker for StubToolChecker {
    fn is_installed(&self, tool: &str) -> bool {
        self.installed.contains(tool)
    }

    fn run_cargo_publish_dry_run_outcome(&self, _path: &Path) -> Option<CommandRunResult> {
        None
    }
}
