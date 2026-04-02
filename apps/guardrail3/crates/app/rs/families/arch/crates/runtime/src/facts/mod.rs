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
    pub all_rs_files: Vec<String>,
}

pub(crate) fn collect(tree: &ProjectTree, route: &RsArchRoute) -> ArchFacts {
    // Build crate tree from route roots — NOT by walking the surface.
    let root_dirs: Vec<String> = route
        .roots()
        .iter()
        .map(|r| r.rel_dir().to_owned())
        .collect();
    let crate_tree = crate_tree::collect(tree, &root_dirs);
    let dependency_edges = dependency_edges::collect(tree, &crate_tree);
    let facade_surfaces = facade_surface::collect(tree, &crate_tree);
    let module_layouts = module_layout::collect(tree, &crate_tree);
    let all_rs_files = collect_all_rs_files(tree, &root_dirs);
    ArchFacts {
        crate_tree,
        dependency_edges,
        facade_surfaces,
        module_layouts,
        all_rs_files,
    }
}

/// Collect .rs files by walking within each route root's subtree.
/// Uses dir_contents (known-path lookup) not all_dir_rels (discovery).
fn collect_all_rs_files(tree: &ProjectTree, root_dirs: &[String]) -> Vec<String> {
    let mut files = Vec::new();
    for root in root_dirs {
        collect_rs_files_recursive(tree, root, &mut files);
    }
    files.sort();
    files.dedup();
    files
}

fn collect_rs_files_recursive(tree: &ProjectTree, dir: &str, files: &mut Vec<String>) {
    let Some(entry) = tree.dir_contents(dir) else {
        return;
    };
    for file in entry.files() {
        if file.ends_with(".rs") {
            files.push(ProjectTree::join_rel(dir, file));
        }
    }
    for subdir in entry.dirs() {
        let child = ProjectTree::join_rel(dir, subdir);
        collect_rs_files_recursive(tree, &child, files);
    }
}
