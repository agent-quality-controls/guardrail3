use g3ts_astro_types::G3TsAstroFileTreeChecksInput;
use guardrail3_check_types::G3CheckResult;

pub fn check(input: &G3TsAstroFileTreeChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    results.extend(check_setup(input));
    results.extend(check_content(input));
    results.extend(check_state(input));
    results
}

pub fn check_setup(input: &G3TsAstroFileTreeChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    crate::ts_astro_filetree_01_astro_config_exists::check(input, &mut results);
    crate::ts_astro_filetree_03_live_config_exists::check(input, &mut results);
    remap_ids(
        &mut results,
        &[
            ("TS-ASTRO-FILETREE-01", "TS-ASTRO-SETUP-FILETREE-01"),
            ("TS-ASTRO-FILETREE-03", "TS-ASTRO-SETUP-FILETREE-03"),
        ],
    );
    results
}

pub fn check_content(input: &G3TsAstroFileTreeChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    crate::ts_astro_filetree_02_content_config_exists::check(input, &mut results);
    crate::ts_astro_filetree_04_no_route_markdown_pages::check(input, &mut results);
    crate::ts_astro_filetree_05_no_velite_config::check(input, &mut results);
    crate::ts_astro_filetree_06_no_velite_output::check(input, &mut results);
    remap_ids(
        &mut results,
        &[
            ("TS-ASTRO-FILETREE-02", "TS-ASTRO-CONTENT-FILETREE-02"),
            ("TS-ASTRO-FILETREE-04", "TS-ASTRO-CONTENT-FILETREE-04"),
            ("TS-ASTRO-FILETREE-05", "TS-ASTRO-CONTENT-FILETREE-05"),
            ("TS-ASTRO-FILETREE-06", "TS-ASTRO-CONTENT-FILETREE-06"),
        ],
    );
    results
}

pub fn check_state(input: &G3TsAstroFileTreeChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    crate::ts_astro_filetree_11_no_legacy_parallel_state::check(input, &mut results);
    crate::ts_astro_filetree_12_configured_forbidden_state::check(input, &mut results);
    remap_ids(
        &mut results,
        &[
            ("TS-ASTRO-FILETREE-11", "TS-ASTRO-STATE-FILETREE-11"),
            ("TS-ASTRO-FILETREE-12", "TS-ASTRO-STATE-FILETREE-12"),
        ],
    );
    results
}

fn remap_ids(results: &mut [G3CheckResult], ids: &[(&str, &str)]) {
    for result in results {
        for (from, to) in ids {
            if result.id() == *from {
                let replacement = G3CheckResult::new(
                    (*to).to_owned(),
                    result.severity(),
                    result.title().to_owned(),
                    result.message().to_owned(),
                    result.file().map(ToOwned::to_owned),
                    result.line(),
                );
                *result = if result.inventory() {
                    replacement.into_inventory()
                } else {
                    replacement
                };
                break;
            }
        }
    }
}

#[cfg(test)]
#[path = "run_tests/mod.rs"]
mod run_tests;
