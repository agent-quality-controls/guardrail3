use guardrail3_domain_report::{CheckResult, Severity};

const ID: &str = "HOOK-SHARED-12";

pub fn check(modular_executable: &[(String, bool)], results: &mut Vec<CheckResult>) {
    if modular_executable.is_empty() {
        return;
    }

    for (rel_path, executable) in modular_executable {
        if *executable {
            results.push(
                CheckResult::from_parts(
                    ID.to_owned(),
                    Severity::Warn,
                    "modular hook script is executable".to_owned(),
                    "Modular hook script has the executable bit set.".to_owned(),
                    Some(rel_path.clone()),
                    None,
                    false,
                )
                .as_inventory(),
            );
        } else {
            results.push(CheckResult::from_parts(
                ID.to_owned(),
                Severity::Warn,
                "modular hook script is not executable".to_owned(),
                "Modular hook script exists but does not have the executable bit set.".to_owned(),
                Some(rel_path.clone()),
                None,
                false,
            ));
        }
    }
}

#[cfg(test)]
#[path = "hook_shared_12_modular_scripts_executable_tests/mod.rs"]
mod hook_shared_12_modular_scripts_executable_tests;
