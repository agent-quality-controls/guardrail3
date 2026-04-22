use g3rs_arch_types::G3RsArchSourceChecksInput;
use g3rs_arch_types::types::G3RsArchFacadeSurface;
use guardrail3_check_types::G3CheckResult;

pub fn check(input: &G3RsArchSourceChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();

    for check_input in &input.lib_facade_checks {
        crate::rs_arch_02_lib_facade_only::check(
            &check_input.krate,
            check_input.lib_surface.as_ref(),
            &mut results,
        );
        crate::rs_arch_08a_feature_gated_exports::check(
            &check_input.krate,
            check_input.lib_surface.as_ref(),
            &mut results,
        );
    }

    for surface in &input.mod_facade_surfaces {
        crate::rs_arch_04_mod_facade_only::check(surface, &mut results);
    }

    for site in &input.path_attr_sites {
        crate::rs_arch_09_no_path_attr::check(site, &mut results);
    }

    results
}

pub(crate) fn broad_reexports(
    surface: &G3RsArchFacadeSurface,
) -> impl Iterator<Item = &g3rs_arch_types::types::G3RsArchFacadeItem> {
    surface.broad_reexports.iter()
}

#[cfg(test)]
#[path = "run_tests/mod.rs"]
mod run_tests;
