use g3ts_astro_types::G3TsAstroFileTreeChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "TS-ASTRO-FILETREE-01";

pub(crate) fn check(input: &G3TsAstroFileTreeChecksInput, results: &mut Vec<G3CheckResult>) {
    for app_root in &input.app_roots {
        if let Some(rel_path) = &app_root.astro_config_rel_path {
            results.push(
                G3CheckResult::new(
                    ID.to_owned(),
                    G3Severity::Info,
                    "astro config exists".to_owned(),
                    format!("Found Astro config `{rel_path}`."),
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
            "astro config missing".to_owned(),
            format!(
                "No `astro.config.*` file was found for Astro app root `{}`.",
                app_root.app_root_rel_path
            ),
            None,
            None,
        ));
    }
}
