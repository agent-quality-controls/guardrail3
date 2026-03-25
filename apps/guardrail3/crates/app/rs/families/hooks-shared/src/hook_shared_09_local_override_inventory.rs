use guardrail3_domain_report::{CheckResult, Severity};

const ID: &str = "HOOK-SHARED-09";

pub fn check(local_override_scripts: &[String], results: &mut Vec<CheckResult>) {
    if local_override_scripts.is_empty() {
        results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: "no local hook overrides".to_owned(),
                message:
                    "No cached override scripts found in `.guardrail3/overrides/pre-commit.d`."
                        .to_owned(),
                file: Some(".guardrail3/overrides/pre-commit.d".to_owned()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
        return;
    }

    results.push(
        CheckResult {
            id: ID.to_owned(),
            severity: Severity::Info,
            title: "local hook overrides inventory".to_owned(),
            message: local_override_scripts.join(", "),
            file: Some(".guardrail3/overrides/pre-commit.d".to_owned()),
            line: None,
            inventory: false,
        }
        .as_inventory(),
    );
}

#[cfg(test)]
#[path = "hook_shared_09_local_override_inventory_tests.rs"]
mod tests;
