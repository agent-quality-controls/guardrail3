use super::facts::{
    CargoFamilyFacts, InputFailureFacts, MissingMemberCargoFacts, PolicyRootCargoFacts,
    WorkspaceMemberCargoFacts,
};

pub struct PolicyRootCargoInput<'a> {
    pub root: &'a PolicyRootCargoFacts,
}

pub struct WorkspaceMemberCargoInput<'a> {
    pub workspace: &'a PolicyRootCargoFacts,
    pub member: &'a WorkspaceMemberCargoFacts,
}

pub struct MissingMemberCargoInput<'a> {
    pub missing: &'a MissingMemberCargoFacts,
}

pub struct InputFailureCargoInput<'a> {
    pub failure: &'a InputFailureFacts,
}

impl<'a> PolicyRootCargoInput<'a> {
    pub const fn new(root: &'a PolicyRootCargoFacts) -> Self {
        Self { root }
    }

    pub fn from_facts(facts: &'a CargoFamilyFacts) -> Vec<Self> {
        facts.policy_roots.iter().map(Self::new).collect()
    }
}

impl<'a> WorkspaceMemberCargoInput<'a> {
    pub const fn new(
        workspace: &'a PolicyRootCargoFacts,
        member: &'a WorkspaceMemberCargoFacts,
    ) -> Self {
        Self { workspace, member }
    }

    pub fn from_facts(facts: &'a CargoFamilyFacts) -> Vec<Self> {
        facts
            .workspace_members
            .iter()
            .filter_map(|member| {
                facts
                    .policy_roots
                    .iter()
                    .find(|root| root.rel_dir == member.workspace_root_rel)
                    .map(|workspace| Self::new(workspace, member))
            })
            .collect()
    }
}

impl<'a> MissingMemberCargoInput<'a> {
    pub const fn new(missing: &'a MissingMemberCargoFacts) -> Self {
        Self { missing }
    }

    pub fn from_facts(facts: &'a CargoFamilyFacts) -> Vec<Self> {
        facts.missing_members.iter().map(Self::new).collect()
    }
}

impl<'a> InputFailureCargoInput<'a> {
    pub const fn new(failure: &'a InputFailureFacts) -> Self {
        Self { failure }
    }

    pub fn from_facts(facts: &'a CargoFamilyFacts) -> Vec<Self> {
        facts.input_failures.iter().map(Self::new).collect()
    }
}
