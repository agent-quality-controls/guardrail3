use crate::facts::ArchFacts;
use guardrail3_app_rs_family_mapper::RsArchRoute;
use guardrail3_app_rs_family_view::FamilyView;
use guardrail3_domain_report::CheckResult;

pub fn check(surface: &FamilyView, route: &RsArchRoute) -> Vec<CheckResult> {
    let facts = crate::facts::collect(surface, route);
    run_with_facts(surface, &facts)
}

pub(crate) fn run_with_facts(surface: &FamilyView, facts: &ArchFacts) -> Vec<CheckResult> {
    let mut results = Vec::new();

    // Facade rules: per crate node.
    for node in facts.crate_tree.nodes.values() {
        crate::facade::rs_arch_01_crate_has_facade::check(node, &mut results);

        let lib_surface = node
            .lib_rs_rel
            .as_ref()
            .and_then(|rel| facts.facade_surfaces.get(rel));
        crate::facade::rs_arch_02_lib_facade_only::check(node, lib_surface, &mut results);

        // Complexity rules: per crate node.
        crate::complexity::rs_arch_07_force_crate_split::check(node, &mut results);
        crate::complexity::rs_arch_08_feature_gated_exports::check(node, lib_surface, &mut results);
    }

    // Facade rules: per module directory.
    for module in facts.module_layouts.values() {
        crate::facade::rs_arch_03_mod_rs_required::check(module, &mut results);
    }

    // Facade rules: per mod.rs surface.
    for fs in facts.facade_surfaces.values() {
        if fs.is_mod_rs {
            crate::facade::rs_arch_04_mod_facade_only::check(fs, &mut results);
        }
    }

    // Facade rules: #[path] scan on all .rs files.
    for rel_path in &facts.all_rs_files {
        crate::facade::rs_arch_09_no_path_attr::check_file(surface, rel_path, &mut results);
    }

    // Dependency rules: per dependency edge.
    for edge in &facts.dependency_edges.edges {
        crate::dependency::rs_arch_05_no_boundary_crossing::check(
            edge,
            &facts.crate_tree,
            &mut results,
        );
        crate::dependency::rs_arch_06_shared_flag_required::check(
            edge,
            &facts.crate_tree,
            &mut results,
        );
    }

    results
}
