use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "TS-ASTRO-STATE-FILETREE-11";

pub(crate) fn check(
    app_root_rel_path: &str,
    legacy_state_rel_path: &str,
    results: &mut Vec<G3CheckResult>,
) {
    results.push(G3CheckResult::new(
        ID.to_owned(),
        G3Severity::Error,
        "Astro content app must not contain legacy parallel framework state".to_owned(),
        format!(
            "Astro content app `{app_root_rel_path}` contains `{legacy_state_rel_path}`. Remove `.next/**`, `.contentlayer/**`, and `contentlayer.config.*` from this Astro app. These files prove a parallel Next/Contentlayer pipeline is present or was left behind, so agents can bypass Astro content collections."
        ),
        Some(legacy_state_rel_path.to_owned()),
        None,
    ));
}
