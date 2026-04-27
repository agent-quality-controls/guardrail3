use g3ts_astro_content_types::G3TsAstroContentAppRootInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "g3ts-astro-content/live-config-exists";

pub(crate) fn check(root: &G3TsAstroContentAppRootInput, results: &mut Vec<G3CheckResult>) {
    if let Some(rel_path) = &root.live_config_rel_path {
        results.push(
            G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Info,
                "live config present".to_owned(),
                format!(
                    "Astro content app root `{}` declares `{rel_path}`.",
                    root.app_root_rel_path
                ),
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
        "Live Astro content app root is missing `src/live.config.*`".to_owned(),
        format!(
            "Astro content app root `{}` uses live collections but has no `src/live.config.*` file. Add `src/live.config.ts` and declare live content collections there.",
            root.app_root_rel_path
        ),
        Some(root.app_root_rel_path.clone()),
        None,
    ));
}
