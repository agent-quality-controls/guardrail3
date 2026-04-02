use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use guardrail3_app_rs_family_view::{DirEntry, FamilyView};
use guardrail3_outbound_traits::{CommandRunResult, ToolChecker};

pub fn dir_entry(dirs: &[&str], files: &[&str]) -> DirEntry {
    DirEntry::new(
        dirs.iter().map(|value| (*value).to_owned()).collect(),
        files.iter().map(|value| (*value).to_owned()).collect(),
        Vec::new(),
        Vec::new(),
    )
}

pub fn project_tree(structure: Vec<(&str, DirEntry)>, content: Vec<(&str, &str)>) -> FamilyView {
    let full_structure: BTreeMap<_, _> = structure
        .into_iter()
        .map(|(rel, entry)| (rel.to_owned(), entry))
        .collect();
    let full_content: BTreeMap<_, _> = content
        .into_iter()
        .map(|(rel, body)| (rel.to_owned(), body.to_owned()))
        .collect();
    FamilyView::build(
        PathBuf::from("/tmp/project"),
        &full_structure,
        &full_content,
        &["".to_owned()],
        &[],
        &[],
        None,
        &[],
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
