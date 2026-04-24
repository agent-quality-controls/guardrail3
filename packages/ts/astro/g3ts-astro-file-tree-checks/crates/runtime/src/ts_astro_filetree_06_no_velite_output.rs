use g3ts_astro_types::G3TsAstroFileTreeChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "TS-ASTRO-FILETREE-06";

pub(crate) fn check(input: &G3TsAstroFileTreeChecksInput, results: &mut Vec<G3CheckResult>) {
    for app_root in &input.app_roots {
        for rel_path in &app_root.velite_output_rel_paths {
            results.push(G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Error,
                "Astro app root must not contain `.velite/**` output".to_owned(),
                format!(
                    "Astro app root `{}` contains generated Velite artifact `{rel_path}`. Remove `.velite/**` from this Astro app and load content through Astro collections instead. Generated Velite output keeps a second content pipeline alive inside an Astro app.",
                    app_root.app_root_rel_path
                ),
                Some(rel_path.clone()),
                None,
            ));
        }
    }
}
