use std::collections::BTreeMap;

use g3rs_arch_source_checks_types::G3RsArchSourceChecksInput;
use g3rs_arch_types::G3RsArchFacadeSurface;
use guardrail3_check_types::G3CheckResult;

pub fn check(input: &G3RsArchSourceChecksInput) -> Vec<G3CheckResult> {
    let facade_map = input
        .facade_surfaces
        .iter()
        .map(|surface| (surface.rel_path.as_str(), surface))
        .collect::<BTreeMap<_, _>>();

    let mut results = Vec::new();

    for node in &input.crate_nodes {
        let lib_surface = node
            .lib_rs_rel
            .as_deref()
            .and_then(|rel_path| facade_map.get(rel_path).copied());
        crate::rs_arch_02_lib_facade_only::check(node, lib_surface, &mut results);
        crate::rs_arch_08_feature_gated_exports::check(node, lib_surface, &mut results);
    }

    for surface in &input.facade_surfaces {
        if surface.is_mod_rs {
            crate::rs_arch_04_mod_facade_only::check(surface, &mut results);
        }
    }

    for file in &input.source_files {
        crate::rs_arch_09_no_path_attr::check_file(file, &mut results);
    }

    results
}

pub(crate) fn broad_reexports(
    surface: &G3RsArchFacadeSurface,
) -> impl Iterator<Item = &g3rs_arch_types::G3RsArchFacadeItem> {
    surface.broad_reexports.iter()
}
