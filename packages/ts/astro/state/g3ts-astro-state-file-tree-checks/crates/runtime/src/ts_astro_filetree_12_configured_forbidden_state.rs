use g3ts_astro_types::{G3TsAstroAppRootInput, G3TsAstroStateFileTreeChecksInput};
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "TS-ASTRO-STATE-FILETREE-12";

pub(crate) fn check(input: &G3TsAstroStateFileTreeChecksInput, results: &mut Vec<G3CheckResult>) {
    for app_root in strict_content_roots(input) {
        for rel_path in &app_root.forbidden_state_rel_paths {
            results.push(G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Error,
                "Astro app contains state forbidden by `[ts.astro.state].forbidden`".to_owned(),
                format!(
                    "Astro content app `{}` contains `{rel_path}`, which matches its configured `[ts.astro.state].forbidden` policy. Remove the generated or legacy state so agents cannot bypass the Astro content pipeline.",
                    app_root.app_root_rel_path
                ),
                Some(rel_path.clone()),
                None,
            ));
        }
    }
}

fn strict_content_roots(
    input: &G3TsAstroStateFileTreeChecksInput,
) -> impl Iterator<Item = &G3TsAstroAppRootInput> {
    input
        .build_collection_roots
        .iter()
        .chain(input.live_collection_roots.iter())
}
