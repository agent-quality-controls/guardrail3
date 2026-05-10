use g3ts_astro_content_types::G3TsAstroContentAppRootInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

/// Rule identifier.
const ID: &str = "g3ts-astro-content/content-config-exists";

/// Run this rule and append findings to `results`.
pub(crate) fn check(root: &G3TsAstroContentAppRootInput, results: &mut Vec<G3CheckResult>) {
    if let Some(rel_path) = &root.content_config_rel_path {
        results.push(
            G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Info,
                "content config exists".to_owned(),
                format!("Found build-collection config `{rel_path}`."),
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
        "Build-collections app root is missing `src/content.config.*`".to_owned(),
        format!(
            "Astro app root `{}` uses build collections but has no `src/content.config.*` file. Add `src/content.config.ts` at that app root and declare the build collections there. Collection-backed content must be declared in one shared Astro content config instead of route-local loaders.",
            root.app_root_rel_path
        ),
        None,
        None,
    ));
}
