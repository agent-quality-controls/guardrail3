use guardrail3_check_types::{G3CheckResult, G3Severity};
use rust_toolchain_toml_parser::types::RustToolchainToml;
use rust_toolchain_toml_parser::types::rust_toolchain_toml::ToolchainSection;

const ID: &str = "RS-TOOLCHAIN-CONFIG-01";

#[derive(Clone, Copy)]
enum ChannelKind {
    Stable,
    PinnedStable,
    Nightly,
    Beta,
    Unsupported,
}

pub(crate) fn check(
    toolchain_rel_path: &str,
    toolchain_toml: &RustToolchainToml,
    results: &mut Vec<G3CheckResult>,
) {
    let Some(toolchain) = toolchain_table(toolchain_toml, toolchain_rel_path, results) else {
        return;
    };

    check_channel(toolchain, toolchain_rel_path, results);
    check_components(toolchain, toolchain_rel_path, results);
}

fn toolchain_table<'a>(
    parsed: &'a RustToolchainToml,
    rel: &str,
    results: &mut Vec<G3CheckResult>,
) -> Option<&'a ToolchainSection> {
    match parsed.toolchain.as_ref() {
        Some(toolchain) => Some(toolchain),
        None => {
            results.push(G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Error,
                "toolchain table missing".to_owned(),
                "Add a `[toolchain]` table with `channel` and `components`.".to_owned(),
                Some(rel.to_owned()),
                None,
            ));
            None
        }
    }
}

fn check_channel(toolchain: &ToolchainSection, rel: &str, results: &mut Vec<G3CheckResult>) {
    match toolchain.channel.as_deref() {
        Some(channel) => match classify_channel(channel) {
            ChannelKind::Stable => results.push(
                G3CheckResult::new(
                    ID.to_owned(),
                    G3Severity::Info,
                    "toolchain channel is stable".to_owned(),
                    "channel = \"stable\".".to_owned(),
                    Some(rel.to_owned()),
                    None,
                )
                .into_inventory(),
            ),
            ChannelKind::PinnedStable => results.push(
                G3CheckResult::new(
                    ID.to_owned(),
                    G3Severity::Info,
                    "toolchain channel is pinned".to_owned(),
                    format!("Pinned channel `{channel}` is acceptable."),
                    Some(rel.to_owned()),
                    None,
                )
                .into_inventory(),
            ),
            ChannelKind::Nightly => results.push(G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Error,
                "toolchain channel is nightly".to_owned(),
                "Channel is set to nightly. Use `channel = \"stable\"` or a pinned stable version."
                    .to_owned(),
                Some(rel.to_owned()),
                None,
            )),
            ChannelKind::Beta => results.push(G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Error,
                "toolchain channel is beta".to_owned(),
                "Channel is set to beta. Use `channel = \"stable\"` or a pinned stable version."
                    .to_owned(),
                Some(rel.to_owned()),
                None,
            )),
            ChannelKind::Unsupported => results.push(G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Error,
                "toolchain channel is unsupported".to_owned(),
                "Channel value is not recognized. Use `channel = \"stable\"` or a pinned stable version."
                    .to_owned(),
                Some(rel.to_owned()),
                None,
            )),
        },
        None => results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Warn,
            "toolchain channel missing".to_owned(),
            "Add `channel = \"stable\"` under `[toolchain]`.".to_owned(),
            Some(rel.to_owned()),
            None,
        )),
    }
}

fn classify_channel(raw: &str) -> ChannelKind {
    let normalized = raw.trim().to_ascii_lowercase();
    let segments = normalized.split('-').collect::<Vec<_>>();
    let first = segments.first().copied().unwrap_or("");

    if first.starts_with("nightly") {
        return ChannelKind::Nightly;
    }
    if first.starts_with("beta") {
        return ChannelKind::Beta;
    }

    if segments
        .iter()
        .skip(1)
        .any(|segment| segment.starts_with("nightly"))
    {
        return ChannelKind::Nightly;
    }
    if segments
        .iter()
        .skip(1)
        .any(|segment| segment.starts_with("beta"))
    {
        return ChannelKind::Beta;
    }

    if first == "stable" {
        return ChannelKind::Stable;
    }
    if parse_pinned_stable(raw).is_some() {
        return ChannelKind::PinnedStable;
    }

    ChannelKind::Unsupported
}

fn parse_pinned_stable(raw: &str) -> Option<(u64, u64, u64)> {
    let normalized = raw.trim().trim_start_matches('v');
    let version_part = normalized
        .split_once('-')
        .map_or(normalized, |(version_part, _)| version_part);
    let mut parts = version_part.split('.');

    let major = parts.next()?.parse().ok()?;
    let minor = parts.next()?.parse().ok()?;
    let patch = parts.next().unwrap_or("0").parse().ok()?;

    if parts.next().is_some() {
        return None;
    }

    Some((major, minor, patch))
}

fn check_components(toolchain: &ToolchainSection, rel: &str, results: &mut Vec<G3CheckResult>) {
    let names = toolchain
        .components
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>();

    for expected in ["clippy", "rustfmt"] {
        if names.contains(&expected) {
            results.push(
                G3CheckResult::new(
                    ID.to_owned(),
                    G3Severity::Info,
                    format!("toolchain component `{expected}` present"),
                    format!("`{expected}` is listed in `components`."),
                    Some(rel.to_owned()),
                    None,
                )
                .into_inventory(),
            );
        } else {
            results.push(G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Warn,
                format!("toolchain component `{expected}` missing"),
                format!("Add `{expected}` to `[toolchain].components`."),
                Some(rel.to_owned()),
                None,
            ));
        }
    }
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rule_tests;
