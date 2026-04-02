use std::collections::BTreeSet;

use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::RustfmtRootInput;

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

