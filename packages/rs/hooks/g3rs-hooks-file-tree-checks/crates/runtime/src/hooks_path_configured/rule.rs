use guardrail3_check_types::{G3CheckResult, G3Severity};

/// `ID` constant.
const ID: &str = "g3rs-hooks/hooks-path-configured";

/// `check` function.
pub(crate) fn check(hooks_path: Option<&str>, results: &mut Vec<G3CheckResult>) {
    match hooks_path {
        Some(".githooks") => results.push(
            G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Error,
                "core.hooksPath configured".to_owned(),
                "git config core.hooksPath points to `.githooks`.".to_owned(),
                None,
                None,
            )
            .into_inventory(),
        ),
        Some(actual) => results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "core.hooksPath has wrong value".to_owned(),
            format!("Expected `.githooks`, got `{actual}`."),
            None,
            None,
        )),
        None => results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "core.hooksPath not configured".to_owned(),
            "git config core.hooksPath does not resolve to `.githooks`.".to_owned(),
            None,
            None,
        )),
    }
}
