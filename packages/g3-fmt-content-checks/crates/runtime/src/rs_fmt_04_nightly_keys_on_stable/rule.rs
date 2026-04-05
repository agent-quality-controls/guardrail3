use g3_fmt_content_checks_types::G3FmtContentChecksInput;
use guardrail3_check_types::{GrdzCheckResult, GrdzSeverity};

use crate::inputs::rustfmt_table;

const ID: &str = "RS-FMT-04";
pub(crate) const NIGHTLY_KEYS: &[&str] = &[
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

pub(crate) fn check(input: &G3FmtContentChecksInput, results: &mut Vec<GrdzCheckResult>) {
    let table = rustfmt_table(&input.rustfmt);
    let nightly_keys = NIGHTLY_KEYS
        .iter()
        .copied()
        .filter(|key| table.contains_key(*key))
        .collect::<Vec<_>>();
    if nightly_keys.is_empty() {
        return;
    }

    match input
        .toolchain
        .toolchain()
        .and_then(|toolchain| toolchain.channel())
    {
        Some("stable") => {
            for key in nightly_keys {
                results.push(GrdzCheckResult::new(
                    ID.to_owned(),
                    GrdzSeverity::Warn,
                    format!("nightly-only rustfmt setting `{key}` on stable"),
                    format!(
                        "`{key}` is nightly-only, but {} uses `stable`. Either remove `{key}` from rustfmt.toml or switch the toolchain channel to nightly.",
                        input.toolchain_rel_path
                    ),
                    Some(input.rustfmt_rel_path.clone()),
                    None,
                ));
            }
        }
        Some(_) => {}
        None => results.push(GrdzCheckResult::new(
            ID.to_owned(),
            GrdzSeverity::Error,
            "rust-toolchain channel missing".to_owned(),
            format!(
                "Nightly-only rustfmt settings require `[toolchain].channel` in {}.",
                input.toolchain_rel_path
            ),
            Some(input.toolchain_rel_path.clone()),
            None,
        )),
    }
}
