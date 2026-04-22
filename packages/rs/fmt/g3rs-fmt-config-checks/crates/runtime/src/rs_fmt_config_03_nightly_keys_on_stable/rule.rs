use g3rs_fmt_types::{G3RsFmtConfigChecksInput, G3RsFmtToolchainState};
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "RS-FMT-CONFIG-03";

pub(crate) fn check(input: &G3RsFmtConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    let g3rs_fmt_types::G3RsFmtRustfmtConfigState::Parsed(rustfmt) = &input.rustfmt_state else {
        return;
    };
    if rustfmt.nightly_keys.is_empty() {
        return;
    }

    match &input.toolchain_state {
        G3RsFmtToolchainState::Parsed(toolchain) => match toolchain.channel.as_deref() {
            Some("stable") => {
                for key in &rustfmt.nightly_keys {
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
        },
        G3RsFmtToolchainState::Missing => results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "rust-toolchain.toml missing".to_owned(),
            format!(
                "Nightly-only rustfmt settings require a root {} to verify the channel.",
                input.toolchain_rel_path
            ),
            Some(input.toolchain_rel_path.clone()),
            None,
        )),
        G3RsFmtToolchainState::Unreadable => results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "rust-toolchain.toml unreadable".to_owned(),
            format!(
                "Nightly-only rustfmt settings require a readable root {}.",
                input.toolchain_rel_path
            ),
            Some(input.toolchain_rel_path.clone()),
            None,
        )),
        G3RsFmtToolchainState::ParseError => results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "rust-toolchain.toml parse error".to_owned(),
            format!(
                "Nightly-only rustfmt settings require a parseable root {}.",
                input.toolchain_rel_path
            ),
            Some(input.toolchain_rel_path.clone()),
            None,
        )),
    }
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rule_tests;
