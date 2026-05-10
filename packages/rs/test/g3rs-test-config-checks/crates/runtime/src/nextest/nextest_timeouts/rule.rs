use g3rs_test_types::G3RsTestConfigChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

/// `ID` constant.
const ID: &str = "g3rs-test/nextest-timeouts";

/// `check` function.
pub(crate) fn check(input: &G3RsTestConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    let async_active = input.has_tests && (input.tokio_dependency_present || input.has_tokio_tests);
    if !async_active {
        results.push(
            G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Info,
                "nextest timeouts inactive".to_owned(),
                format!(
                    "{} has no async test surface that requires nextest timeouts.",
                    display_root(&input.root_rel_dir)
                ),
                Some(input.cargo_rel_path.clone()),
                None,
            )
            .into_inventory(),
        );
        return;
    }

    let Some(nextest) = input.nextest.as_ref() else {
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "nextest config missing".to_owned(),
            format!(
                "{} requires `{}` with timeout settings for async tests.",
                display_root(&input.root_rel_dir),
                input.nextest_rel_path
            ),
            Some(input.nextest_rel_path.clone()),
            None,
        ));
        return;
    };

    let default_profile = nextest.profile.get("default");
    let has_slow_timeout = default_profile
        .and_then(|profile| profile.slow_timeout.as_ref())
        .is_some();
    let has_leak_timeout = default_profile
        .and_then(|profile| profile.leak_timeout.as_ref())
        .is_some();
    if has_slow_timeout && has_leak_timeout {
        results.push(
            G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Info,
                "nextest timeouts configured".to_owned(),
                format!(
                    "`{}` defines both `slow-timeout` and `leak-timeout`.",
                    input.nextest_rel_path
                ),
                Some(input.nextest_rel_path.clone()),
                None,
            )
            .into_inventory(),
        );
    } else {
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "nextest timeouts incomplete".to_owned(),
            format!(
                "`{}` must define both `[profile.default].slow-timeout` and `[profile.default].leak-timeout`.",
                input.nextest_rel_path
            ),
            Some(input.nextest_rel_path.clone()),
            None,
        ));
    }
}

/// `display_root` function.
fn display_root(rel_dir: &str) -> String {
    if rel_dir.is_empty() {
        "project root".to_owned()
    } else {
        format!("`{rel_dir}`")
    }
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rule_tests;
