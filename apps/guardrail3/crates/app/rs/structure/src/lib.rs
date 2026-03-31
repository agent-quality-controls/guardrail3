use guardrail3_app_rs_ownership::{
    RustFamilyFileFact, RustOwnedSurfaceFacts, collect as collect_owned_surface,
};
use guardrail3_app_rs_placement::{
    RustRootPlacementFacts, RustRootPlacementInputFailureFacts, RustRootPlacementRootFacts,
    RustZoneOverlapFacts, collect as collect_placement,
};
use guardrail3_domain_project_tree::ProjectTree;

#[derive(Debug, Clone, Default)]
pub struct RustStructureFacts {
    placement: RustRootPlacementFacts,
    owned_surface: RustOwnedSurfaceFacts,
}

impl RustStructureFacts {
    #[must_use]
    pub fn new(placement: RustRootPlacementFacts, owned_surface: RustOwnedSurfaceFacts) -> Self {
        Self {
            placement,
            owned_surface,
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
}

#[must_use]
pub fn collect(tree: &ProjectTree) -> RustStructureFacts {
    let placement = collect_placement(tree);
    let owned_surface = collect_owned_surface(tree, &placement);
    RustStructureFacts::new(placement, owned_surface)
}
