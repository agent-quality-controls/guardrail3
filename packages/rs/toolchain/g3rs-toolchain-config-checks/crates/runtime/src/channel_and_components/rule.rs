use guardrail3_check_types::{G3CheckResult, G3Severity};
use rust_toolchain_toml_parser::types::RustToolchainToml;
use rust_toolchain_toml_parser::types::rust_toolchain_toml::ToolchainSection;

const ID: &str = "g3rs-toolchain/channel-and-components";

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

    match first {
        "nightly" => return classify_named_channel(ChannelKind::Nightly, &segments[1..]),
        "beta" => return classify_named_channel(ChannelKind::Beta, &segments[1..]),
        "stable" => return classify_named_channel(ChannelKind::Stable, &segments[1..]),
        _ => {}
    }
    if parse_pinned_stable(raw).is_some() {
        return ChannelKind::PinnedStable;
    }

    ChannelKind::Unsupported
}

fn classify_named_channel(kind: ChannelKind, suffix: &[&str]) -> ChannelKind {
    if suffix.is_empty() {
        return kind;
    }

    if matches!(kind, ChannelKind::Nightly | ChannelKind::Beta) && suffix_starts_with_date(suffix) {
        let target = &suffix[1..];
        return if target.is_empty() || is_target_suffix(target) {
            kind
        } else {
            ChannelKind::Unsupported
        };
    }

    if is_target_suffix(suffix) {
        return kind;
    }

    ChannelKind::Unsupported
}

fn suffix_starts_with_date(suffix: &[&str]) -> bool {
    let [year, month, day, ..] = suffix else {
        return false;
    };
    [*year, *month, *day]
        .into_iter()
        .zip([4usize, 2, 2])
        .all(|(part, len)| part.len() == len && part.chars().all(|ch| ch.is_ascii_digit()))
}

fn is_target_suffix(suffix: &[&str]) -> bool {
    suffix.len() >= 2
        && suffix.iter().all(|segment| {
            !segment.is_empty()
                && segment
                    .chars()
                    .all(|ch| ch.is_ascii_alphanumeric() || ch == '_')
        })
}

fn parse_pinned_stable(raw: &str) -> Option<(u64, u64, u64)> {
    let normalized = raw.trim().trim_start_matches('v');
    if normalized.split_once('-').is_some() {
        return None;
    }
    let version_part = normalized;
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
