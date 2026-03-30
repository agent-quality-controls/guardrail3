use super::facts::{
    CargoFamilyFacts, InputFailureFacts, MissingMemberCargoFacts, PolicyRootCargoFacts,
    WorkspaceMemberCargoFacts,
};

pub struct PolicyRootCargoInput<'a> {
    pub(crate) root: &'a PolicyRootCargoFacts,
}

pub struct WorkspaceMemberCargoInput<'a> {
    pub(crate) workspace: &'a PolicyRootCargoFacts,
    pub(crate) member: &'a WorkspaceMemberCargoFacts,
}

pub struct MissingMemberCargoInput<'a> {
    pub(crate) missing: &'a MissingMemberCargoFacts,
}

pub struct MissingMemberInventoryCargoInput<'a> {
    pub(crate) workspace: &'a PolicyRootCargoFacts,
    pub(crate) has_missing_members: bool,
}

pub struct InputFailureCargoInput<'a> {
    pub(crate) failure: &'a InputFailureFacts,
}

pub struct InputFailureInventoryCargoInput<'a> {
    pub(crate) root: &'a PolicyRootCargoFacts,
    pub(crate) has_input_failures: bool,
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

impl<'a> MissingMemberInventoryCargoInput<'a> {
    pub const fn new(workspace: &'a PolicyRootCargoFacts, has_missing_members: bool) -> Self {
        Self {
            workspace,
            has_missing_members,
        }
    }

    pub fn from_facts(facts: &'a CargoFamilyFacts) -> Vec<Self> {
        facts
            .policy_roots
            .iter()
            .filter(|root| root.kind == super::facts::PolicyRootKind::WorkspaceRoot)
            .map(|workspace| {
                let has_missing_members = facts
                    .missing_members
                    .iter()
                    .any(|missing| missing.workspace_root_rel == workspace.rel_dir);
                Self::new(workspace, has_missing_members)
            })
            .collect()
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

impl<'a> InputFailureInventoryCargoInput<'a> {
    pub const fn new(root: &'a PolicyRootCargoFacts, has_input_failures: bool) -> Self {
        Self {
            root,
            has_input_failures,
        }
    }

    pub fn from_facts(facts: &'a CargoFamilyFacts) -> Vec<Self> {
        facts
            .policy_roots
            .iter()
            .map(|root| {
                let guardrail_rel_path = if root.rel_dir.is_empty() {
                    "guardrail3.toml".to_owned()
                } else {
                    format!("{}/guardrail3.toml", root.rel_dir)
                };
                let has_member_input_failures = facts.workspace_members.iter().any(|member| {
                    member.workspace_root_rel == root.rel_dir && member.parse_error.is_some()
                });
                let has_input_failures = has_member_input_failures
                    || facts.input_failures.iter().any(|failure| {
                        failure.rel_path == root.cargo_rel_path
                            || failure.rel_path == guardrail_rel_path
                    });
                Self::new(root, has_input_failures)
            })
            .collect()
    }
}
