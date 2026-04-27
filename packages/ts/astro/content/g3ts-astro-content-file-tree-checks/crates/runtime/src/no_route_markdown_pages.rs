use g3ts_astro_content_types::G3TsAstroRouteMarkdownPageInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "g3ts-astro-content/no-route-markdown-pages";

pub(crate) fn check(page: &G3TsAstroRouteMarkdownPageInput, results: &mut Vec<G3CheckResult>) {
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
