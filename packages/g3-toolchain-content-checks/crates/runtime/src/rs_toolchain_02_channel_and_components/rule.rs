use g3_toolchain_content_checks_types::G3ToolchainContentChecksInput;
use guardrail3_check_types::{GrdzCheckResult, GrdzSeverity};

const ID: &str = "RS-TOOLCHAIN-02";

#[derive(Clone, Copy)]
enum ChannelKind {
    Stable,
    PinnedStable,
    Nightly,
    Beta,
    Unsupported,
}

pub(crate) fn check(input: &G3ToolchainContentChecksInput, results: &mut Vec<GrdzCheckResult>) {
    let rel = &input.toolchain_rel_path;

    let Some(toolchain) = toolchain_table(&input.toolchain_toml, rel, results) else {
        return;
    };

    check_channel(toolchain, rel, results);
    check_components(toolchain, rel, results);
}

fn toolchain_table<'a>(
    parsed: &'a toml::Value,
    rel: &str,
    results: &mut Vec<GrdzCheckResult>,
) -> Option<&'a toml::value::Table> {
    match parsed.get("toolchain") {
        Some(toml::Value::Table(toolchain)) => Some(toolchain),
        Some(_) => {
            results.push(GrdzCheckResult::new(
                ID.to_owned(),
                GrdzSeverity::Error,
                "toolchain table is invalid".to_owned(),
                "`rust-toolchain.toml` must define `[toolchain]` as a table.".to_owned(),
                Some(rel.to_owned()),
                None,
            ));
            None
        }
        None => {
            results.push(GrdzCheckResult::new(
                ID.to_owned(),
                GrdzSeverity::Error,
                "toolchain table missing".to_owned(),
                "Add a `[toolchain]` table with `channel` and `components`.".to_owned(),
                Some(rel.to_owned()),
                None,
            ));
            None
        }
    }
}

fn check_channel(
    toolchain: &toml::value::Table,
    rel: &str,
    results: &mut Vec<GrdzCheckResult>,
) {
    match toolchain.get("channel") {
        Some(toml::Value::String(channel)) => match classify_channel(channel) {
            ChannelKind::Stable => results.push(
                GrdzCheckResult::new(
                    ID.to_owned(),
                    GrdzSeverity::Info,
                    "toolchain channel is stable".to_owned(),
                    "channel = \"stable\".".to_owned(),
                    Some(rel.to_owned()),
                    None,
                )
                .into_inventory(),
            ),
            ChannelKind::PinnedStable => results.push(
                GrdzCheckResult::new(
                    ID.to_owned(),
                    GrdzSeverity::Info,
                    "toolchain channel is pinned".to_owned(),
                    format!("Pinned channel `{channel}` is acceptable."),
                    Some(rel.to_owned()),
                    None,
                )
                .into_inventory(),
            ),
            ChannelKind::Nightly => results.push(GrdzCheckResult::new(
                ID.to_owned(),
                GrdzSeverity::Error,
                "toolchain channel is nightly".to_owned(),
                "Channel is set to nightly. Use `channel = \"stable\"` or a pinned stable version."
                    .to_owned(),
                Some(rel.to_owned()),
                None,
            )),
            ChannelKind::Beta => results.push(GrdzCheckResult::new(
                ID.to_owned(),
                GrdzSeverity::Error,
                "toolchain channel is beta".to_owned(),
                "Channel is set to beta. Use `channel = \"stable\"` or a pinned stable version."
                    .to_owned(),
                Some(rel.to_owned()),
                None,
            )),
            ChannelKind::Unsupported => results.push(GrdzCheckResult::new(
                ID.to_owned(),
                GrdzSeverity::Error,
                "toolchain channel is unsupported".to_owned(),
                "Channel value is not recognized. Use `channel = \"stable\"` or a pinned stable version."
                    .to_owned(),
                Some(rel.to_owned()),
                None,
            )),
        },
        Some(_) => results.push(GrdzCheckResult::new(
            ID.to_owned(),
            GrdzSeverity::Error,
            "toolchain channel is invalid".to_owned(),
            "`[toolchain].channel` must be a string.".to_owned(),
            Some(rel.to_owned()),
            None,
        )),
        None => results.push(GrdzCheckResult::new(
            ID.to_owned(),
            GrdzSeverity::Warn,
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

fn check_components(
    toolchain: &toml::value::Table,
    rel: &str,
    results: &mut Vec<GrdzCheckResult>,
) {
    match toolchain.get("components") {
        Some(toml::Value::Array(components)) => {
            if !components
                .iter()
                .all(|component| component.as_str().is_some())
            {
                results.push(GrdzCheckResult::new(
                    ID.to_owned(),
                    GrdzSeverity::Error,
                    "toolchain components are invalid".to_owned(),
                    "`[toolchain].components` must be an array of strings.".to_owned(),
                    Some(rel.to_owned()),
                    None,
                ));
                return;
            }

            let names = components
                .iter()
                .filter_map(toml::Value::as_str)
                .collect::<Vec<_>>();
            for expected in ["clippy", "rustfmt"] {
                if names.contains(&expected) {
                    results.push(
                        GrdzCheckResult::new(
                            ID.to_owned(),
                            GrdzSeverity::Info,
                            format!("toolchain component `{expected}` present"),
                            format!("`{expected}` is listed in `components`."),
                            Some(rel.to_owned()),
                            None,
                        )
                        .into_inventory(),
                    );
                } else {
                    results.push(GrdzCheckResult::new(
                        ID.to_owned(),
                        GrdzSeverity::Warn,
                        format!("toolchain component `{expected}` missing"),
                        format!("Add `{expected}` to `[toolchain].components`."),
                        Some(rel.to_owned()),
                        None,
                    ));
                }
            }
        }
        Some(_) => results.push(GrdzCheckResult::new(
            ID.to_owned(),
            GrdzSeverity::Error,
            "toolchain components are invalid".to_owned(),
            "`[toolchain].components` must be an array of strings.".to_owned(),
            Some(rel.to_owned()),
            None,
        )),
        None => {
            for expected in ["clippy", "rustfmt"] {
                results.push(GrdzCheckResult::new(
                    ID.to_owned(),
                    GrdzSeverity::Warn,
                    format!("toolchain component `{expected}` missing"),
                    format!("Add `{expected}` to `[toolchain].components`."),
                    Some(rel.to_owned()),
                    None,
                ));
            }
        }
    }
}
