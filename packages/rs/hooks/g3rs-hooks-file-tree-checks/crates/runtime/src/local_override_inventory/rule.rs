use guardrail3_check_types::{G3CheckResult, G3Severity};

/// `ID` constant.
const ID: &str = "g3rs-hooks/local-override-inventory";

/// `check` function.
pub(crate) fn check(local_override_scripts: &[String], results: &mut Vec<G3CheckResult>) {
    if local_override_scripts.is_empty() {
        results.push(
            G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Info,
                "no local hook overrides".to_owned(),
                "No cached override scripts found in `.guardrail3/overrides/pre-commit.d`."
                    .to_owned(),
                Some(".guardrail3/overrides/pre-commit.d".to_owned()),
                None,
            )
            .into_inventory(),
        );
        return;
    }

    results.push(
        G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Info,
            "local hook overrides inventory".to_owned(),
            local_override_scripts.join(", "),
            Some(".guardrail3/overrides/pre-commit.d".to_owned()),
            None,
        )
        .into_inventory(),
    );
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rule_tests;
