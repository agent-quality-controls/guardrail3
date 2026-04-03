use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::ToolchainRootInput;

const ID: &str = "RS-TOOLCHAIN-02";

#[derive(Clone, Copy)]
enum ChannelKind {
    Stable,
    PinnedStable,
    Nightly,
    Beta,
    Unsupported,
}

pub fn check(input: &ToolchainRootInput<'_>, results: &mut Vec<CheckResult>) {
    if input.legacy_toolchain_rel.is_some() {
        return;
    }

    let Some(rel) = input.toolchain_toml_rel else {
        return;
    };

    let Some(parsed) = input.parsed else {
        if let Some(parse_error) = input.parse_error {
            results.push(CheckResult::from_parts(
                ID.to_owned(),
                Severity::Error,
                "rust-toolchain.toml parse error".to_owned(),
                format!("Invalid TOML: {parse_error}"),
                Some(rel.to_owned()),
                None,
                false,
            ));
        }
        return;
    };

    let Some(toolchain) = toolchain_table(parsed, rel, results) else {
        return;
    };

    check_channel(toolchain, rel, results);
    check_components(toolchain, rel, results);
}

fn toolchain_table<'a>(
    parsed: &'a toml::Value,
    rel: &str,
    results: &mut Vec<CheckResult>,
) -> Option<&'a toml::value::Table> {
    match parsed.get("toolchain") {
        Some(toml::Value::Table(toolchain)) => Some(toolchain),
        Some(_) => {
            results.push(CheckResult::from_parts(
                ID.to_owned(),
                Severity::Error,
                "toolchain table is invalid".to_owned(),
                "`rust-toolchain.toml` must define `[toolchain]` as a table.".to_owned(),
                Some(rel.to_owned()),
                None,
                false,
            ));
            None
        }
        None => {
            results.push(CheckResult::from_parts(
                ID.to_owned(),
                Severity::Error,
                "toolchain table missing".to_owned(),
                "Add a `[toolchain]` table with `channel` and `components`.".to_owned(),
                Some(rel.to_owned()),
                None,
                false,
            ));
            None
        }
    }
}

fn check_channel(toolchain: &toml::value::Table, rel: &str, results: &mut Vec<CheckResult>) {
    match toolchain.get("channel") {
        Some(toml::Value::String(channel)) => match classify_channel(channel) {
            ChannelKind::Stable => results.push(
                CheckResult::from_parts(
                    ID.to_owned(),
                    Severity::Info,
                    "toolchain channel is stable".to_owned(),
                    "channel = \"stable\".".to_owned(),
                    Some(rel.to_owned()),
                    None,
                    false,
                )
                .as_inventory(),
            ),
            ChannelKind::PinnedStable => results.push(
                CheckResult {
                    id: ID.to_owned(),
                    severity: Severity::Info,
                    title: "toolchain channel is pinned".to_owned(),
                    message: format!("Pinned channel `{channel}` is acceptable."),
                    file: Some(rel.to_owned()),
                    line: None,
                    inventory: false,
                }
                .as_inventory(),
            ),
            ChannelKind::Nightly => results.push(CheckResult::from_parts(
                ID.to_owned(),
                Severity::Error,
                "toolchain channel is nightly".to_owned(),
                "Channel is set to nightly. Use `channel = \"stable\"` or a pinned stable version.".to_owned(),
                Some(rel.to_owned()),
                None,
                false,
            )),
            ChannelKind::Beta => results.push(CheckResult::from_parts(
                ID.to_owned(),
                Severity::Error,
                "toolchain channel is beta".to_owned(),
                "Channel is set to beta. Use `channel = \"stable\"` or a pinned stable version.".to_owned(),
                Some(rel.to_owned()),
                None,
                false,
            )),
            ChannelKind::Unsupported => results.push(CheckResult {
                id: ID.to_owned(),
                severity: Severity::Error,
                title: "toolchain channel is unsupported".to_owned(),
                message: "Channel value is not recognized. Use `channel = \"stable\"` or a pinned stable version.".to_owned(),
                file: Some(rel.to_owned()),
                line: None,
                inventory: false,
            }),
        },
        Some(_) => results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: "toolchain channel is invalid".to_owned(),
            message: "`[toolchain].channel` must be a string.".to_owned(),
            file: Some(rel.to_owned()),
            line: None,
            inventory: false,
        }),
        None => results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: "toolchain channel missing".to_owned(),
            message: "Add `channel = \"stable\"` under `[toolchain]`.".to_owned(),
            file: Some(rel.to_owned()),
            line: None,
            inventory: false,
        }),
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

fn check_components(toolchain: &toml::value::Table, rel: &str, results: &mut Vec<CheckResult>) {
    match toolchain.get("components") {
        Some(toml::Value::Array(components)) => {
            if !components
                .iter()
                .all(|component| component.as_str().is_some())
            {
                results.push(CheckResult::from_parts(
                    ID.to_owned(),
                    Severity::Error,
                    "toolchain components are invalid".to_owned(),
                    "`[toolchain].components` must be an array of strings.".to_owned(),
                    Some(rel.to_owned()),
                    None,
                    false,
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
                        CheckResult::from_parts(
                            ID.to_owned(),
                            Severity::Info,
                            format!("toolchain component `{expected}` present"),
                            format!("`{expected}` is listed in `components`."),
                            Some(rel.to_owned()),
                            None,
                            false,
                        )
                        .as_inventory(),
                    );
                } else {
                    results.push(CheckResult::from_parts(
                        ID.to_owned(),
                        Severity::Warn,
                        format!("toolchain component `{expected}` missing"),
                        format!("Add `{expected}` to `[toolchain].components`."),
                        Some(rel.to_owned()),
                        None,
                        false,
                    ));
                }
            }
        }
        Some(_) => results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: "toolchain components are invalid".to_owned(),
            message: "`[toolchain].components` must be an array of strings.".to_owned(),
            file: Some(rel.to_owned()),
            line: None,
            inventory: false,
        }),
        None => results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: "toolchain components missing".to_owned(),
            message: "Add `components = [\"clippy\", \"rustfmt\"]` under `[toolchain]`.".to_owned(),
            file: Some(rel.to_owned()),
            line: None,
            inventory: false,
        }),
    }
}




// reason: test-only sidecar module wiring
