use g3ts_astro_types::G3TsAstroFileTreeChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "TS-ASTRO-FILETREE-03";

pub(crate) fn check(input: &G3TsAstroFileTreeChecksInput, results: &mut Vec<G3CheckResult>) {
    for root in &input.live_collection_roots {
        if let Some(rel_path) = &root.live_config_rel_path {
            results.push(
                G3CheckResult::new(
                    ID.to_owned(),
                    G3Severity::Info,
                    "live config present".to_owned(),
                    format!(
                        "Astro live-collections app root `{}` declares `{rel_path}`.",
                        root.app_root_rel_path
                    ),
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
            "live config missing".to_owned(),
            format!(
                "Astro live-collections app root `{}` is missing `src/live.config.*`.",
                root.app_root_rel_path
            ),
            Some(root.app_root_rel_path.clone()),
            None,
        ));
    }
}
