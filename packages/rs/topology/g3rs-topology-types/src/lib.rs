#[cfg(feature = "api")]
mod types;

#[cfg(feature = "api")]
pub use types::{
    G3RsTopologyCargoManifestKind, G3RsTopologyDescendantCargoRoot,
    G3RsTopologyEscapingWorkspaceMemberPathInput, G3RsTopologyFileTreeChecksInput,
    G3RsTopologyFileTreeInputFailure, G3RsTopologyIllegalFamilyFilePlacementInput,
    G3RsTopologyNestedWorkspaceInput, G3RsTopologyWorkspaceFamily, G3RsTopologyWorkspaceFamilyFile,
    G3RsTopologyWorkspaceFamilyFileAttachment, G3RsTopologyWorkspaceFamilyFileKind,
    G3RsTopologyWorkspaceMemberIssueInput, G3RsTopologyWorkspaceMemberIssueKind,
};
