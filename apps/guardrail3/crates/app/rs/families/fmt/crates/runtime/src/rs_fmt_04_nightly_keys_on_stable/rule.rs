use guardrail3_domain_report::{CheckResult, Severity};

use crate::facts::ToolchainChannelState;
use crate::inputs::RustfmtRootInput;

const ID: &str = "RS-FMT-CONFIG-03";
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

    let nightly_keys = NIGHTLY_KEYS
        .iter()
        .copied()
        .filter(|key| table.contains_key(*key))
        .collect::<Vec<_>>();
    if nightly_keys.is_empty() {
        return;
    }

    match &input.toolchain_channel {
        ToolchainChannelState::Present(channel) if channel == "stable" => {
            for key in nightly_keys {
                results.push(CheckResult::from_parts(
                    ID.to_owned(),
                    Severity::Warn,
                    format!("nightly-only rustfmt setting `{key}` on stable"),
                    format!(
                        "`{key}` is nightly-only, but rust-toolchain.toml uses `stable`. Either remove `{key}` from rustfmt.toml or switch the toolchain channel to nightly."
                    ),
                    Some(rel.to_owned()),
                    None,
                    false,
                ));
            }
        }
        ToolchainChannelState::Present(_) => {}
        ToolchainChannelState::MissingManifest => results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "rust-toolchain.toml missing".to_owned(),
            "Nightly-only rustfmt settings require a root rust-toolchain.toml to verify the channel."
                .to_owned(),
            Some("rust-toolchain.toml".to_owned()),
            None,
            false,
        )),
        ToolchainChannelState::ParseError => results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "rust-toolchain.toml parse error".to_owned(),
            "Nightly-only rustfmt settings require a parseable root rust-toolchain.toml."
                .to_owned(),
            Some("rust-toolchain.toml".to_owned()),
            None,
            false,
        )),
        ToolchainChannelState::MissingChannel => results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: "rust-toolchain channel missing".to_owned(),
            message:
                "Nightly-only rustfmt settings require `[toolchain].channel` in root rust-toolchain.toml."
                    .to_owned(),
            file: Some("rust-toolchain.toml".to_owned()),
            line: None,
            inventory: false,
        }),
    }
}

