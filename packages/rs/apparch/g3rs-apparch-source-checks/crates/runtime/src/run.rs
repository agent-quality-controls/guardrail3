use std::collections::BTreeMap;

use g3rs_apparch_source_checks_types::G3RsApparchSourceChecksInput;
use g3rs_apparch_types::G3RsApparchCrate;
use guardrail3_check_types::G3CheckResult;

pub fn check(input: &G3RsApparchSourceChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    let crates_by_path = input
        .crates
        .iter()
        .map(|krate| (krate.cargo_rel_path.clone(), krate))
        .collect::<BTreeMap<_, _>>();

    crate::rs_apparch_source_04_io_traits_in_types::check(input, &crates_by_path, &mut results);

    results
}

pub(crate) fn display_crate(krate: &G3RsApparchCrate) -> &str {
    if krate.crate_name.is_empty() {
        &krate.cargo_rel_path
    } else {
        &krate.crate_name
    }
}
