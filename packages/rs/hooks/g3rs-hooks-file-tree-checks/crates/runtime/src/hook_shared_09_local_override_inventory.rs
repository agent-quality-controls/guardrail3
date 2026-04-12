use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "RS-HOOKS-FILETREE-05";

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
#[path = "hook_shared_09_local_override_inventory_tests/mod.rs"]
mod tests;
