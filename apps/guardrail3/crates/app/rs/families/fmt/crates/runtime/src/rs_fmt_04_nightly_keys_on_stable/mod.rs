use guardrail3_domain_report::{CheckResult, Severity};

use super::facts::ToolchainChannelState;
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
                        "`{key}` is nightly-only, but rust-toolchain.toml uses `stable`."
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
            "Nightly-only rustfmt settings require a root rust-toolchain.toml to prove the channel."
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

#[cfg(test)]
pub(crate) enum TestToolchainState {
    Stable,
    Other,
}

#[cfg(test)]
pub(crate) fn run_check(state: TestToolchainState) -> Vec<CheckResult> {
    let toolchain_channel = match state {
        TestToolchainState::Stable => ToolchainChannelState::Present("stable".to_owned()),
        TestToolchainState::Other => ToolchainChannelState::Present("1.85.0".to_owned()),
    };
    let input = RustfmtRootInput {
        config_rel: Some("rustfmt.toml".to_owned()),
        parsed: Some(
            toml::from_str::<toml::Value>(
                r#"
edition = "2024"
max_width = 100
tab_spaces = 4
use_field_init_shorthand = true
use_try_shorthand = true
reorder_imports = true
reorder_modules = true
group_imports = "StdExternalCrate"
"#,
            )
            .expect("RS-FMT-04 in-memory rustfmt TOML fixture should parse"),
        ),
        escape_hatches: Vec::new(),
        cargo_edition: super::facts::CargoEditionState::Present("2024".to_owned()),
        toolchain_channel,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}

#[cfg(test)]
pub(crate) fn run_family(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_root(root)
}

#[cfg(test)]

mod rs_fmt_04_nightly_keys_on_stable_tests;
