use crate::domain::report::{CheckResult, Severity};

use super::inputs::RustfmtRootInput;

const ID: &str = "RS-FMT-04";
const NIGHTLY_KEYS: &[&str] = &[
    "group_imports",
    "imports_granularity",
    "format_code_in_doc_comments",
    "format_strings",
    "overflow_delimited_expr",
    "normalize_comments",
    "normalize_doc_attributes",
    "wrap_comments",
    "format_macro_matchers",
    "format_macro_bodies",
    "condense_wildcard_suffixes",
];

pub fn check(input: &RustfmtRootInput<'_>, results: &mut Vec<CheckResult>) {
    let Some(rel) = input.config_rel else {
        return;
    };
    let Some(parsed) = input.parsed else {
        return;
    };
    if input.toolchain_channel != Some("stable") {
        return;
    }

    let Some(table) = parsed.as_table() else {
        return;
    };

    for key in NIGHTLY_KEYS {
        if table.contains_key(*key) {
            results.push(CheckResult {
                id: ID.to_owned(),
                severity: Severity::Warn,
                title: format!("nightly-only rustfmt setting `{key}` on stable"),
                message: format!(
                    "`{key}` is nightly-only, but rust-toolchain.toml uses `stable`."
                ),
                file: Some(rel.to_owned()),
                line: None,
                inventory: false,
            });
        }
    }
}
