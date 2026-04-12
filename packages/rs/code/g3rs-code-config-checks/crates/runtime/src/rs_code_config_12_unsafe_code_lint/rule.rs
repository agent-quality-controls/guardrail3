use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "RS-CODE-CONFIG-12";

pub(crate) fn check(
    cargo_rel_path: &str,
    lint_level: Option<&str>,
    results: &mut Vec<G3CheckResult>,
) {
    match lint_level {
        Some("forbid") => results.push(
            G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Info,
                "unsafe_code = forbid".to_owned(),
                "unsafe_code is set to forbid in workspace lints.".to_owned(),
                Some(cargo_rel_path.to_owned()),
                None,
            )
            .into_inventory(),
        ),
        Some("deny") => results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "unsafe_code should be forbid".to_owned(),
            "unsafe_code = deny can be overridden; use forbid in workspace lints.".to_owned(),
            Some(cargo_rel_path.to_owned()),
            None,
        )),
        _ => {}
    }
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod rule_tests;
