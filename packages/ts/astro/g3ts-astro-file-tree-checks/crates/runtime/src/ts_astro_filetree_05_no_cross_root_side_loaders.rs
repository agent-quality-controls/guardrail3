use g3ts_astro_types::G3TsAstroFileTreeChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "TS-ASTRO-FILETREE-05";

pub(crate) fn check(input: &G3TsAstroFileTreeChecksInput, results: &mut Vec<G3CheckResult>) {
    for loader in &input.cross_root_side_loaders {
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "cross-root side loader present".to_owned(),
            format!(
                "`{}` reaches outside the Astro app root to load `{}`.",
                loader.loader_rel_path, loader.target_rel_path
            ),
            Some(loader.loader_rel_path.clone()),
            None,
        ));
    }
}
