use guardrail3_domain_report::{CheckResult, Severity};

const ID: &str = "HOOK-SHARED-09";

pub fn check(local_override_scripts: &[String], results: &mut Vec<CheckResult>) {
    if local_override_scripts.is_empty() {
        results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                "no local hook overrides".to_owned(),
                "No cached override scripts found in `.guardrail3/overrides/pre-commit.d`."
                    .to_owned(),
                Some(".guardrail3/overrides/pre-commit.d".to_owned()),
                None,
                false,
            )
            .as_inventory(),
        );
        return;
    }

    results.push(
        CheckResult::from_parts(
            ID.to_owned(),
            Severity::Info,
            "local hook overrides inventory".to_owned(),
            local_override_scripts.join(", "),
            Some(".guardrail3/overrides/pre-commit.d".to_owned()),
            None,
            false,
        )
        .as_inventory(),
    );
}

#[cfg(test)]

mod tests;
