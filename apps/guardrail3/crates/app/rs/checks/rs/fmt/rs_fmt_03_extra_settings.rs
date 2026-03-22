use std::collections::BTreeSet;

use crate::domain::report::{CheckResult, Severity};

use super::inputs::RustfmtRootInput;

const ID: &str = "RS-FMT-03";

pub fn check(input: &RustfmtRootInput<'_>, results: &mut Vec<CheckResult>) {
    let Some(rel) = input.config_rel else {
        return;
    };
    let Some(parsed) = input.parsed else {
        return;
    };
    let Some(table) = parsed.as_table() else {
        return;
    };

    let expected = expected_keys();
    for key in table.keys() {
        if !expected.contains(key.as_str()) {
            results.push(
                CheckResult {
                    id: ID.to_owned(),
                    severity: Severity::Info,
                    title: format!("rustfmt extra setting: {key}"),
                    message: "Non-baseline rustfmt setting present".to_owned(),
                    file: Some(rel.to_owned()),
                    line: None,
                    inventory: false,
                }
                .as_inventory(),
            );
        }
    }
}

fn expected_keys() -> BTreeSet<&'static str> {
    [
        "edition",
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
