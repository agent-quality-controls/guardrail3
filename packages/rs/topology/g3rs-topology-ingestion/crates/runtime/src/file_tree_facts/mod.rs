use g3rs_topology_types::{
    G3RsTopologyIllegalFamilyFilePlacementInput, G3RsTopologyNestedGuardrail3RsTomlInput,
    G3RsTopologyNestedWorkspaceInput, G3RsTopologyWorkspaceMemberIssueInput,
};

/// `collect` module.
mod collect;
/// `illegal_family_files` module.
mod illegal_family_files;
/// `membership` module.
mod membership;
/// `nested` module.
mod nested;
/// `paths` module.
mod paths;

pub(crate) use collect::collect;

/// `FileTreeFacts` struct.
#[derive(Debug, Clone, Default)]
pub(crate) struct FileTreeFacts {
    /// `nested_workspaces` item.
    pub(crate) nested_workspaces: Vec<G3RsTopologyNestedWorkspaceInput>,
    /// `nested_guardrail3_rs_tomls` item.
    pub(crate) nested_guardrail3_rs_tomls: Vec<G3RsTopologyNestedGuardrail3RsTomlInput>,
    /// `membership_issues` item.
    pub(crate) membership_issues: Vec<G3RsTopologyWorkspaceMemberIssueInput>,
    /// `escaping_member_paths` item.
    pub(crate) escaping_member_paths:
        Vec<g3rs_topology_types::G3RsTopologyEscapingWorkspaceMemberPathInput>,
    /// `illegal_family_files` item.
    pub(crate) illegal_family_files: Vec<G3RsTopologyIllegalFamilyFilePlacementInput>,
}
