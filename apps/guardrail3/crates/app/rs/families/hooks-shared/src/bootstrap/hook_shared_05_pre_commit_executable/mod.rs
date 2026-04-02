use guardrail3_domain_report::{CheckResult, Severity};

const ID: &str = "HOOK-SHARED-05";

pub fn check(rel_path: &str, executable: Option<bool>, results: &mut Vec<CheckResult>) {
    match executable {
        Some(true) => results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Error,
                "pre-commit hook is executable".to_owned(),
                "Dispatcher hook has the executable bit set.".to_owned(),
                Some(rel_path.to_owned()),
                None,
                false,
            )
            .as_inventory(),
        ),
        Some(false) => results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "pre-commit hook is not executable".to_owned(),
            "Dispatcher hook exists but does not have the executable bit set.".to_owned(),
            Some(rel_path.to_owned()),
            None,
            false,
        )),
        None => results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "pre-commit hook permissions unavailable".to_owned(),
            "Could not determine whether the dispatcher hook is executable.".to_owned(),
            Some(rel_path.to_owned()),
            None,
            false,
        )),
    }
}

#[cfg(test)]

mod tests;
