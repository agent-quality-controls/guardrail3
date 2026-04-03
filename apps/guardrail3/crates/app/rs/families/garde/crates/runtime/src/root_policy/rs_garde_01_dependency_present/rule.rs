use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::GardeRootInput;

const ID: &str = "RS-GARDE-01";

pub fn check(input: &GardeRootInput<'_>, results: &mut Vec<CheckResult>) {
    if input.root.garde_dependency_present {
        results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                "garde dependency found".to_owned(),
                format!(
                    "garde is present in `{}` for this {}. Garde-specific boundary checks are active.",
                    input.root.cargo_rel_path,
                    input.root.kind.label()
                ),
                Some(input.root.cargo_rel_path.clone()),
                None,
                false,
            )
            .as_inventory(),
        );
    } else {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "garde dependency missing".to_owned(),
            format!(
                "Missing `garde` dependency in `{}`. Add `garde` to `[dependencies]` in this Cargo.toml.",
                input.root.cargo_rel_path
            ),
            Some(input.root.cargo_rel_path.clone()),
            None,
            false,
        ));
    }
}

