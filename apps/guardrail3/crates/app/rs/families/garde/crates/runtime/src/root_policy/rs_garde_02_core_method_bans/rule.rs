use guardrail3_domain_report::{CheckResult, Severity};

use crate::garde_support::{CORE_METHOD_BANS, extract_ban_paths, missing_bans};
use crate::inputs::GardeRootInput;

const ID: &str = "RS-GARDE-02";

pub fn check(input: &GardeRootInput<'_>, results: &mut Vec<CheckResult>) {
    let Some(parsed) = input.root.clippy_parsed.as_ref() else {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Warn,
            "cannot verify core garde method bans".to_owned(),
            input.root.clippy_parse_error.clone().unwrap_or_else(|| {
                "No covering clippy configuration found for garde method-ban validation.".to_owned()
            }),
            input.root.clippy_rel_path.clone(),
            None,
            false,
        ));
        return;
    };

    let found = extract_ban_paths(parsed, "disallowed-methods");
    let missing = missing_bans(&found, CORE_METHOD_BANS);
    if missing.is_empty() {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Info,
            "core garde method bans present".to_owned(),
            "All core serde/toml/yaml deserialization bans are present in the covering clippy configuration.".to_owned(),
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
            "missing core garde method bans".to_owned(),
            format!(
                "Missing core garde deserialization bans from `disallowed-methods`: {}.",
                missing.join(", ")
            ),
            input.root.clippy_rel_path.clone(),
            None,
            false,
        ));
    }
}

