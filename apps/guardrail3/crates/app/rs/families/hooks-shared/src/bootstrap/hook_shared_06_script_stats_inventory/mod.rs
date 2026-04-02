use guardrail3_domain_report::{CheckResult, Severity};

const ID: &str = "HOOK-SHARED-06";

pub fn check(rel_path: &str, content: &str, results: &mut Vec<CheckResult>) {
    let line_count = content.lines().count();
    let byte_count = content.len();
    results.push(
        CheckResult::from_parts(
            ID.to_owned(),
            Severity::Info,
            "pre-commit script stats".to_owned(),
            format!("{line_count} lines, {byte_count} bytes"),
            Some(rel_path.to_owned()),
            None,
            false,
        )
        .as_inventory(),
    );
}

#[cfg(test)]

mod hook_shared_06_script_stats_inventory_tests;
