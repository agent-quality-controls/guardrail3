use std::collections::BTreeMap;
use std::path::PathBuf;

use super::discover::collect;
use super::facts::CargoFamilyFacts;
use super::inputs::{WorkspaceCargoInput, WorkspaceMemberInput, WorkspaceMembersSetInput};
use crate::domain::project_tree::{DirEntry, ProjectTree};
use crate::domain::report::CheckResult;

pub fn entry(dirs: &[&str], files: &[&str]) -> DirEntry {
    DirEntry {
        dirs: dirs.iter().map(|value| (*value).to_owned()).collect(),
        files: files.iter().map(|value| (*value).to_owned()).collect(),
    }
}

pub fn tree(structure: &[(&str, DirEntry)], content: &[(&str, &str)]) -> ProjectTree {
    ProjectTree {
        root: PathBuf::from("/tmp/project"),
        structure: structure
            .iter()
            .map(|(path, entry)| ((*path).to_owned(), entry.clone()))
            .collect::<BTreeMap<_, _>>(),
        content: content
            .iter()
            .map(|(path, body)| ((*path).to_owned(), (*body).to_owned()))
            .collect::<BTreeMap<_, _>>(),
    }
}

pub fn collected_facts(tree: &ProjectTree) -> CargoFamilyFacts {
    collect(tree).expect("expected cargo workspace facts")
}

pub fn member_input<'a>(facts: &'a CargoFamilyFacts, member_rel: &str) -> WorkspaceMemberInput<'a> {
    let member = facts
        .members
        .iter()
        .find(|member| member.member_rel == member_rel)
        .expect("expected member facts");
    WorkspaceMemberInput::new(&facts.workspace, member)
}

pub fn workspace_input<'a>(facts: &'a CargoFamilyFacts) -> WorkspaceCargoInput<'a> {
    WorkspaceCargoInput::new(&facts.workspace)
}

pub fn members_set_input<'a>(facts: &'a CargoFamilyFacts) -> WorkspaceMembersSetInput<'a> {
    WorkspaceMembersSetInput::from_facts(facts)
}

pub fn has_result<F>(results: &[CheckResult], id: &str, predicate: F) -> bool
where
    F: Fn(&CheckResult) -> bool,
{
    results
        .iter()
        .any(|result| result.id == id && predicate(result))
}
