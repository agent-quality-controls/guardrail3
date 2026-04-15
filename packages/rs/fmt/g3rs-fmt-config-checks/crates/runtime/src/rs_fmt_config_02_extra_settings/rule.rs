use std::collections::BTreeSet;

use g3rs_fmt_types::G3RsFmtConfigChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::inputs::rustfmt;

const ID: &str = "RS-FMT-CONFIG-02";

pub(crate) fn check(input: &G3RsFmtConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    let Some(_rustfmt) = rustfmt(input) else {
        return;
    };
    let expected = expected_keys();
    for key in &input.rustfmt_explicit_keys {
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
