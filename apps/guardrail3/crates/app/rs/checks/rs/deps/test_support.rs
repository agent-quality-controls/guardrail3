use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use super::facts::{
    AllowlistCoverageFacts, DependencyEntryFacts, DependencySectionKind, DepsFacts,
    InputFailureFacts, LockfileFacts, ToolFacts, collect,
};
use super::inputs::{
    AllowlistCoverageDepsInput, DependencyEntryDepsInput, InputFailureDepsInput, LockfileDepsInput,
    ToolDepsInput,
};
use crate::domain::project_tree::{DirEntry, ProjectTree};
use crate::domain::report::CheckResult;
use crate::ports::outbound::ToolChecker;

pub fn dir_entry(dirs: &[&str], files: &[&str]) -> DirEntry {
    DirEntry {
        dirs: dirs.iter().map(|value| (*value).to_owned()).collect(),
        files: files.iter().map(|value| (*value).to_owned()).collect(),
    }
}

pub fn project_tree(structure: Vec<(&str, DirEntry)>, content: Vec<(&str, &str)>) -> ProjectTree {
    ProjectTree {
        root: PathBuf::from("/tmp/project"),
        structure: structure
            .into_iter()
            .map(|(rel, entry)| (rel.to_owned(), entry))
            .collect::<BTreeMap<_, _>>(),
        content: content
            .into_iter()
            .map(|(rel, body)| (rel.to_owned(), body.to_owned()))
            .collect::<BTreeMap<_, _>>(),
    }
}

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

    fn run_cargo_publish_dry_run(&self, _path: &Path) -> Option<String> {
        None
    }
}

pub fn collected_facts(tree: &ProjectTree, installed: &[&str]) -> DepsFacts {
    collect(tree, &StubToolChecker::new(installed))
}

pub fn tool_input<'a>(facts: &'a DepsFacts, tool_name: &str) -> ToolDepsInput<'a> {
    let tool = facts
        .tools
        .iter()
        .find(|tool| tool.tool_name == tool_name)
        .expect("expected tool facts");
    ToolDepsInput::new(tool)
}

pub fn dependency_input<'a>(
    facts: &'a DepsFacts,
    cargo_rel_path: &str,
    section_kind: DependencySectionKind,
    dep_package_name: &str,
) -> DependencyEntryDepsInput<'a> {
    let entry = facts
        .dependency_entries
        .iter()
        .find(|entry| {
            entry.cargo_rel_path == cargo_rel_path
                && entry.section_kind == section_kind
                && entry.dep_package_name == dep_package_name
        })
        .expect("expected dependency entry facts");
    DependencyEntryDepsInput::new(entry)
}

pub fn coverage_input<'a>(
    facts: &'a DepsFacts,
    cargo_rel_path: &str,
) -> AllowlistCoverageDepsInput<'a> {
    let coverage = facts
        .allowlist_coverage
        .iter()
        .find(|coverage| coverage.cargo_rel_path == cargo_rel_path)
        .expect("expected allowlist coverage facts");
    AllowlistCoverageDepsInput::new(coverage)
}

pub fn lockfile_input<'a>(facts: &'a DepsFacts) -> LockfileDepsInput<'a> {
    LockfileDepsInput::new(facts.lockfiles.first().expect("expected lockfile facts"))
}

pub fn failure_input<'a>(facts: &'a DepsFacts, rel_path: &str) -> InputFailureDepsInput<'a> {
    let failure = facts
        .input_failures
        .iter()
        .find(|failure| failure.rel_path == rel_path)
        .expect("expected input failure facts");
    InputFailureDepsInput::new(failure)
}

pub fn has_result<F>(results: &[CheckResult], id: &str, predicate: F) -> bool
where
    F: Fn(&CheckResult) -> bool,
{
    results
        .iter()
        .any(|result| result.id == id && predicate(result))
}

pub fn tool_facts(tool_name: &str, installed: bool) -> DepsFacts {
    DepsFacts {
        tools: vec![ToolFacts {
            tool_name: tool_name.to_owned(),
            installed,
        }],
        lockfiles: vec![LockfileFacts {
            root_rel_dir: String::new(),
            cargo_lock_rel_path: "Cargo.lock".to_owned(),
            cargo_lock_exists: true,
            cargo_lock_ignored: false,
            gitignore_rel_path: Some(".gitignore".to_owned()),
            profile_name: Some("service".to_owned()),
        }],
        dependency_entries: Vec::new(),
        allowlist_coverage: Vec::new(),
        input_failures: Vec::new(),
    }
}

pub fn dependency_facts(
    section_kind: DependencySectionKind,
    allowlist_present: bool,
    allowlisted: bool,
    dep_package_name: &str,
) -> DepsFacts {
    DepsFacts {
        tools: Vec::new(),
        lockfiles: vec![LockfileFacts {
            root_rel_dir: String::new(),
            cargo_lock_rel_path: "Cargo.lock".to_owned(),
            cargo_lock_exists: true,
            cargo_lock_ignored: false,
            gitignore_rel_path: Some(".gitignore".to_owned()),
            profile_name: Some("service".to_owned()),
        }],
        dependency_entries: vec![DependencyEntryFacts {
            crate_name: "api".to_owned(),
            cargo_rel_path: "crates/api/Cargo.toml".to_owned(),
            section_kind,
            dep_alias: dep_package_name.to_owned(),
            dep_package_name: dep_package_name.to_owned(),
            allowlist_present,
            allowlisted,
        }],
        allowlist_coverage: Vec::new(),
        input_failures: Vec::new(),
    }
}

pub fn coverage_facts(profile_name: Option<&str>, has_allowlist: bool) -> DepsFacts {
    DepsFacts {
        tools: Vec::new(),
        lockfiles: vec![LockfileFacts {
            root_rel_dir: String::new(),
            cargo_lock_rel_path: "Cargo.lock".to_owned(),
            cargo_lock_exists: true,
            cargo_lock_ignored: false,
            gitignore_rel_path: Some(".gitignore".to_owned()),
            profile_name: Some("service".to_owned()),
        }],
        dependency_entries: Vec::new(),
        allowlist_coverage: vec![AllowlistCoverageFacts {
            crate_name: "core".to_owned(),
            cargo_rel_path: "packages/core/Cargo.toml".to_owned(),
            profile_name: profile_name.map(str::to_owned),
            has_allowlist,
        }],
        input_failures: Vec::new(),
    }
}

pub fn lockfile_facts(
    cargo_lock_exists: bool,
    cargo_lock_ignored: bool,
    root_profile_name: Option<&str>,
) -> DepsFacts {
    DepsFacts {
        tools: Vec::new(),
        lockfiles: vec![LockfileFacts {
            root_rel_dir: String::new(),
            cargo_lock_rel_path: "Cargo.lock".to_owned(),
            cargo_lock_exists,
            cargo_lock_ignored,
            gitignore_rel_path: Some(".gitignore".to_owned()),
            profile_name: root_profile_name.map(str::to_owned),
        }],
        dependency_entries: Vec::new(),
        allowlist_coverage: Vec::new(),
        input_failures: Vec::new(),
    }
}

pub fn failure_facts(rel_path: &str, message: &str) -> DepsFacts {
    DepsFacts {
        tools: Vec::new(),
        lockfiles: vec![LockfileFacts {
            root_rel_dir: String::new(),
            cargo_lock_rel_path: "Cargo.lock".to_owned(),
            cargo_lock_exists: true,
            cargo_lock_ignored: false,
            gitignore_rel_path: Some(".gitignore".to_owned()),
            profile_name: Some("service".to_owned()),
        }],
        dependency_entries: Vec::new(),
        allowlist_coverage: Vec::new(),
        input_failures: vec![InputFailureFacts {
            rel_path: rel_path.to_owned(),
            message: message.to_owned(),
        }],
    }
}
