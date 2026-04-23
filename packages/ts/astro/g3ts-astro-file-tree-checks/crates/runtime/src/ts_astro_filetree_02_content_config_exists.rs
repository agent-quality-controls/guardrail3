use g3ts_astro_types::G3TsAstroFileTreeChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "TS-ASTRO-FILETREE-02";

pub(crate) fn check(input: &G3TsAstroFileTreeChecksInput, results: &mut Vec<G3CheckResult>) {
    for root in &input.build_collection_roots {
        if let Some(rel_path) = &root.content_config_rel_path {
            results.push(
                G3CheckResult::new(
                    ID.to_owned(),
                    G3Severity::Info,
                    "content config exists".to_owned(),
                    format!("Found build-collection config `{rel_path}`."),
                    Some(rel_path.clone()),
                    None,
                )
                .into_inventory(),
            );
            continue;
        }

        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "content config missing".to_owned(),
            format!(
                "Astro app root `{}` uses build collections but is missing `src/content.config.*`.",
                root.app_root_rel_path
            ),
            None,
            None,
        ));
    }
}
