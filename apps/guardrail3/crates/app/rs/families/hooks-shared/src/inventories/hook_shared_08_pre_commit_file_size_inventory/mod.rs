use guardrail3_domain_report::{CheckResult, Severity};

const ID: &str = "HOOK-SHARED-08";

pub fn check(rel_path: &str, content: &str, results: &mut Vec<CheckResult>) {
    results.push(
        CheckResult::from_parts(
            ID.to_owned(),
            Severity::Info,
            "pre-commit file size".to_owned(),
            format!("{} bytes", content.len()),
            Some(rel_path.to_owned()),
            None,
            false,
        )
        .as_inventory(),
    );
}

#[cfg(test)]

mod tests;
