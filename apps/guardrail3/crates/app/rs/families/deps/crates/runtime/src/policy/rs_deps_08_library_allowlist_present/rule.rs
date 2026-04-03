use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::AllowlistCoverageDepsInput;

const ID: &str = "RS-DEPS-08";

pub fn check(input: &AllowlistCoverageDepsInput<'_>, results: &mut Vec<CheckResult>) {
    if input.coverage.has_allowlist {
        results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                "dependency allowlist present".to_owned(),
                format!(
                    "Crate `{}` has an `allowed_deps` policy.",
                    input.coverage.crate_name
                ),
                Some(input.coverage.cargo_rel_path.clone()),
                None,
                false,
            )
            .as_inventory(),
        );
    } else {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Warn,
            "dependency allowlist missing".to_owned(),
            format!(
                "Crate `{}` has no `allowed_deps` policy. Add an `allowed_deps` list for this crate in guardrail3.toml.",
                input.coverage.crate_name
            ),
            Some(input.coverage.cargo_rel_path.clone()),
            None,
            false,
        ));
    }
}

