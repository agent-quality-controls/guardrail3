use g3rs_apparch_types::G3RsApparchCrate;
use g3rs_apparch_types::G3RsApparchSourceChecksInput;
use guardrail3_check_types::G3CheckResult;

pub fn check(input: &G3RsApparchSourceChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();

    for io_traits_check in &input.io_traits_checks {
        crate::rs_apparch_source_04_io_traits_in_types::check(io_traits_check, &mut results);
    }
    for types_public_surface_check in &input.types_public_surface_checks {
        crate::rs_apparch_source_05_types_public_surface::check(
            types_public_surface_check,
            &mut results,
        );
    }

    results
}

pub(crate) fn display_crate(krate: &G3RsApparchCrate) -> &str {
    if krate.crate_name.is_empty() {
        &krate.cargo_rel_path
    } else {
        &krate.crate_name
    }
}

#[cfg(test)]
#[path = "run_tests/mod.rs"]
mod run_tests;
