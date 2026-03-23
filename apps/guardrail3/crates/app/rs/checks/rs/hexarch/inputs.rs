use super::dependency_facts::{
    BoundaryConfigFacts, CycleFacts, DependencyEdgeFacts, MemberDependencyFacts, PatchEntryFacts,
};
use super::facts::{
    ContainerFacts, DirectionalContainerFacts, HexAppFacts, HexRootFacts, LeafFacts,
    RootWorkspaceFacts, WorkspaceCoverageFacts,
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
}

impl<'a> HexRootInput<'a> {
    pub fn new(facts: &'a HexRootFacts) -> Self {
        Self {
            app_name: &facts.app_name,
            app_rel_dir: &facts.app_rel_dir,
            crates_rel_dir: &facts.crates_rel_dir,
            dirs: &facts.dirs,
            files: &facts.files,
        }
    }
}

pub struct DirectionalContainerHexarchInput<'a> {
    pub app_name: &'a str,
    pub rel_path: &'a str,
    pub label: &'a str,
    pub dirs: &'a [String],
}

impl<'a> DirectionalContainerHexarchInput<'a> {
    pub fn new(facts: &'a DirectionalContainerFacts) -> Self {
        Self {
            app_name: &facts.app_name,
            rel_path: &facts.rel_path,
            label: &facts.label,
            dirs: &facts.dirs,
        }
    }
}

pub struct ContainerHexarchInput<'a> {
    pub app_name: &'a str,
    pub rel_path: &'a str,
    pub label: &'a str,
    pub dirs: &'a [String],
    pub files: &'a [String],
    pub has_gitkeep: bool,
}

impl<'a> ContainerHexarchInput<'a> {
    pub fn new(facts: &'a ContainerFacts) -> Self {
        Self {
            app_name: &facts.app_name,
            rel_path: &facts.rel_path,
            label: &facts.label,
            dirs: &facts.dirs,
            files: &facts.files,
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
    pub workspace_members: &'a [String],
    pub discovered_crate_dirs: &'a [String],
}

impl<'a> WorkspaceCoverageHexarchInput<'a> {
    pub fn new(facts: &'a WorkspaceCoverageFacts) -> Self {
        Self {
            app_name: &facts.app_name,
            app_rel_dir: &facts.app_rel_dir,
            cargo_parse_error: facts.cargo_parse_error.as_deref(),
            workspace_members: &facts.workspace_members,
            discovered_crate_dirs: &facts.discovered_crate_dirs,
        }
    }
}

pub struct RootWorkspaceHexarchInput<'a> {
    pub cargo_parse_error: Option<&'a str>,
    pub workspace_members: &'a [String],
    pub rust_app_roots: &'a [String],
}

impl<'a> RootWorkspaceHexarchInput<'a> {
    pub fn new(facts: &'a RootWorkspaceFacts) -> Self {
        Self {
            cargo_parse_error: facts.cargo_parse_error.as_deref(),
            workspace_members: &facts.workspace_members,
            rust_app_roots: &facts.rust_app_roots,
        }
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

pub struct SourceCrateHexarchInput<'a> {
    pub source: &'a SourceCrateFacts,
}

impl<'a> SourceCrateHexarchInput<'a> {
    pub fn new(source: &'a SourceCrateFacts) -> Self {
        Self { source }
    }
}
