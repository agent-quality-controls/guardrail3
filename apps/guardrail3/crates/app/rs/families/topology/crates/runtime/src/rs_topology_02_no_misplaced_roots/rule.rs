use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::MisplacedRootInput;

const ID: &str = "RS-TOPOLOGY-02";

pub fn check(input: &MisplacedRootInput<'_>, results: &mut Vec<CheckResult>) {
    if !input.reporting_enabled {
        return;
    }

    results.push(CheckResult::from_parts(
        ID.to_owned(),
        Severity::Error,
        format!(
            "Rust root `{}` is misplaced outside topology zones",
            display_dir(&input.root.rel_dir)
        ),
        format!(
            "`{}` lives outside any `apps/*` or `packages/*` zone while Rust topology enforcement is active.",
            input.root.cargo_rel_path
        ),
        Some(input.root.cargo_rel_path.clone()),
        None,
        false,
    ));
}

pub fn check_success(
    reporting_enabled: bool,
    has_misplaced_roots: bool,
    results: &mut Vec<CheckResult>,
) {
    if !reporting_enabled {
        results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                "Misplaced-root reporting is inactive".to_owned(),
                if has_misplaced_roots {
                    "Discovered Rust roots outside governed zones exist, but `RS-TOPOLOGY-02` is inactive because both owner topology families are disabled."
                        .to_owned()
                } else {
                    "No misplaced-root errors can fire in this run because `RS-TOPOLOGY-02` is inactive while both owner topology families are disabled."
                        .to_owned()
                },
                None,
                None,
                false,
            )
            .as_inventory(),
        );
        return;
    }

    if has_misplaced_roots {
        return;
    }

    results.push(
        CheckResult::from_parts(
            ID.to_owned(),
            Severity::Info,
            "No misplaced Rust roots found".to_owned(),
            "All discovered live Rust roots stay within governed topology zones or declared auxiliary roots."
                    .to_owned(),
            None,
            None,
            false,
        )
        .as_inventory(),
    );
}

fn display_dir(rel_dir: &str) -> &str {
    if rel_dir.is_empty() { "." } else { rel_dir }
}

