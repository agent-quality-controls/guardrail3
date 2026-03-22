use std::collections::BTreeSet;

use super::facts::{CargoFamilyFacts, MemberCargoFacts, WorkspaceCargoFacts};

pub struct WorkspaceCargoInput<'a> {
    pub workspace: &'a WorkspaceCargoFacts,
}

pub struct WorkspaceMemberInput<'a> {
    pub workspace: &'a WorkspaceCargoFacts,
    pub member: &'a MemberCargoFacts,
}

pub struct WorkspaceMembersSetInput<'a> {
    pub workspace: &'a WorkspaceCargoFacts,
    pub declared_members: &'a BTreeSet<String>,
    pub discovered_members: &'a BTreeSet<String>,
}

impl<'a> WorkspaceCargoInput<'a> {
    pub const fn new(workspace: &'a WorkspaceCargoFacts) -> Self {
        Self { workspace }
    }
}

impl<'a> WorkspaceMemberInput<'a> {
    pub const fn new(workspace: &'a WorkspaceCargoFacts, member: &'a MemberCargoFacts) -> Self {
        Self { workspace, member }
    }
}

impl<'a> WorkspaceMembersSetInput<'a> {
    pub const fn from_facts(facts: &'a CargoFamilyFacts) -> Self {
        Self {
            workspace: &facts.workspace,
            declared_members: &facts.workspace.declared_members,
            discovered_members: &facts.discovered_member_rels,
        }
    }
}
