use g3ts_astro_types::G3TsAstroFileTreeChecksInput;
use guardrail3_check_types::G3CheckResult;

#[must_use]
pub fn check(input: &G3TsAstroFileTreeChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    crate::ts_astro_filetree_02_content_config_exists::check(input, &mut results);
    crate::ts_astro_filetree_04_no_route_markdown_pages::check(input, &mut results);
    crate::ts_astro_filetree_05_no_velite_config::check(input, &mut results);
    crate::ts_astro_filetree_06_no_velite_output::check(input, &mut results);
    results
}
