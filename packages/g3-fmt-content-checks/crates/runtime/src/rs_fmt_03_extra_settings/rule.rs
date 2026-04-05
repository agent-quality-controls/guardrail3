use std::collections::BTreeSet;

use g3_fmt_content_checks_types::G3FmtContentChecksInput;
use guardrail3_check_types::{GrdzCheckResult, GrdzSeverity};

use crate::inputs::rustfmt_table;

const ID: &str = "RS-FMT-03";

pub(crate) fn check(input: &G3FmtContentChecksInput, results: &mut Vec<GrdzCheckResult>) {
    let expected = expected_keys();
    let table = rustfmt_table(&input.rustfmt);
    for key in table.keys() {
        if key == "skip_macro_invocations" && input.rustfmt.skip_macro_invocations.is_empty() {
            continue;
        }
        if !expected.contains(key.as_str()) {
            results.push(
                GrdzCheckResult::new(
                    ID.to_owned(),
                    GrdzSeverity::Info,
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
