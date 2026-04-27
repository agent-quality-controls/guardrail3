use g3ts_astro_types::G3TsAstroFileTreeChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "TS-ASTRO-CONTENT-FILETREE-04";

pub(crate) fn check(input: &G3TsAstroFileTreeChecksInput, results: &mut Vec<G3CheckResult>) {
    if input.build_collection_roots.is_empty() && input.live_collection_roots.is_empty() {
        return;
    }

    for page in &input.route_markdown_pages {
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "Route markdown page bypasses Astro collections".to_owned(),
            format!(
                "Route markdown page `{}` lives under the route tree in a collection-backed Astro app. Move that content into `src/content/**` and render it through the declared collection pipeline instead. Route-owned markdown bypasses shared schemas and content adapters.",
                page.rel_path
            ),
            Some(page.rel_path.clone()),
            None,
        ));
    }
}
