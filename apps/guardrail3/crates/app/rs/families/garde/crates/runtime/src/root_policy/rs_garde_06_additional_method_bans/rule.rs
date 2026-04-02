use guardrail3_domain_report::{CheckResult, Severity};

use crate::garde_support::{ADDITIONAL_METHOD_BANS, extract_ban_paths, missing_bans};
use crate::inputs::GardeRootInput;

const ID: &str = "RS-GARDE-06";

pub fn check(input: &GardeRootInput<'_>, results: &mut Vec<CheckResult>) {
    let Some(parsed) = input.root.clippy_parsed.as_ref() else {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Warn,
            "cannot verify additional garde method bans".to_owned(),
            input.root.clippy_parse_error.clone().unwrap_or_else(|| {
                "No covering clippy configuration found for additional garde method-ban validation.".to_owned()
            }),
            input.root.clippy_rel_path.clone(),
            None,
            false,
        ));
        return;
    };

    let found = extract_ban_paths(parsed, "disallowed-methods");
    let missing = missing_bans(&found, ADDITIONAL_METHOD_BANS);
    if missing.is_empty() {
        results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                "additional garde method bans present".to_owned(),
                "All additional garde deserialization entry-point bans are present in the covering clippy configuration.".to_owned(),
                input.root.clippy_rel_path.clone(),
                None,
                false,
            )
            .as_inventory(),
        );
    } else {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Warn,
            "missing additional garde method bans".to_owned(),
            format!(
                "Missing additional garde deserialization bans from `disallowed-methods`: {}.",
                missing.join(", ")
            ),
            input.root.clippy_rel_path.clone(),
            None,
            false,
        ));
    }
}

