use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::path::Path;
use std::path::PathBuf;

use guardrail3_app_rs_family_view::{DirEntry, FamilyView as ProjectTree};
use guardrail3_app_rs_family_hooks_shared::hook_shell::{ParsedShellScript, parse_script};
use guardrail3_outbound_traits::{CommandRunResult, ToolChecker};

pub fn parsed_hook(content: &str) -> ParsedShellScript<'_> {
    parse_script(content)
}

pub fn hook_tree(pre_commit: &str) -> ProjectTree {
    let full_structure = BTreeMap::from([(
        ".githooks".to_owned(),
        DirEntry::new(
            Vec::new(),
            vec!["pre-commit".to_owned()],
            Vec::new(),
            Vec::new(),
        ),
    )]);
    let full_content =
        BTreeMap::from([(".githooks/pre-commit".to_owned(), pre_commit.to_owned())]);
    ProjectTree::build(
        PathBuf::from("/tmp/project"),
        &full_structure,
        &full_content,
        &["".to_owned()],
        &[],
        &[],
        None,
    )
}

#[derive(Debug)]
pub struct StubToolChecker {
    installed: BTreeSet<&'static str>,
}

impl StubToolChecker {
    pub fn new(installed: &[&'static str]) -> Self {
        Self {
            installed: installed.iter().copied().collect(),
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
