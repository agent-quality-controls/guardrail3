use std::collections::BTreeMap;

use g3rs_apparch_config_checks_types::G3RsApparchConfigChecksInput;
use g3rs_apparch_types::G3RsApparchCrate;
use guardrail3_check_types::G3CheckResult;

pub fn check(input: &G3RsApparchConfigChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    let crates_by_path = input
        .crates
        .iter()
        .map(|krate| (krate.cargo_rel_path.clone(), krate))
        .collect::<BTreeMap<_, _>>();

    for krate in &input.crates {
        crate::rs_apparch_config_01_types_dependency_direction::check(
            krate,
            &crates_by_path,
            &input.dependency_edges,
            &mut results,
        );
        crate::rs_apparch_config_02_logic_dependency_direction::check(
            krate,
            &crates_by_path,
            &input.dependency_edges,
            &mut results,
        );
        crate::rs_apparch_config_03_io_outbound_dependency_direction::check(
            krate,
            &crates_by_path,
            &input.dependency_edges,
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
mod tests;
