use g3rs_release_types::G3RsReleaseFileTreeReadme;
use guardrail3_check_types::{G3CheckResult, G3Severity};

/// Rule identifier.
const ID: &str = "g3rs-release/readme-exists";

/// Run this rule and append findings to `results`.
pub(crate) fn check(readme: &G3RsReleaseFileTreeReadme, results: &mut Vec<G3CheckResult>) {
    if !readme.publishable || readme.readme_declared_false {
        return;
    }

    if readme.readme_exists {
        results.push(
            G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Info,
                format!("{}: README present", readme.crate_name),
                format!("README exists at `{}`.", readme.readme_rel_path),
                Some(readme.readme_rel_path.clone()),
                None,
            )
            .into_inventory(),
        );
    } else {
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            format!("{}: README missing", readme.crate_name),
            format!(
                "Publishable crate `{}` is missing README content at `{}`. Create a README.md for this crate.",
                readme.crate_name, readme.readme_rel_path
            ),
            Some(readme.cargo_rel_path.clone()),
            None,
        ));
    }
}
