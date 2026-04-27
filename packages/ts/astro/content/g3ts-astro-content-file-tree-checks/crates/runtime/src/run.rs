use g3ts_astro_content_types::G3TsAstroContentFileTreeChecksInput;
use guardrail3_check_types::G3CheckResult;

#[must_use]
pub fn check(input: &G3TsAstroContentFileTreeChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    for build_root in &input.build_collection_roots {
        crate::content_config_exists::check(build_root, &mut results);
    }
    for live_root in &input.live_collection_roots {
        crate::live_config_exists::check(live_root, &mut results);
    }
    if !input.build_collection_roots.is_empty() || !input.live_collection_roots.is_empty() {
        for page in &input.route_markdown_pages {
            crate::no_route_markdown_pages::check(page, &mut results);
        }
    }
    for app_root in &input.app_roots {
        crate::no_velite_config::check(app_root, &mut results);
    }
    for velite_output_path in &input.velite_output_paths {
        crate::no_velite_output::check(
            &velite_output_path.app_root_rel_path,
            &velite_output_path.rel_path,
            &mut results,
        );
    }
    results
}
