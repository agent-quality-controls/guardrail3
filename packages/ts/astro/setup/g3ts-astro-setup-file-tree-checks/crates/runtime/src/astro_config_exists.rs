use g3ts_astro_setup_types::G3TsAstroSetupAppRootInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

/// Stable rule identifier surfaced in findings.
const ID: &str = "g3ts-astro-setup/astro-config-exists";

/// Validates the rule and pushes findings into `results`.
pub(crate) fn check(app_root: &G3TsAstroSetupAppRootInput, results: &mut Vec<G3CheckResult>) {
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
        return;
    }

    results.push(G3CheckResult::new(
        ID.to_owned(),
        G3Severity::Error,
        "Astro app root is missing `astro.config.*`".to_owned(),
        format!(
            "Astro app root `{}` has no `astro.config.*` file. Add `astro.config.ts`, `astro.config.mjs`, or another supported Astro config file at that app root. Astro apps need one framework config entrypoint so integrations and render mode are declared in one place.",
            app_root.app_root_rel_path
        ),
        None,
        None,
    ));
}
