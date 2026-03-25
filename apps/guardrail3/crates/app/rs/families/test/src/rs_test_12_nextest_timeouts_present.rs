use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::RootTestInput;

const ID: &str = "RS-TEST-12";

pub fn check(input: &RootTestInput<'_>, results: &mut Vec<CheckResult>) {
    if !input.root.tokio_present || input.root.nextest_parse_error.is_some() {
        return;
    }

    if !input.root.nextest_exists {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: "nextest config missing".to_owned(),
            message: format!(
                "{} uses tokio but `{}` is missing.",
                display_root(&input.root.rel_dir),
                input.root.nextest_rel_path
            ),
            file: Some(input.root.nextest_rel_path.clone()),
            line: None,
            inventory: false,
        });
        return;
    }

    let Some(parsed) = input.root.nextest_parsed.as_ref() else {
        return;
    };
    let default_profile = parsed
        .get("profile")
        .and_then(|profile| profile.get("default"));
    let has_slow_timeout = default_profile
        .and_then(|profile| profile.get("slow-timeout"))
        .is_some();
    let has_leak_timeout = default_profile
        .and_then(|profile| profile.get("leak-timeout"))
        .is_some();
    if has_slow_timeout && has_leak_timeout {
        results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: "nextest timeouts configured".to_owned(),
                message: format!(
                    "`{}` defines both `slow-timeout` and `leak-timeout`.",
                    input.root.nextest_rel_path
                ),
                file: Some(input.root.nextest_rel_path.clone()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    } else {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: "nextest timeouts incomplete".to_owned(),
            message: format!("`{}` must set both `[profile.default].slow-timeout` and `[profile.default].leak-timeout`.", input.root.nextest_rel_path),
            file: Some(input.root.nextest_rel_path.clone()),
            line: None,
            inventory: false,
        });
    }
}

fn display_root(rel_dir: &str) -> String {
    if rel_dir.is_empty() {
        "project root".to_owned()
    } else {
        format!("`{rel_dir}`")
    }
}

#[cfg(test)]
#[path = "rs_test_12_nextest_timeouts_present_tests.rs"]
mod tests;
