use std::collections::BTreeSet;

use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::RustfmtRootInput;

const ID: &str = "RS-FMT-03";

pub fn check(input: &RustfmtRootInput, results: &mut Vec<CheckResult>) {
    let Some(rel) = input.config_rel.as_deref() else {
        return;
    };
    let Some(parsed) = input.parsed.as_ref() else {
        return;
    };
    let Some(table) = parsed.as_table() else {
        return;
    };

    let expected = expected_keys();
    for key in table.keys() {
        if !expected.contains(key.as_str()) {
            results.push(
                CheckResult::from_parts(
                    ID.to_owned(),
                    Severity::Info,
                    format!("rustfmt extra setting: {key}"),
                    "Non-baseline rustfmt setting present".to_owned(),
                    Some(rel.to_owned()),
                    None,
                    false,
                )
                .as_inventory(),
            );
        }
    }
}

fn expected_keys() -> BTreeSet<&'static str> {
    [
        "edition",
        "style_edition",
        "max_width",
        "tab_spaces",
        "use_field_init_shorthand",
        "use_try_shorthand",
        "reorder_imports",
        "reorder_modules",
        "ignore",
    ]
    .into_iter()
    .collect()
}

#[cfg(test)]
pub(crate) fn run_check(parsed: toml::Value) -> Vec<CheckResult> {
    let input = RustfmtRootInput {
        config_rel: Some("rustfmt.toml".to_owned()),
        parsed: Some(parsed),
        escape_hatches: Vec::new(),
        cargo_edition: super::facts::CargoEditionState::Present("2024".to_owned()),
        toolchain_channel: super::facts::ToolchainChannelState::Present("stable".to_owned()),
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}

#[cfg(test)]

mod rs_fmt_03_extra_settings_tests;
