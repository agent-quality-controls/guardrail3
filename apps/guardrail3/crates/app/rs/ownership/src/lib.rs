mod discover;
mod kinds;

pub use kinds::{
    RustFamilyFileAttachment, RustFamilyFileFact, RustFamilyFileKind, RustOwnedSurfaceFacts,
};

use guardrail3_app_rs_placement::RustRootPlacementFacts;
use guardrail3_domain_project_tree::ProjectTreeView;

#[must_use]
pub fn collect(
    tree: &dyn ProjectTreeView,
    placement: &RustRootPlacementFacts,
) -> RustOwnedSurfaceFacts {
    discover::collect(tree, placement)
}

#[cfg(test)]
#[path = "lib_tests/mod.rs"]
mod lib_tests;
