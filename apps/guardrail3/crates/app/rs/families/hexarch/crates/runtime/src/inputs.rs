use super::dependency_facts::{
    BoundaryConfigFacts, CycleFacts, DependencyEdgeFacts, MemberDependencyFacts,
    MemberManifestFailureFacts, PatchEntryFacts,
};
use super::facts::{
    AppLocalCargoRootFact, ContainerFacts, DirectionalContainerFacts, HexAppFacts, HexRootFacts,
    LeafFacts, RootWorkspaceFacts, WorkspaceCoverageFacts,
};
use super::source_facts::SourceCrateFacts;

pub struct AppHexarchInput<'a> {
    pub(crate) app_name: &'a str,
    pub(crate) app_rel_dir: &'a str,
    pub(crate) cargo_rel_path: &'a str,
    pub(crate) cargo_parse_error: Option<&'a str>,
    pub(crate) is_workspace: bool,
    pub(crate) top_level_crates_entry_count: usize,
    pub(crate) src_dir_exists: bool,
}

impl<'a> AppHexarchInput<'a> {
    pub fn new(facts: &'a HexAppFacts) -> Self {
        Self {
            app_name: &facts.app_name,
            app_rel_dir: &facts.app_rel_dir,
            cargo_rel_path: &facts.cargo_rel_path,
            cargo_parse_error: facts.cargo_parse_error.as_deref(),
            is_workspace: facts.is_workspace,
            top_level_crates_entry_count: facts.top_level_crates_entry_count,
            src_dir_exists: facts.src_dir_exists,
        }
    }
}

pub struct HexRootInput<'a> {
    pub(crate) app_name: &'a str,
    pub(crate) app_rel_dir: &'a str,
    pub(crate) crates_rel_dir: &'a str,
    pub(crate) dirs: &'a [String],
    pub(crate) files: &'a [String],
    pub(crate) symlink_dirs: &'a [String],
    pub(crate) symlink_files: &'a [String],
}

impl<'a> HexRootInput<'a> {
    pub fn new(facts: &'a HexRootFacts) -> Self {
        Self {
            app_name: &facts.app_name,
            app_rel_dir: &facts.app_rel_dir,
            crates_rel_dir: &facts.crates_rel_dir,
            dirs: &facts.dirs,
            files: &facts.files,
            symlink_dirs: &facts.symlink_dirs,
            symlink_files: &facts.symlink_files,
        }
    }
}

pub struct DirectionalContainerHexarchInput<'a> {
    pub(crate) app_name: &'a str,
    pub(crate) rel_path: &'a str,
    pub(crate) label: &'a str,
    pub(crate) dirs: &'a [String],
    pub(crate) symlink_dirs: &'a [String],
}

impl<'a> DirectionalContainerHexarchInput<'a> {
    pub fn new(facts: &'a DirectionalContainerFacts) -> Self {
        Self {
            app_name: &facts.app_name,
            rel_path: &facts.rel_path,
            label: &facts.label,
            dirs: &facts.dirs,
            symlink_dirs: &facts.symlink_dirs,
        }
    }
}

pub struct ContainerHexarchInput<'a> {
    pub(crate) app_name: &'a str,
    pub(crate) rel_path: &'a str,
    pub(crate) label: &'a str,
    pub(crate) dirs: &'a [String],
    pub(crate) symlink_dirs: &'a [String],
    pub(crate) files: &'a [String],
    pub(crate) symlink_files: &'a [String],
    pub(crate) has_gitkeep: bool,
}

impl<'a> ContainerHexarchInput<'a> {
    pub fn new(facts: &'a ContainerFacts) -> Self {
        Self {
            app_name: &facts.app_name,
            rel_path: &facts.rel_path,
            label: &facts.label,
            dirs: &facts.dirs,
            symlink_dirs: &facts.symlink_dirs,
            files: &facts.files,
            symlink_files: &facts.symlink_files,
            has_gitkeep: facts.has_gitkeep,
        }
    }
}

pub struct LeafHexarchInput<'a> {
    pub(crate) app_name: &'a str,
    pub(crate) rel_path: &'a str,
    pub(crate) label: &'a str,
    pub(crate) has_cargo: bool,
    pub(crate) has_crates_dir: bool,
    pub(crate) gitkeep_only: bool,
}

impl<'a> LeafHexarchInput<'a> {
    pub fn new(facts: &'a LeafFacts) -> Self {
        Self {
            app_name: &facts.app_name,
            rel_path: &facts.rel_path,
            label: &facts.label,
            has_cargo: facts.has_cargo,
            has_crates_dir: facts.has_crates_dir,
            gitkeep_only: facts.gitkeep_only,
        }
    }
}

pub struct WorkspaceCoverageHexarchInput<'a> {
    pub(crate) app_name: &'a str,
    pub(crate) app_rel_dir: &'a str,
    pub(crate) cargo_parse_error: Option<&'a str>,
    pub(crate) is_workspace: bool,
    pub(crate) workspace_members: Vec<WorkspaceMemberHexarchInput<'a>>,
    pub(crate) app_local_cargo_roots: Vec<AppLocalCargoRootHexarchInput<'a>>,
}

impl<'a> WorkspaceCoverageHexarchInput<'a> {
    pub fn new(facts: &'a WorkspaceCoverageFacts) -> Self {
        Self {
            app_name: &facts.app_name,
            app_rel_dir: &facts.app_rel_dir,
            cargo_parse_error: facts.cargo_parse_error.as_deref(),
            is_workspace: facts.is_workspace,
            workspace_members: facts
                .workspace_members
                .iter()
                .map(WorkspaceMemberHexarchInput::new)
                .collect(),
            app_local_cargo_roots: facts
                .app_local_cargo_roots
                .iter()
                .map(AppLocalCargoRootHexarchInput::new)
                .collect(),
        }
    }
}

pub struct AppLocalCargoRootHexarchInput<'a> {
    pub(crate) rel_dir: &'a str,
    pub(crate) cargo_rel_path: &'a str,
    pub(crate) is_workspace: bool,
}

impl<'a> AppLocalCargoRootHexarchInput<'a> {
    pub fn new(facts: &'a AppLocalCargoRootFact) -> Self {
        Self {
            rel_dir: &facts.rel_dir,
            cargo_rel_path: &facts.cargo_rel_path,
            is_workspace: facts.is_workspace,
        }
    }
}

pub struct WorkspaceMemberHexarchInput<'a> {
    pub(crate) raw: &'a str,
    pub(crate) within_app_boundary: bool,
}

impl<'a> WorkspaceMemberHexarchInput<'a> {
    pub fn new(facts: &'a super::facts::WorkspaceMemberFact) -> Self {
        Self {
            raw: &facts.raw,
            within_app_boundary: facts.within_app_boundary,
        }
    }

    pub const fn is_within_app_boundary(&self) -> bool {
        self.within_app_boundary
    }
}

pub struct RootWorkspaceHexarchInput<'a> {
    pub(crate) cargo_parse_error: Option<&'a str>,
    pub(crate) workspace_members: Vec<RootWorkspaceMemberHexarchInput<'a>>,
    pub(crate) rust_app_roots: &'a [String],
}

impl<'a> RootWorkspaceHexarchInput<'a> {
    pub fn new(facts: &'a RootWorkspaceFacts) -> Self {
        Self {
            cargo_parse_error: facts.cargo_parse_error.as_deref(),
            workspace_members: facts
                .workspace_members
                .iter()
                .map(RootWorkspaceMemberHexarchInput::new)
                .collect(),
            rust_app_roots: &facts.rust_app_roots,
        }
    }
}

pub struct RootWorkspaceMemberHexarchInput<'a> {
    pub(crate) raw: &'a str,
    pub(crate) resolved_dirs: &'a [String],
}

impl<'a> RootWorkspaceMemberHexarchInput<'a> {
    pub fn new(facts: &'a super::facts::RootWorkspaceMemberFact) -> Self {
        Self {
            raw: &facts.raw,
            resolved_dirs: &facts.resolved_dirs,
        }
    }

    pub fn covers_dir(&self, rel_dir: &str) -> bool {
        self.resolved_dirs
            .iter()
            .any(|resolved| resolved == rel_dir || resolved.starts_with(&format!("{rel_dir}/")))
    }
}

pub struct DependencyEdgeHexarchInput<'a> {
    pub(crate) edge: &'a DependencyEdgeFacts,
}

impl<'a> DependencyEdgeHexarchInput<'a> {
    pub fn new(edge: &'a DependencyEdgeFacts) -> Self {
        Self { edge }
    }
}

pub struct PatchHexarchInput<'a> {
    pub(crate) patch: &'a PatchEntryFacts,
}

impl<'a> PatchHexarchInput<'a> {
    pub fn new(patch: &'a PatchEntryFacts) -> Self {
        Self { patch }
    }
}

pub(crate) struct MemberConfigHexarchInput<'a> {
    pub member: &'a BoundaryConfigFacts,
}

impl<'a> MemberConfigHexarchInput<'a> {
    pub fn new(member: &'a BoundaryConfigFacts) -> Self {
        Self { member }
    }
}

pub(crate) struct MemberDependencyHexarchInput<'a> {
    pub member: &'a MemberDependencyFacts,
    pub edges: Vec<&'a DependencyEdgeFacts>,
}

impl<'a> MemberDependencyHexarchInput<'a> {
    pub fn new(member: &'a MemberDependencyFacts, edges: Vec<&'a DependencyEdgeFacts>) -> Self {
        Self { member, edges }
    }
}

pub(crate) struct CycleHexarchInput<'a> {
    pub cycle: &'a CycleFacts,
}

impl<'a> CycleHexarchInput<'a> {
    pub fn new(cycle: &'a CycleFacts) -> Self {
        Self { cycle }
    }
}

pub(crate) struct MemberManifestFailureHexarchInput<'a> {
    pub failure: &'a MemberManifestFailureFacts,
}

impl<'a> MemberManifestFailureHexarchInput<'a> {
    pub fn new(failure: &'a MemberManifestFailureFacts) -> Self {
        Self { failure }
    }
}

pub(crate) struct SourceCrateHexarchInput<'a> {
    pub source: &'a SourceCrateFacts,
}

impl<'a> SourceCrateHexarchInput<'a> {
    pub fn new(source: &'a SourceCrateFacts) -> Self {
        Self { source }
    }
}
