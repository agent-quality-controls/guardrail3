use std::collections::BTreeSet;

use g3rs_fmt_types::G3RsFmtConfigChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "RS-FMT-CONFIG-02";

pub(crate) fn check(input: &G3RsFmtConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    let g3rs_fmt_types::G3RsFmtRustfmtConfigState::Parsed(rustfmt) = &input.rustfmt_state else {
        return;
    };
    let expected = expected_keys();
    for key in &rustfmt.explicit_keys {
        if !expected.contains(key.as_str()) {
            results.push(
                G3CheckResult::new(
                    ID.to_owned(),
                    G3Severity::Info,
                    format!("rustfmt extra setting: {key}"),
                    format!(
                    "`{key}` in `{}` is not part of the standard rustfmt baseline. Verify it is intentional.",
                        input.rustfmt_rel_path
                    ),
                    Some(input.rustfmt_rel_path.clone()),
                    None,
                )
                .into_inventory(),
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
#[path = "rule_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rule_tests;
