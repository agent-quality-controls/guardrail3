use g3rs_apparch_types::G3RsApparchCrate;
use g3rs_apparch_types::G3RsApparchSourceChecksInput;
use guardrail3_check_types::G3CheckResult;

#[must_use]
pub fn check(input: &G3RsApparchSourceChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();

    for io_traits_check in &input.io_traits_checks {
        crate::io_traits_in_types::check(io_traits_check, &mut results);
    }
    for types_public_surface_check in &input.types_public_surface_checks {
        crate::types_public_surface::check(types_public_surface_check, &mut results);
    }

    results
}

/// Returns the human-readable identifier for `krate`, falling back to its manifest path when unnamed.
pub(crate) fn display_crate(krate: &G3RsApparchCrate) -> &str {
    if krate.crate_name.is_empty() {
        &krate.cargo_rel_path
    } else {
        &krate.crate_name
    }
}
