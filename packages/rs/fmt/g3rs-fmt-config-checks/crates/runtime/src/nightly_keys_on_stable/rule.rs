use g3rs_fmt_types::{G3RsFmtConfigChecksInput, G3RsFmtToolchainState};
use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::inputs::{rustfmt, rustfmt_table};

/// Rule identifier emitted by this check.
const ID: &str = "g3rs-fmt/nightly-keys-on-stable";
/// Constant value used by the surrounding module.
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

/// Runs the rule and appends any findings to `results`.
pub(crate) fn check(input: &G3RsFmtConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    let Some(rustfmt) = rustfmt(input) else {
        return;
    };
    let table = rustfmt_table(rustfmt);
    let nightly_keys = NIGHTLY_KEYS
        .iter()
        .copied()
        .filter(|key| table.contains_key(*key))
        .collect::<Vec<_>>();
    if nightly_keys.is_empty() {
        return;
    }

    match &input.toolchain_state {
        G3RsFmtToolchainState::Parsed(toolchain) => check_parsed_toolchain(
            input,
            toolchain
                .toolchain
                .as_ref()
                .and_then(|toolchain| toolchain.channel.as_deref()),
            &nightly_keys,
            results,
        ),
        G3RsFmtToolchainState::Missing => results.push(toolchain_blocker(
            input,
            "rust-toolchain.toml missing",
            "Nightly-only rustfmt settings require a root {} to verify the channel.",
        )),
        G3RsFmtToolchainState::Unreadable => results.push(toolchain_blocker(
            input,
            "rust-toolchain.toml unreadable",
            "Nightly-only rustfmt settings require a readable root {}.",
        )),
        G3RsFmtToolchainState::ParseError => results.push(toolchain_blocker(
            input,
            "rust-toolchain.toml parse error",
            "Nightly-only rustfmt settings require a parseable root {}.",
        )),
    }
}

/// Implements per-key reporting once the toolchain is parsed.
fn check_parsed_toolchain(
    input: &G3RsFmtConfigChecksInput,
    channel: Option<&str>,
    nightly_keys: &[&'static str],
    results: &mut Vec<G3CheckResult>,
) {
    match channel {
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

/// Builds a blocker finding pointing at `rust-toolchain.toml`. `message_template`
/// must contain a single `{}` placeholder that interpolates the toolchain rel-path.
fn toolchain_blocker(
    input: &G3RsFmtConfigChecksInput,
    title: &str,
    message_template: &str,
) -> G3CheckResult {
    G3CheckResult::new(
        ID.to_owned(),
        G3Severity::Error,
        title.to_owned(),
        message_template.replace("{}", &input.toolchain_rel_path),
        Some(input.toolchain_rel_path.clone()),
        None,
    )
}
