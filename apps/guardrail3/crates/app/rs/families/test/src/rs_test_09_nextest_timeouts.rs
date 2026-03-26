use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::RootTestInput;

const ID: &str = "RS-TEST-09";

pub fn check(input: &RootTestInput<'_>, results: &mut Vec<CheckResult>) {
    let async_active = input.has_tests && (input.root.tokio_dependency_present || input.has_tokio_tests);
    if !async_active {
        return;
    }

    if !input.root.nextest_exists {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: "nextest config missing".to_owned(),
            message: format!(
                "{} requires `{}` with timeout settings for async tests.",
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
    let default_profile = parsed.get("profile").and_then(|profile| profile.get("default"));
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
            message: format!(
                "`{}` must define both `[profile.default].slow-timeout` and `[profile.default].leak-timeout`.",
                input.root.nextest_rel_path
            ),
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
#[path = "rs_test_09_nextest_timeouts_tests/mod.rs"]
mod tests;
