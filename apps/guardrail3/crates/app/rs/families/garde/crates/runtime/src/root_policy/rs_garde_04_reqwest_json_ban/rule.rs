use guardrail3_domain_report::{CheckResult, Severity};

use crate::garde_support::{REQWEST_JSON_BAN, extract_ban_paths};
use crate::inputs::GardeRootInput;

const ID: &str = "RS-GARDE-04";

pub fn check(input: &GardeRootInput<'_>, results: &mut Vec<CheckResult>) {
    let Some(parsed) = input.root.clippy_parsed.as_ref() else {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Warn,
            "cannot verify reqwest garde ban".to_owned(),
            input.root.clippy_parse_error.clone().unwrap_or_else(|| {
                "No clippy.toml found. Create one with a `disallowed-methods` section."
                    .to_owned()
            }),
            input.root.clippy_rel_path.clone(),
            None,
            false,
        ));
        return;
    };

    let found = extract_ban_paths(parsed, "disallowed-methods");
    if found.contains(REQWEST_JSON_BAN) {
        results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                "reqwest garde ban present".to_owned(),
                "`reqwest::Response::json` is banned in the covering clippy configuration."
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
            "missing reqwest garde ban".to_owned(),
            "Missing `reqwest::Response::json` from `disallowed-methods`. Add it to `disallowed-methods` in clippy.toml.".to_owned(),
            input.root.clippy_rel_path.clone(),
            None,
            false,
        ));
    }
}

