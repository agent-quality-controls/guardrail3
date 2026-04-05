use g3_fmt_content_checks_types::G3FmtContentChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

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

pub(crate) fn check(input: &G3FmtContentChecksInput, results: &mut Vec<G3CheckResult>) {
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
        .toolchain
        .as_ref()
        .and_then(|toolchain| toolchain.channel.as_deref())
    {
        Some("stable") => {
            for key in nightly_keys {
                results.push(G3CheckResult::new(
                    ID.to_owned(),
                    G3Severity::Warn,
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
        None => results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
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

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod tests;
