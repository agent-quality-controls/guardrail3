mod crate_tree;
mod dependency_edges;
mod facade_surface;
mod module_layout;

use guardrail3_app_rs_family_mapper::RsProjectSurface as ProjectTree;

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
    pub all_rs_files: Vec<String>,
}

pub(crate) fn collect(tree: &ProjectTree) -> ArchFacts {
    let crate_tree = crate_tree::collect(tree);
    let dependency_edges = dependency_edges::collect(tree, &crate_tree);
    let facade_surfaces = facade_surface::collect(tree, &crate_tree);
    let module_layouts = module_layout::collect(tree, &crate_tree);
    let all_rs_files = collect_all_rs_files(tree);
    ArchFacts {
        crate_tree,
        dependency_edges,
        facade_surfaces,
        module_layouts,
        all_rs_files,
    }
}

fn collect_all_rs_files(tree: &ProjectTree) -> Vec<String> {
    let mut files = Vec::new();
    for dir in tree.all_dir_rels() {
        if let Some(entry) = tree.dir_contents(&dir) {
            for file in entry.files() {
                if file.ends_with(".rs") {
                    files.push(ProjectTree::join_rel(&dir, file));
                }
            }
        }
    }
    if let Some(entry) = tree.dir_contents("") {
        for file in entry.files() {
            if file.ends_with(".rs") {
                files.push(file.clone());
            }
        }
    }
    files.sort();
    files
}
