use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "TS-ASTRO-STATE-FILETREE-12";

pub(crate) fn check(
    app_root_rel_path: &str,
    forbidden_state_rel_path: &str,
    results: &mut Vec<G3CheckResult>,
) {
    results.push(G3CheckResult::new(
        ID.to_owned(),
        G3Severity::Error,
        "Astro app contains state forbidden by `[ts.astro.state].forbidden`".to_owned(),
        format!(
            "Astro content app `{app_root_rel_path}` contains `{forbidden_state_rel_path}`, which matches its configured `[ts.astro.state].forbidden` policy. Remove the generated or legacy state so agents cannot bypass the Astro content pipeline."
        ),
        Some(forbidden_state_rel_path.to_owned()),
        None,
    ));
}
