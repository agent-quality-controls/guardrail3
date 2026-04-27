use g3ts_astro_types::G3TsAstroFileTreeChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "TS-ASTRO-CONTENT-FILETREE-05";

pub(crate) fn check(input: &G3TsAstroFileTreeChecksInput, results: &mut Vec<G3CheckResult>) {
    for app_root in &input.app_roots {
        if let Some(rel_path) = &app_root.velite_config_rel_path {
            results.push(G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Error,
                "Astro app root must not contain `velite.config.*`".to_owned(),
                format!(
                    "Astro app root `{}` contains `{rel_path}`. Remove `velite.config.*` and move this app onto Astro collections only. Keeping Velite config in an Astro app recreates the parallel content pipeline this family is meant to forbid.",
                    app_root.app_root_rel_path
                ),
                Some(rel_path.clone()),
                None,
            ));
        }
    }
}
