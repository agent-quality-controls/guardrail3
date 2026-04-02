use guardrail3_app_rs_ownership::{
    RustFamilyFileFact, RustOwnedSurfaceFacts, collect as collect_owned_surface,
};
use guardrail3_app_rs_placement::{
    RustRootPlacementFacts, RustRootPlacementInputFailureFacts, RustRootPlacementRootFacts,
    RustZoneOverlapFacts, collect as collect_placement,
};
use std::collections::BTreeMap;
use std::path::PathBuf;

use guardrail3_domain_project_tree::{DirEntry, ProjectTree};

#[derive(Debug, Clone, Default)]
pub struct RustStructureFacts {
    placement: RustRootPlacementFacts,
    owned_surface: RustOwnedSurfaceFacts,
    /// Carried forward from ProjectTree — filtered content for classified roots.
    content: BTreeMap<String, String>,
    /// Carried forward from ProjectTree — project root path.
    root: PathBuf,
    /// Carried forward from ProjectTree — directory structure (for legality glob expansion).
    structure: BTreeMap<String, DirEntry>,
}

impl RustStructureFacts {
    #[must_use]
    pub fn new(
        placement: RustRootPlacementFacts,
        owned_surface: RustOwnedSurfaceFacts,
        content: BTreeMap<String, String>,
        root: PathBuf,
        structure: BTreeMap<String, DirEntry>,
    ) -> Self {
        Self {
            placement,
            owned_surface,
            content,
            root,
            structure,
        }
    }

    #[must_use]
    pub fn placement(&self) -> &RustRootPlacementFacts {
        &self.placement
    }

    #[must_use]
    pub fn owned_surface(&self) -> &RustOwnedSurfaceFacts {
        &self.owned_surface
    }

    #[must_use]
    pub fn roots(&self) -> &[RustRootPlacementRootFacts] {
        self.placement.roots()
    }

    #[must_use]
    pub fn overlaps(&self) -> &[RustZoneOverlapFacts] {
        self.placement.overlaps()
    }

    #[must_use]
    pub fn input_failures(&self) -> &[RustRootPlacementInputFailureFacts] {
        self.placement.input_failures()
    }

    #[must_use]
    pub fn family_files(&self) -> &[RustFamilyFileFact] {
        self.owned_surface.family_files()
    }

    #[must_use]
    pub fn content(&self) -> &BTreeMap<String, String> {
        &self.content
    }

    #[must_use]
    pub fn file_content(&self, rel: &str) -> Option<&str> {
        self.content.get(rel).map(String::as_str)
    }

    #[must_use]
    pub fn root_path(&self) -> &PathBuf {
        &self.root
    }

    #[must_use]
    pub fn dir_structure(&self) -> &BTreeMap<String, DirEntry> {
        &self.structure
    }

    /// Glob-match directories in the carried-forward structure.
    #[must_use]
    pub fn matching_dir_rels(&self, pattern: &str) -> Vec<String> {
        let normalized = pattern.trim_matches('/');
        let Ok(pat) = glob::Pattern::new(normalized) else {
            return Vec::new();
        };
        self.structure
            .keys()
            .filter(|dir_rel| !dir_rel.is_empty() && pat.matches(dir_rel))
            .cloned()
            .collect()
    }
}

#[must_use]
pub fn collect(tree: ProjectTree) -> RustStructureFacts {
    let placement = collect_placement(&tree);
    let owned_surface = collect_owned_surface(&tree, &placement);
    let content = tree.content().clone();
    let root = tree.root().clone();
    let structure = tree.structure().clone();
    // tree is consumed here — no further access possible
    RustStructureFacts::new(placement, owned_surface, content, root, structure)
}
