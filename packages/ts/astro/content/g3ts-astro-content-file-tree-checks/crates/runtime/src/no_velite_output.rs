use guardrail3_check_types::{G3CheckResult, G3Severity};

/// Rule identifier.
const ID: &str = "g3ts-astro-content/no-velite-output";

/// Run this rule and append findings to `results`.
pub(crate) fn check(
    app_root_rel_path: &str,
    velite_output_rel_path: &str,
    results: &mut Vec<G3CheckResult>,
) {
    results.push(G3CheckResult::new(
        ID.to_owned(),
        G3Severity::Error,
        "Astro app root must not contain `.velite/**` output".to_owned(),
        format!(
            "Astro app root `{app_root_rel_path}` contains generated Velite artifact `{velite_output_rel_path}`. Remove `.velite/**` from this Astro app and load content through Astro collections instead. Generated Velite output keeps a second content pipeline alive inside an Astro app."
        ),
        Some(velite_output_rel_path.to_owned()),
        None,
    ));
}
