use guardrail3_domain_report::{CheckResult, Severity};

use crate::garde_support::{EXTRACTOR_TYPE_BANS, extract_ban_paths, missing_bans};
use crate::inputs::GardeRootInput;

const ID: &str = "RS-GARDE-03";

pub fn check(input: &GardeRootInput<'_>, results: &mut Vec<CheckResult>) {
    let Some(parsed) = input.root.clippy_parsed.as_ref() else {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Warn,
            "cannot verify garde extractor bans".to_owned(),
            input.root.clippy_parse_error.clone().unwrap_or_else(|| {
                "No clippy.toml found. Create one with a `disallowed-types` section."
                    .to_owned()
            }),
            input.root.clippy_rel_path.clone(),
            None,
            false,
        ));
        return;
    };

    let found = extract_ban_paths(parsed, "disallowed-types");
    let missing = missing_bans(&found, EXTRACTOR_TYPE_BANS);
    if missing.is_empty() {
        results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                "garde extractor bans present".to_owned(),
                "All required Axum extractor bans are present in the covering clippy configuration."
                    .to_owned(),
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
            "missing garde extractor bans".to_owned(),
            format!(
                "Missing extractor type bans from `disallowed-types`: {}. Add these entries to `disallowed-types` in clippy.toml.",
                missing.join(", ")
            ),
            input.root.clippy_rel_path.clone(),
            None,
            false,
        ));
    }
}

