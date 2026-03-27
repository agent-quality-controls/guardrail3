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
    pub app_name: &'a str,
    pub app_rel_dir: &'a str,
    pub cargo_rel_path: &'a str,
    pub cargo_parse_error: Option<&'a str>,
    pub is_workspace: bool,
    pub top_level_crates_entry_count: usize,
    pub src_dir_exists: bool,
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
    pub app_name: &'a str,
    pub app_rel_dir: &'a str,
    pub crates_rel_dir: &'a str,
    pub dirs: &'a [String],
    pub files: &'a [String],
    pub symlink_dirs: &'a [String],
    pub symlink_files: &'a [String],
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
    pub app_name: &'a str,
    pub rel_path: &'a str,
    pub label: &'a str,
    pub dirs: &'a [String],
    pub symlink_dirs: &'a [String],
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
    pub app_name: &'a str,
    pub rel_path: &'a str,
    pub label: &'a str,
    pub dirs: &'a [String],
    pub symlink_dirs: &'a [String],
    pub files: &'a [String],
    pub symlink_files: &'a [String],
    pub has_gitkeep: bool,
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
    pub app_name: &'a str,
    pub rel_path: &'a str,
    pub label: &'a str,
    pub has_cargo: bool,
    pub has_crates_dir: bool,
    pub gitkeep_only: bool,
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
    pub app_name: &'a str,
    pub app_rel_dir: &'a str,
    pub cargo_parse_error: Option<&'a str>,
    pub is_workspace: bool,
    pub workspace_members: Vec<WorkspaceMemberHexarchInput<'a>>,
    pub app_local_cargo_roots: Vec<AppLocalCargoRootHexarchInput<'a>>,
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
    pub rel_dir: &'a str,
    pub cargo_rel_path: &'a str,
    pub cargo_parse_error: Option<&'a str>,
    pub is_workspace: bool,
}

impl<'a> AppLocalCargoRootHexarchInput<'a> {
    pub fn new(facts: &'a AppLocalCargoRootFact) -> Self {
        Self {
            rel_dir: &facts.rel_dir,
            cargo_rel_path: &facts.cargo_rel_path,
            cargo_parse_error: facts.cargo_parse_error.as_deref(),
            is_workspace: facts.is_workspace,
        }
    }
}

pub struct WorkspaceMemberHexarchInput<'a> {
    pub raw: &'a str,
    pub resolved_dirs: &'a [String],
    pub within_app_boundary: bool,
}

impl<'a> WorkspaceMemberHexarchInput<'a> {
    pub fn new(facts: &'a super::facts::WorkspaceMemberFact) -> Self {
        Self {
            raw: &facts.raw,
            resolved_dirs: &facts.resolved_dirs,
            within_app_boundary: facts.within_app_boundary,
        }
    }

    pub fn covers_dir(&self, rel_dir: &str) -> bool {
        self.resolved_dirs
            .iter()
            .any(|resolved| resolved == rel_dir)
    }

    pub const fn is_within_app_boundary(&self) -> bool {
        self.within_app_boundary
    }
}

pub struct RootWorkspaceHexarchInput<'a> {
    pub cargo_parse_error: Option<&'a str>,
    pub workspace_members: Vec<RootWorkspaceMemberHexarchInput<'a>>,
    pub rust_app_roots: &'a [String],
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
    pub raw: &'a str,
    pub resolved_dirs: &'a [String],
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
    pub edge: &'a DependencyEdgeFacts,
}

impl<'a> DependencyEdgeHexarchInput<'a> {
    pub fn new(edge: &'a DependencyEdgeFacts) -> Self {
        Self { edge }
    }
}

pub struct PatchHexarchInput<'a> {
    pub patch: &'a PatchEntryFacts,
}

impl<'a> PatchHexarchInput<'a> {
    pub fn new(patch: &'a PatchEntryFacts) -> Self {
        Self { patch }
    }
}

pub struct MemberConfigHexarchInput<'a> {
    pub member: &'a BoundaryConfigFacts,
}

impl<'a> MemberConfigHexarchInput<'a> {
    pub fn new(member: &'a BoundaryConfigFacts) -> Self {
        Self { member }
    }
}

pub struct MemberDependencyHexarchInput<'a> {
    pub member: &'a MemberDependencyFacts,
    pub edges: Vec<&'a DependencyEdgeFacts>,
}

impl<'a> MemberDependencyHexarchInput<'a> {
    pub fn new(member: &'a MemberDependencyFacts, edges: Vec<&'a DependencyEdgeFacts>) -> Self {
        Self { member, edges }
    }
}

pub struct CycleHexarchInput<'a> {
    pub cycle: &'a CycleFacts,
}

impl<'a> CycleHexarchInput<'a> {
    pub fn new(cycle: &'a CycleFacts) -> Self {
        Self { cycle }
    }
}

pub struct MemberManifestFailureHexarchInput<'a> {
    pub failure: &'a MemberManifestFailureFacts,
}

impl<'a> MemberManifestFailureHexarchInput<'a> {
    pub fn new(failure: &'a MemberManifestFailureFacts) -> Self {
        Self { failure }
    }
}

pub struct SourceCrateHexarchInput<'a> {
    pub source: &'a SourceCrateFacts,
}

impl<'a> SourceCrateHexarchInput<'a> {
    pub fn new(source: &'a SourceCrateFacts) -> Self {
        Self { source }
    }
}
