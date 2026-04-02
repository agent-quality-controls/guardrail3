use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::RepoReleaseInput;

const ID: &str = "RS-PUB-12";

pub fn check(input: &RepoReleaseInput<'_>, results: &mut Vec<CheckResult>) {
    results.push(
        CheckResult::from_parts(
            ID.to_owned(),
            Severity::Info,
            "Crate inventory".to_owned(),
            format!(
                "Repo has {} publishable crate(s) and {} non-publishable crate(s).",
                input.repo.publishable_count, input.repo.non_publishable_count
            ),
            Some(input.repo.cargo_rel_path.clone()),
            None,
            false,
        )
        .as_inventory(),
    );
}

