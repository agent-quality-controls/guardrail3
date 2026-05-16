use guardrail3_check_types::{G3CheckResult, G3Severity};
use rust_toolchain_toml_parser::types::RustToolchainToml;
use rust_toolchain_toml_parser::types::rust_toolchain_toml::ToolchainSection;

/// Stable rule identifier surfaced in findings.
const ID: &str = "g3rs-toolchain/channel-and-components";

/// Classification of a `[toolchain] channel = ...` value.
#[derive(Clone, Copy)]
enum ChannelKind {
    /// Channel is exactly `stable` (or stable plus a target triple).
    Stable,
    /// Channel is a pinned stable version like `1.85.0`.
    PinnedStable,
    /// Channel is `nightly` (optionally dated).
    Nightly,
    /// Channel is `beta` (optionally dated).
    Beta,
    /// Channel value is not recognized.
    Unsupported,
}

/// Validates the `[toolchain]` table's channel and components.
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

/// Returns the `[toolchain]` section, surfacing an error finding when missing.
fn toolchain_table<'a>(
    parsed: &'a RustToolchainToml,
    rel: &str,
    results: &mut Vec<G3CheckResult>,
) -> Option<&'a ToolchainSection> {
    parsed.toolchain.as_ref().map_or_else(
        || {
            results.push(G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Error,
                "toolchain table missing".to_owned(),
                "Add a `[toolchain]` table with `channel` and `components`.".to_owned(),
                Some(rel.to_owned()),
                None,
            ));
            None
        },
        Some,
    )
}

/// Inspects the channel value and surfaces inventory or violations.
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

/// Classifies a raw `channel` string into a [`ChannelKind`].
fn classify_channel(raw: &str) -> ChannelKind {
    let normalized = raw.trim().to_ascii_lowercase();
    let segments = normalized.split('-').collect::<Vec<_>>();
    let (first, rest) = segments
        .split_first()
        .map_or(("", &[][..]), |(head, tail)| (*head, tail));

    match first {
        "nightly" => return classify_named_channel(ChannelKind::Nightly, rest),
        "beta" => return classify_named_channel(ChannelKind::Beta, rest),
        "stable" => return classify_named_channel(ChannelKind::Stable, rest),
        _ => {}
    }
    if parse_pinned_stable(raw).is_some() {
        return ChannelKind::PinnedStable;
    }

    ChannelKind::Unsupported
}

/// Refines `kind` based on suffix segments after the channel name.
fn classify_named_channel(kind: ChannelKind, suffix: &[&str]) -> ChannelKind {
    if suffix.is_empty() {
        return kind;
    }

    if matches!(kind, ChannelKind::Nightly | ChannelKind::Beta) && suffix_starts_with_date(suffix) {
        let target = suffix.get(1..).unwrap_or(&[]);
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

/// Returns true when `suffix` starts with a `YYYY-MM-DD` date.
fn suffix_starts_with_date(suffix: &[&str]) -> bool {
    let [year, month, day, ..] = suffix else {
        return false;
    };
    [*year, *month, *day]
        .into_iter()
        .zip([4usize, 2, 2])
        .all(|(part, len)| part.len() == len && part.chars().all(|ch| ch.is_ascii_digit()))
}

/// Returns true when `suffix` is a valid LLVM target triple body.
fn is_target_suffix(suffix: &[&str]) -> bool {
    suffix.len() >= 2
        && suffix.iter().all(|segment| {
            !segment.is_empty()
                && segment
                    .chars()
                    .all(|ch| ch.is_ascii_alphanumeric() || ch == '_')
        })
}

/// Parsed semver tuple of a pinned stable channel.
type PinnedStableVersion = (u64, u64, u64);

/// Parses `raw` as a pinned stable version `MAJOR.MINOR[.PATCH]`.
fn parse_pinned_stable(raw: &str) -> Option<PinnedStableVersion> {
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

/// Inspects required toolchain components and surfaces inventory or warnings.
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
