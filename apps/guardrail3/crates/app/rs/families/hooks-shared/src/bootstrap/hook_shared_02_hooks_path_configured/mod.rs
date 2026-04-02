use guardrail3_domain_report::{CheckResult, Severity};

const ID: &str = "HOOK-SHARED-02";

pub fn check(hooks_path: Option<&str>, results: &mut Vec<CheckResult>) {
    match hooks_path {
        Some(".githooks") => results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Error,
                "core.hooksPath configured".to_owned(),
                "git config core.hooksPath points to `.githooks`.".to_owned(),
                None,
                None,
                false,
            )
            .as_inventory(),
        ),
        Some(actual) => results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "core.hooksPath has wrong value".to_owned(),
            format!("Expected `.githooks`, got `{actual}`."),
            None,
            None,
            false,
        )),
        None => results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: "core.hooksPath not configured".to_owned(),
            message: "git config core.hooksPath does not resolve to `.githooks`.".to_owned(),
            file: None,
            line: None,
            inventory: false,
        }),
    }
}

#[cfg(test)]

mod hook_shared_02_hooks_path_configured_tests;
