mod crate_tree;
mod dependency_edges;
mod facade_surface;
mod module_layout;

use guardrail3_app_rs_family_mapper::{RsArchRoute, RsProjectSurface as ProjectTree};

pub(crate) use self::crate_tree::{CrateNode, CrateTree};
pub(crate) use self::dependency_edges::{DependencyEdge, DependencyEdges};
pub(crate) use self::facade_surface::{FacadeSurface, FacadeSurfaceMap};
pub(crate) use self::module_layout::{ModuleDir, ModuleLayoutMap};

#[derive(Debug, Default)]
pub(crate) struct ArchFacts {
    pub crate_tree: CrateTree,
    pub dependency_edges: DependencyEdges,
    pub facade_surfaces: FacadeSurfaceMap,
    pub module_layouts: ModuleLayoutMap,
}

pub(crate) fn collect(tree: &ProjectTree, route: &RsArchRoute) -> ArchFacts {
    let crate_tree = crate_tree::collect(tree, route);
    let dependency_edges = dependency_edges::collect(tree, &crate_tree);
    let facade_surfaces = facade_surface::collect(tree, &crate_tree);
    let module_layouts = module_layout::collect(tree, &crate_tree);
    ArchFacts {
        crate_tree,
        dependency_edges,
        facade_surfaces,
        module_layouts,
    }
}
